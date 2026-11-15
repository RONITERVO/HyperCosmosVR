use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use cosmos_audio::{AudioEngine, AudioOutputMode};
use cosmos_journey::{
    BeatTargetSelector, ExportedJourneyPaths, JourneyDirector, JourneyPlayState, JourneyRecorder,
    JourneyRegistry, LocalizedTitle, ManualOverrideInput, PeripheralChoice,
    PeripheralPromptVisual,
};
use cosmos_manifest::{AudioAssetRecord, Locale, UniverseCatalog};
use cosmos_math::{smoothstep, SectorId};
use cosmos_narration::{
    encode_cue_tags_mask, srgb_to_linear_hdr, FocusCandidate, FsmConfig, FsmInput, FsmOutput,
    FsmStateKind, MemoryPlanState, NarrationPlan, NarrationSegment, ResolverConfig,
    RestNarrationFsm, SegmentRole, SharedNarrationMemory, TargetFocus, TargetIdentity,
    TargetResolver,
};
use cosmos_nav::{
    DesktopFlightInput, DiegeticAction, FlightCamera, FocusTarget, ScaleMeridianArc, SummonMode,
    VrFlightInput, WristResonanceRing, WristSummonInput, PLAYBACK_RATES,
};
use cosmos_render::{
    AudioReactiveUniforms as RenderAudioUniforms, DiegeticUiInput, DocumentaryFocusState,
    PostSettings, RenderCamera, RenderSceneFrame, SceneBuildInput, SpriteInstance,
    UniverseSceneBuilder,
};
use cosmos_spatial::{
    all_experience_placements, prism_local_to_hsl, CosmosSpatialIndex, EntityKind, VisibleEntity,
};
use cosmos_typo::{
    cue_reveal_cursor_for_asset, AmbientLabelInput, AnchorMode, AngularRect,
    BilingualNarrationSpec, BilingualPhase, CameraBasis, FontRole, GlyphInkMode, GlyphInstance,
    MsdfAtlas, TetherInstance, TypoFrameOutput, TypoLayoutEngine,
};
use glam::{DVec3, Vec3};

use crate::assets::{
    build_ambient_label_inputs, resolve_focus_for_visible, UniverseAssets,
};
use crate::guide::{GuideAction, GuideContext, GuideFrame, GuidePage, TourContext};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SegmentPhase {
    Spanish,
    Translation,
    Completed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SolarObservationMode {
    Interior,
    VisibleComposite,
    VisibleDetail,
    Euv,
}

impl SolarObservationMode {
    fn disclosure(self, locale: Locale) -> &'static str {
        match (self, locale) {
            (Self::Interior, Locale::FiFi) => "Sisäleikkaus · kaavamainen kuljetus",
            (Self::Interior, Locale::EsEs) => "Corte interior · transporte esquemático",
            (Self::Interior, _) => "Interior cutaway · schematic transport",
            (Self::VisibleComposite, Locale::FiFi) => "Näkyvä valo + havainnollistava plasma",
            (Self::VisibleComposite, Locale::EsEs) => "Luz visible + plasma ilustrativo",
            (Self::VisibleComposite, _) => "Visible light + illustrative plasma",
            (Self::VisibleDetail, Locale::FiFi) => "Näkyvä valo + havainnollistava plasma\nTehostettu kontrasti",
            (Self::VisibleDetail, Locale::EsEs) => "Luz visible + plasma ilustrativo\nContraste realzado",
            (Self::VisibleDetail, _) => "Visible light + illustrative plasma\nContrast enhanced",
            (Self::Euv, Locale::FiFi) => "EUV-kehä · vääräväri",
            (Self::Euv, Locale::EsEs) => "Atmósfera EUV · falso color",
            (Self::Euv, _) => "EUV atmosphere · false color",
        }
    }
}

#[derive(Clone, Debug)]
struct ActiveNarrationSession {
    target: TargetIdentity,
    anchor_pos_cam: Vec3,
    surface_dist_m: Option<f32>,
    plan: NarrationPlan,
    memory_state: MemoryPlanState,
    seg_idx: usize,
    phase: SegmentPhase,
    playlist: Vec<u32>,
    playlist_pos: usize,
    active_asset_idx: Option<u32>,
    last_es_text: String,
    active_music_note_id: Option<u64>,
}

/// CLI / launcher options controlling Story-Journey selection and autoplay at startup.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct LaunchJourneyConfig {
    pub journey_id_or_path: Option<String>,
    pub autoplay: bool,
}

impl LaunchJourneyConfig {
    pub fn apply(&self, runtime: &mut UniverseRuntime) -> Result<()> {
        if let Some(ref target) = self.journey_id_or_path {
            runtime.activate_journey(target, self.autoplay)?;
        } else if self.autoplay {
            let def = runtime.journey_registry.default_id().to_owned();
            runtime.activate_journey(&def, true)?;
        }
        Ok(())
    }
}

/// Optional deterministic override used by `--headless-render` to freeze mid-narration
/// SyncVoice live-ink cursor & diegetic wrist ring for crisp automated visual inspection.
#[derive(Clone, Debug, Default)]
pub struct VerificationNarrationOverride {
    pub active_experience_id: u8,
    pub phase_reveal: f32,
    pub show_spanish_ghost_translation: bool,
    pub summon_wrist_ring: bool,
    pub force_playback_rate: Option<f32>,
    pub force_scale_vel: Option<f64>,
    pub force_zone_index: Option<u16>,
    pub force_feature_index: Option<u16>,
    pub force_earth_cutaway: Option<f32>,
}

/// Unified frame payload combining scene draw commands and HDR 3D MSDF typography output.
pub struct FramePacket {
    pub scene: RenderSceneFrame,
    pub typo: TypoFrameOutput,
    pub active_experience_id: u8,
    pub scale_exp: f64,
    pub hovered_entity_id: Option<u64>,
    pub fsm_state: FsmStateKind,
    pub haptic_pulse: Option<(f32, i64, f32)>,
    pub guide: GuideFrame,
}

/// Complete real-time simulation orchestrator shared by OpenXR VR, Desktop HDR, and Headless modes.
pub struct UniverseRuntime {
    pub catalog: Arc<UniverseCatalog>,
    pub spatial: CosmosSpatialIndex,
    pub atlas: MsdfAtlas,
    pub resolver: TargetResolver,
    pub memory: SharedNarrationMemory,
    pub fsm: RestNarrationFsm,
    pub flight: FlightCamera,
    pub wrist_ring: WristResonanceRing,
    pub scale_meridian: ScaleMeridianArc,
    pub audio: AudioEngine,
    pub typo_layout: TypoLayoutEngine,
    pub scene_builder: UniverseSceneBuilder,
    pub journey_registry: JourneyRegistry,
    pub journey_director: Option<JourneyDirector>,
    pub journey_recorder: JourneyRecorder,
    pub user_journeys_dir: PathBuf,
    pub last_recorded_export: Option<ExportedJourneyPaths>,
    pub locale: Locale,
    pub time_sec: f32,
    active_session: Option<ActiveNarrationSession>,
    last_zone_sound_key: Option<(u8, u16)>,
    prev_fsm_kind: FsmStateKind,
    prev_hovered_bead: Option<u32>,
    active_peripheral_prompt: Option<PeripheralPromptVisual>,
    pub guide_page: GuidePage,
    guide_enabled: bool,
    guide_is_vr: bool,
    guide_hovered: Option<GuideAction>,
    guide_pointer_captured: bool,
    guide_pointer_ray: Option<(Vec3, Vec3)>,
    guide_haptic: Option<(f32, i64, f32)>,
    desktop_control_gesture: bool,
    vr_control_gesture: bool,
    narration_paused: bool,
    vr_tracking_head: Vec3,
    vr_tracking_origin: Option<Vec3>,
    vr_tracking_orientation: glam::Quat,
    vr_rig_orientation: glam::Quat,
    vr_hologram_anchor: Option<(Vec3, glam::Quat)>,
    solar_observation_anchor: Option<(SolarObservationMode, Vec3, glam::Quat)>,
    vr_ring_visible: bool,
}

fn signature_zone_slug(experience_id: u8) -> Option<&'static str> {
    match experience_id {
        5 => Some("galactic-center"),
        6 => Some("local-group-center"),
        7 => Some("laniakea-basin"),
        8 => Some("galactic-filaments"),
        9 => Some("cmb"),
        10 => Some("inflation"),
        11 => Some("observed"),
        12 => Some("black-holes"),
        13 => Some("atoms"),
        14 => Some("planck"),
        _ => None,
    }
}

fn resolve_signature_zone_index(catalog: &UniverseCatalog, experience_id: u8) -> u16 {
    let Some(slug) = signature_zone_slug(experience_id) else {
        return 0;
    };
    catalog
        .zones(experience_id)
        .iter()
        .position(|z| catalog.str_slice(z.id) == slug)
        .map(|i| i as u16)
        .unwrap_or(0)
}

#[inline]
fn scale_meridian_basis_quat(camera: &RenderCamera) -> glam::Quat {
    // Rotate 11.5 deg right around camera-up so meridian ticks sit around +30 deg azimuth,
    // clearing Earth's central hero blocker while staying inside the 52 deg half-HFOV.
    let rot_right = glam::Quat::from_axis_angle(
        camera.cam_up.normalize_or(Vec3::Y),
        -11.5_f32.to_radians(),
    );
    rot_right * camera_basis_quat(camera)
}

impl UniverseRuntime {
    /// Initializes runtime directly overlooking Earth Low Orbit (`u = +6.6`, `03: Termosfera/Aurora`).
    pub fn new(assets: UniverseAssets, audio_mode: AudioOutputMode) -> Result<Self> {
        let mut audio = AudioEngine::new(
            assets.hrir_path.exists().then_some(assets.hrir_path.as_path()),
            audio_mode,
        )?;
        audio.set_playback_rate(1.0);

        let wrist_ring = WristResonanceRing {
            selected_locale: Locale::EnUs,
            ..Default::default()
        };

        let mut journey_registry = JourneyRegistry::with_builtins();
        let assets_journeys_dir = assets.root.join("assets").join("journeys");
        let user_journeys_dir = assets.root.join("user_journeys");
        let _ = journey_registry.discover_in_directory(&assets_journeys_dir);
        let _ = journey_registry.discover_in_directory(&user_journeys_dir);

        let default_journey_id = journey_registry.default_id().to_owned();
        let initial_pose = FlightCamera::earth_low_orbit().pose;
        let journey_director = journey_registry
            .resolve(&default_journey_id, &assets.catalog, &assets.spatial)
            .ok()
            .and_then(|manifest| JourneyDirector::from_manifest(manifest, initial_pose).ok());

        Ok(Self {
            catalog: assets.catalog,
            spatial: assets.spatial,
            atlas: assets.atlas,
            resolver: TargetResolver::new(ResolverConfig::default()),
            memory: SharedNarrationMemory::new(),
            fsm: RestNarrationFsm::new(FsmConfig::default()),
            flight: FlightCamera::earth_low_orbit(),
            wrist_ring,
            scale_meridian: ScaleMeridianArc::default(),
            audio,
            typo_layout: TypoLayoutEngine::default(),
            scene_builder: UniverseSceneBuilder::new(),
            journey_registry,
            journey_director,
            journey_recorder: JourneyRecorder::new(),
            user_journeys_dir,
            last_recorded_export: None,
            locale: Locale::EnUs,
            time_sec: 0.0,
            active_session: None,
            last_zone_sound_key: None,
            prev_fsm_kind: FsmStateKind::Moving,
            prev_hovered_bead: None,
            active_peripheral_prompt: None,
            guide_page: GuidePage::Hidden,
            guide_enabled: false,
            guide_is_vr: false,
            guide_hovered: None,
            guide_pointer_captured: false,
            guide_pointer_ray: None,
            guide_haptic: None,
            desktop_control_gesture: false,
            vr_control_gesture: false,
            narration_paused: false,
            vr_tracking_head: Vec3::ZERO,
            vr_tracking_origin: None,
            vr_tracking_orientation: glam::Quat::IDENTITY,
            vr_rig_orientation: glam::Quat::IDENTITY,
            vr_hologram_anchor: None,
            solar_observation_anchor: None,
            vr_ring_visible: false,
        })
    }

    /// Enables first-use guidance in an interactive session; deterministic gallery captures
    /// remain opt-in so they exercise the same content framing as before.
    pub fn enable_spatial_guide(&mut self, is_vr: bool) {
        self.guide_enabled = true;
        self.guide_is_vr = is_vr;
        self.guide_page = if self.is_journey_active() {
            GuidePage::Hidden
        } else {
            GuidePage::Welcome
        };
        self.sync_narration_pause();
    }

    /// Losing desktop focus releases input and pauses the tour; returning never resumes
    /// camera motion or speech until the user explicitly chooses Resume.
    pub fn pause_for_focus_loss(&mut self) {
        self.flight.linear_vel_cam = DVec3::ZERO;
        self.flight.angular_vel_rad = DVec3::ZERO;
        self.flight.scale_vel = 0.0;
        self.desktop_control_gesture = false;
        if self
            .journey_director
            .as_ref()
            .is_some_and(|d| d.play_state() == JourneyPlayState::Playing)
        {
            self.toggle_journey_play_pause();
        }
    }

    pub fn guide_frame(&self, camera: &RenderCamera) -> GuideFrame {
        if !self.guide_enabled {
            return GuideFrame::default();
        }
        if self.guide_is_vr && self.guide_page == GuidePage::Hidden && self.wrist_ring.open_alpha <= 0.1
        {
            return GuideFrame::default();
        }
        let tour = self
            .journey_director
            .as_ref()
            .filter(|d| d.is_active() || d.play_state() == JourneyPlayState::Completed)
            .map(|d| TourContext {
                title: d.manifest().title.pick(self.locale),
                chapter: d.active_chapter().title.pick(self.locale),
                stop: d.active_beat_index() + 1,
                total: d.total_beats(),
                paused: matches!(
                    d.play_state(),
                    JourneyPlayState::Paused | JourneyPlayState::PausedByLeash
                ),
                at_end: d.play_state() == JourneyPlayState::Completed,
            });
        let ring_hint = self.wrist_ring.hovered_bead_id.map(|id| match id {
            0 => "Return to Earth",
            1 => "Translate into English",
            2 => "Käännös suomeksi",
            10..=18 => "Narration playback speed",
            20 => "Play / pause the selected tour",
            21 => "Next stop on the tour",
            22 => "Record this viewpoint",
            23 => "Start / stop recording your flight",
            24 => "Save your recorded journey",
            _ => "How to explore",
        });
        let mut frame = GuideFrame::build(
            GuideContext {
                page: self.guide_page,
                locale: self.locale,
                is_vr: self.guide_is_vr,
                tour,
                ring_hint,
            },
            camera,
        );
        frame.hovered = self.guide_hovered;
        if self.guide_is_vr {
            frame.pointer_ray = self.guide_pointer_ray;
            frame.hologram_alpha = if self.guide_page == GuidePage::Hidden {
                self.wrist_ring.open_alpha
            } else {
                1.0
            };
        }
        if self.guide_is_vr
            && let Some((position, orientation)) = self.vr_hologram_anchor
        {
            frame.hologram = Some(crate::hologram::HologramPose {
                origin: self.vr_rig_orientation * (position - self.vr_tracking_head),
                orientation: self.vr_rig_orientation * orientation,
            });
        }
        frame
    }

    /// Locate the captured hologram in the current eye-relative render space.
    /// Its tracking-space pose stays fixed when the wearer turns or leans.
    pub fn update_vr_tracking(&mut self, head: Vec3, orientation: glam::Quat, rig: glam::Quat) {
        self.vr_tracking_origin.get_or_insert(head);
        self.vr_tracking_head = head;
        self.vr_tracking_orientation = orientation;
        self.vr_rig_orientation = rig;
        if self.vr_hologram_anchor.is_none() {
            self.reanchor_vr_guide();
        }
    }

    fn reanchor_vr_guide(&mut self) {
        self.vr_hologram_anchor = Some((self.vr_tracking_head, self.vr_tracking_orientation));
    }

    fn append_solar_observation_disclosure(&mut self, mode: Option<SolarObservationMode>, out: &mut TypoFrameOutput) {
        let Some(mode) = mode else {
            self.solar_observation_anchor = None;
            return;
        };
        if self.solar_observation_anchor.is_some_and(|(previous, _, _)| previous != mode) {
            self.solar_observation_anchor = None;
        }
        let vr = self.guide_is_vr;
        let (_, position, orientation) = *self.solar_observation_anchor.get_or_insert_with(|| {
            if vr {
                (mode, self.vr_tracking_head, self.vr_tracking_orientation)
            } else {
                (mode, Vec3::ZERO, self.flight.pose.orient.as_quat())
            }
        });
        let pose = if vr {
            crate::hologram::HologramPose {
                origin: self.vr_rig_orientation * (position - self.vr_tracking_head),
                orientation: self.vr_rig_orientation * orientation,
            }
        } else {
            crate::hologram::HologramPose { origin: Vec3::ZERO, orientation }
        };
        pose.append_observation_label(mode.disclosure(self.locale), &self.atlas, out);
    }

    fn tracked_render_pose(&self, physical_environment: bool) -> cosmos_math::ScalePose64 {
        let mut pose = self.flight.pose;
        if let Some(origin) = self.vr_tracking_origin {
            let displacement = self.vr_rig_orientation * (self.vr_tracking_head - origin);
            let observer_units_per_meter = if physical_environment {
                10.0_f64.powf(-pose.scale_exp)
            } else {
                1.0
            };
            let _ = pose.translate_render_units(
                displacement.as_dvec3() * observer_units_per_meter, &self.spatial.tree,
            );
        }
        pose
    }

    fn active_experience_for_frame(&self, visible: &[VisibleEntity], forced: Option<u8>) -> u8 {
        forced.or_else(|| {
            // A destination beat becomes active before its transition starts.
            // Do not change the current orbital world's units on that selection
            // frame; navigation releases ownership when it changes sector.
            if cosmos_render::universe_scene::earth_environment_is_physical(&self.flight.pose, 3) {
                return Some(3);
            }
            if self.flight.pose.sector_id == SectorId::SOLAR_SYSTEM {
                return Some(4);
            }
            let director = self.journey_director.as_ref().filter(|director| director.is_active())?;
            match director.active_beat().target {
                BeatTargetSelector::CatalogZone { experience_id, .. }
                | BeatTargetSelector::LexiconKey { experience_id, .. } => Some(experience_id),
                BeatTargetSelector::ChromaticRgb { .. } | BeatTargetSelector::ChromaticKey { .. } => Some(1),
                BeatTargetSelector::MusicFilament { .. } => Some(2),
                BeatTargetSelector::None => None,
            }
        }).unwrap_or_else(|| infer_active_experience(&self.flight.pose, visible))
    }

    pub fn toggle_spatial_help(&mut self) {
        if self.guide_page == GuidePage::Hidden {
            self.reanchor_vr_guide();
            if self
                .journey_director
                .as_ref()
                .is_some_and(|d| d.play_state() == JourneyPlayState::Playing)
            {
                self.toggle_journey_play_pause();
            }
            self.guide_page = GuidePage::Help;
        } else {
            self.guide_page = GuidePage::Hidden;
        }
        self.sync_narration_pause();
    }

    pub fn perform_guide_action(&mut self, action: GuideAction) {
        match action {
            GuideAction::Highlights | GuideAction::Odyssey => {
                let id = if action == GuideAction::Highlights {
                    "cosmic_highlights_tour"
                } else {
                    "grand_cosmic_odyssey"
                };
                if let Err(err) = self.activate_journey(id, true) {
                    eprintln!("[HyperCosmosVR] Unable to start tour: {err}");
                    return;
                }
                self.guide_page = GuidePage::Hidden;
            }
            GuideAction::Explore => {
                self.journey_confirm_exit();
                self.guide_page = GuidePage::Hidden;
            }
            GuideAction::Help => {
                self.toggle_spatial_help();
                self.guide_page = GuidePage::Help;
            }
            GuideAction::CloseHelp => self.guide_page = GuidePage::Hidden,
            GuideAction::ToggleTour => self.toggle_journey_play_pause(),
            GuideAction::Previous => self.journey_prev_beat(),
            GuideAction::Next => self.journey_next_beat(),
            GuideAction::Home => self.trigger_home(),
        }
        self.sync_narration_pause();
    }

    fn sync_narration_pause(&mut self) {
        let paused = (self.guide_enabled && self.guide_page != GuidePage::Hidden)
            || self.journey_director.as_ref().is_some_and(|d| {
                matches!(
                    d.play_state(),
                    JourneyPlayState::Paused | JourneyPlayState::PausedByLeash
                )
            });
        if paused != self.narration_paused {
            self.audio.set_narration_paused(paused);
            self.narration_paused = paused;
        }
    }

    fn step_guide_pointer(
        &mut self,
        camera: &RenderCamera,
        origin: Vec3,
        direction: Vec3,
        select: bool,
    ) -> bool {
        let frame = self.guide_frame(camera);
        let hovered = frame.hit_test(camera, origin, direction);
        if self.guide_is_vr && hovered.is_some() && hovered != self.guide_hovered {
            self.guide_haptic = Some((0.15, 8_000_000, 180.0));
        }
        self.guide_hovered = hovered;
        self.guide_pointer_ray = Some((origin, direction));
        let captured = frame.captures_pointer(camera, origin, direction);
        self.guide_pointer_captured = captured;
        if select && let Some(action) = self.guide_hovered {
            if self.guide_is_vr {
                self.guide_haptic = Some((0.4, 20_000_000, 210.0));
            }
            self.perform_guide_action(action);
        }
        captured
    }

    /// Loads (or replaces) the active Story-Journey by id, `.hcj.json` path, or `hypercosmos://` URI.
    pub fn activate_journey(&mut self, id_or_path_or_uri: &str, autoplay: bool) -> Result<()> {
        let manifest =
            self.journey_registry
                .resolve(id_or_path_or_uri, &self.catalog, &self.spatial)?;
        let mut director = JourneyDirector::from_manifest(manifest, self.flight.pose)?;
        if autoplay {
            director.start(&self.spatial.tree, &self.flight.pose, 0);
        }
        self.wrist_ring.journey_active = director.is_active();
        self.journey_director = Some(director);
        self.clear_tour_narration();
        Ok(())
    }

    pub fn is_journey_active(&self) -> bool {
        self.journey_director
            .as_ref()
            .is_some_and(|d| d.is_active())
    }

    pub fn has_tour_controls(&self) -> bool {
        self.journey_director
            .as_ref()
            .is_some_and(|d| d.play_state() != JourneyPlayState::Inactive)
    }

    pub fn is_journey_paused_by_leash(&self) -> bool {
        self.journey_director
            .as_ref()
            .is_some_and(|d| d.play_state() == JourneyPlayState::PausedByLeash)
    }

    pub fn active_peripheral_prompt(&self) -> Option<&PeripheralPromptVisual> {
        self.active_peripheral_prompt.as_ref()
    }

    pub fn toggle_journey_play_pause(&mut self) {
        self.guide_page = GuidePage::Hidden;
        if let Some(ref mut dir) = self.journey_director {
            dir.toggle_play_pause(&self.spatial.tree, &self.flight.pose);
            self.wrist_ring.journey_active = dir.is_active();
            if !dir.is_active() {
                self.active_peripheral_prompt = None;
            }
        }
        self.sync_narration_pause();
    }

    pub fn journey_next_beat(&mut self) {
        let mut changed = false;
        if let Some(ref mut dir) = self.journey_director {
            changed = dir.next_beat(&self.spatial.tree, &self.flight.pose);
            self.wrist_ring.journey_active = dir.is_active();
        }
        if changed {
            self.clear_tour_narration();
        }
        self.sync_narration_pause();
    }

    pub fn journey_prev_beat(&mut self) {
        let mut changed = false;
        if let Some(ref mut dir) = self.journey_director {
            changed = dir.prev_beat(&self.spatial.tree, &self.flight.pose);
            self.wrist_ring.journey_active = dir.is_active();
        }
        if changed {
            self.clear_tour_narration();
        }
        self.sync_narration_pause();
    }

    pub fn journey_next_chapter(&mut self) {
        let mut changed = false;
        if let Some(ref mut dir) = self.journey_director {
            changed = dir.next_chapter(&self.spatial.tree, &self.flight.pose);
            self.wrist_ring.journey_active = dir.is_active();
        }
        if changed {
            self.clear_tour_narration();
        }
        self.sync_narration_pause();
    }

    pub fn journey_prev_chapter(&mut self) {
        let mut changed = false;
        if let Some(ref mut dir) = self.journey_director {
            changed = dir.prev_chapter(&self.spatial.tree, &self.flight.pose);
            self.wrist_ring.journey_active = dir.is_active();
        }
        if changed {
            self.clear_tour_narration();
        }
        self.sync_narration_pause();
    }

    fn clear_tour_narration(&mut self) {
        self.audio.stop_narration();
        if let Some(session) = self.active_session.take()
            && let Some(note_id) = session.active_music_note_id
        {
            self.audio.music_note_off(note_id);
        }
        self.memory.invalidate_pending();
    }

    pub fn journey_confirm_follow(&mut self) {
        if let Some(ref mut dir) = self.journey_director {
            dir.follow_ghost_back();
            self.wrist_ring.journey_active = dir.is_active();
        }
    }

    pub fn journey_confirm_exit(&mut self) {
        if let Some(ref mut dir) = self.journey_director
            && dir.play_state() != JourneyPlayState::Inactive
        {
            let detached = dir.exit_to_free_flight(&self.spatial.tree);
            self.flight.pose = detached;
            self.wrist_ring.journey_active = false;
            self.active_peripheral_prompt = None;
            self.clear_tour_narration();
            self.flight.linear_vel_cam = DVec3::ZERO;
            self.flight.angular_vel_rad = DVec3::ZERO;
            self.flight.scale_vel = 0.0;
        }
        self.sync_narration_pause();
    }

    /// Records the current 6DoF camera+scale pose and focused target as a Story-Journey waypoint.
    pub fn record_manual_waypoint(&mut self) -> usize {
        let selector = self.current_focus_selector();
        let count = self
            .journey_recorder
            .record_manual_waypoint(&self.flight.pose, selector, None);
        self.wrist_ring.recorded_waypoint_count = count.min(u16::MAX as usize) as u16;
        count
    }

    /// Toggles continuous live flight trace capture (runs 4D log-scale RDP simplification on finish).
    pub fn toggle_live_trace_recording(&mut self) -> bool {
        let active = self.journey_recorder.toggle_live_trace(&self.spatial.tree);
        self.wrist_ring.live_trace_active = active;
        self.wrist_ring.recorded_waypoint_count =
            self.journey_recorder.waypoint_count().min(u16::MAX as usize) as u16;
        active
    }

    /// Exports recorded waypoints into `user_journeys/<id>.hcj.json` + `.share.txt`, registers it,
    /// and best-effort copies the `hypercosmos://` URI to the system clipboard for interactive calls.
    pub fn export_recorded_journey_interactive(&mut self) -> Result<ExportedJourneyPaths> {
        let exported = self.export_recorded_journey()?;
        if !cfg!(test) && std::env::var_os("HYPERCOSMOS_NO_CLIPBOARD").is_none() {
            try_copy_to_clipboard_best_effort(&exported.share_uri);
        }
        Ok(exported)
    }

    /// Deterministically forces the active journey into `PausedByLeash` with peripheral pearls populated.
    pub fn force_verification_leash_prompt(&mut self, cam_rot: glam::Quat) -> Option<PeripheralPromptVisual> {
        let dir = self.journey_director.as_mut()?;
        if !dir.is_active() {
            dir.start(&self.spatial.tree, &self.flight.pose, 0);
        }
        let ovr = ManualOverrideInput {
            move_local: Vec3::new(0.95, 0.0, 0.0),
            scale_delta_du: 0.75,
            ..Default::default()
        };
        let mut last_prompt = None;
        for _ in 0..20 {
            let out = dir.step(
                &self.spatial.tree,
                0.08,
                &ovr,
                Vec3::ZERO,
                cam_rot,
                None,
                None,
                false,
            );
            if out.peripheral_prompt.is_some() {
                last_prompt = out.peripheral_prompt;
            }
        }
        self.active_peripheral_prompt = last_prompt.clone();
        self.wrist_ring.journey_active = dir.is_active();
        last_prompt
    }

    /// Exports recorded waypoints into `user_journeys/<id>.hcj.json` + `.share.txt` and registers it.
    pub fn export_recorded_journey(&mut self) -> Result<ExportedJourneyPaths> {
        let seq = self.time_sec.round() as u32;
        let id = format!("user_journey_{seq:05}");
        let exported = self.journey_recorder.export_to_directory(
            &self.spatial.tree,
            &self.user_journeys_dir,
            &id,
            LocalizedTitle::new(
                format!("Mi Ruta Cósmica #{seq}"),
                format!("My Cosmic Route #{seq}"),
                format!("Oma Kosminen Reitti #{seq}"),
            ),
            "HyperCosmosVR User",
        )?;
        let _ = self.journey_registry.register_file(&exported.json_path);
        eprintln!(
            "[HyperCosmosVR] Exported {} beats to '{}' (Share URI saved in '{}')",
            exported.beat_count,
            exported.json_path.display(),
            exported.share_uri_path.display()
        );
        self.wrist_ring.live_trace_active = false;
        self.wrist_ring.recorded_waypoint_count =
            self.journey_recorder.waypoint_count().min(u16::MAX as usize) as u16;
        self.last_recorded_export = Some(exported.clone());
        Ok(exported)
    }

    fn current_focus_selector(&self) -> BeatTargetSelector {
        if let Some(ref sess) = self.active_session {
            return selector_from_plan(&sess.plan);
        }
        BeatTargetSelector::None
    }

    fn step_journey_director_and_recorder(
        &mut self,
        override_input: &ManualOverrideInput,
        cam_rot: glam::Quat,
        pointer_origin: Option<Vec3>,
        pointer_dir: Option<Vec3>,
        pointer_triggered: bool,
        dt_sec: f32,
    ) -> Option<BeatTargetSelector> {
        let selector_now = self.current_focus_selector();
        self.journey_recorder
            .step_live_trace(dt_sec, &self.flight.pose, selector_now);
        self.wrist_ring.live_trace_active = self.journey_recorder.live_trace_active();
        self.wrist_ring.recorded_waypoint_count =
            self.journey_recorder.waypoint_count().min(u16::MAX as usize) as u16;

        let mut pending_target = None;
        if let Some(ref mut dir) = self.journey_director {
            if dir.is_active() {
                let prev_scale = self.flight.pose.scale_exp;
                let step_out = dir.step(
                    &self.spatial.tree,
                    dt_sec,
                    override_input,
                    Vec3::ZERO,
                    cam_rot,
                    pointer_origin,
                    pointer_dir,
                    pointer_triggered,
                );
                if let Some(detached) = step_out.exited_to_free_flight_pose {
                    self.flight.pose = detached;
                    self.active_peripheral_prompt = None;
                } else if let Some(desired) = step_out.desired_camera_pose {
                    self.flight.pose = desired;
                    self.flight.scale_vel =
                        (self.flight.pose.scale_exp - prev_scale) / dt_sec.max(1e-4) as f64;
                    self.active_peripheral_prompt = step_out.peripheral_prompt;
                } else {
                    self.active_peripheral_prompt = None;
                }
                pending_target = step_out.trigger_target;
            } else {
                self.active_peripheral_prompt = None;
            }
            self.wrist_ring.journey_active = dir.is_active();
        }
        pending_target
    }

    fn trigger_journey_narration_selector(
        &mut self,
        selector: &BeatTargetSelector,
        visible: &[VisibleEntity],
    ) {
        let Some(focus) = selector.to_target_focus() else {
            return;
        };
        let target_id = match *selector {
            BeatTargetSelector::CatalogZone {
                experience_id,
                zone_index,
                feature_index,
                ..
            } => TargetIdentity {
                entity_id: ((experience_id as u64) << 40)
                    | ((zone_index as u64) << 16)
                    | (feature_index.unwrap_or(u16::MAX) as u64),
                experience_id,
                zone_index,
                feature_index: feature_index.unwrap_or(u16::MAX),
            },
            BeatTargetSelector::ChromaticKey { key_index, .. } => TargetIdentity {
                entity_id: (1u64 << 40) | (key_index as u64),
                experience_id: 1,
                zone_index: u16::MAX,
                feature_index: key_index,
            },
            BeatTargetSelector::ChromaticRgb { rgb } => TargetIdentity {
                entity_id: (1u64 << 40)
                    | ((rgb[0] as u64) << 16)
                    | ((rgb[1] as u64) << 8)
                    | (rgb[2] as u64),
                experience_id: 1,
                zone_index: u16::MAX,
                feature_index: u16::MAX,
            },
            BeatTargetSelector::MusicFilament { midi, timbre_idx } => TargetIdentity {
                entity_id: (2u64 << 40) | ((midi as u64) << 8) | (timbre_idx as u64),
                experience_id: 2,
                zone_index: u16::MAX,
                feature_index: midi as u16,
            },
            BeatTargetSelector::LexiconKey {
                experience_id,
                key_index,
            } => TargetIdentity {
                entity_id: ((experience_id as u64) << 40) | 0x8000_0000 | (key_index as u64),
                experience_id,
                zone_index: u16::MAX,
                feature_index: key_index,
            },
            BeatTargetSelector::None => return,
        };

        let anchor_pos_cam = visible
            .iter()
            .find(|v| {
                v.experience_id == target_id.experience_id
                    && (target_id.zone_index == u16::MAX || v.zone_index == target_id.zone_index)
            })
            .map(|v| v.pose.pos)
            .unwrap_or(Vec3::new(0.0, 0.0, -2.5));
        let surface_dist_m = Some(anchor_pos_cam.length().max(0.5));
        self.start_narration_from_focus(target_id, focus, anchor_pos_cam, surface_dist_m);
    }

    /// Toggles spoken translation locale cleanly between English (`en-US`) and Finnish (`fi-FI`).
    pub fn toggle_locale(&mut self) {
        self.locale = match self.locale {
            Locale::EnUs => Locale::FiFi,
            Locale::FiFi | Locale::EsEs => Locale::EnUs,
        };
        self.wrist_ring.selected_locale = self.locale;
        self.restart_active_session_for_locale_or_rate();
    }

    /// Steps narration playback speed upward/downward through canonical `1.0x .. 3.0x` steps.
    pub fn step_playback_rate(&mut self, delta_steps: i32) -> f32 {
        let cur_idx = self.wrist_ring.selected_rate_index as i32;
        let next_idx = (cur_idx + delta_steps).clamp(0, PLAYBACK_RATES.len() as i32 - 1) as usize;
        let rate = PLAYBACK_RATES[next_idx];
        self.wrist_ring.set_playback_rate(rate);
        self.audio.snap_playback_rate_bead(rate, true)
    }

    /// Triggers smooth continuous spline hyper-flight back to Earth Low Orbit (`u = +6.6`).
    pub fn trigger_home(&mut self) {
        self.journey_confirm_exit();
        self.guide_page = GuidePage::Hidden;
        self.sync_narration_pause();
        let _ = self.flight.start_home_flight(&self.spatial);
    }

    /// Advances Desktop frame: hover-scan picking -> 6DoF flight -> Rest-to-Learn FSM -> Audio -> MSDF -> Scene.
    #[allow(clippy::too_many_arguments)]
    pub fn step_desktop_frame(
        &mut self,
        desktop_input: &DesktopFlightInput,
        hover_ray_cam: Vec3,
        right_hold_ring: bool,
        tab_just_pressed: bool,
        click_just_pressed: bool,
        camera_for_pick: &RenderCamera,
        dt_sec: f32,
        width: u32,
        height: u32,
    ) -> FramePacket {
        let orient = self.flight.pose.orient.as_quat();
        let guide_captured = self.step_guide_pointer(
            camera_for_pick,
            Vec3::ZERO,
            orient * hover_ray_cam,
            click_just_pressed,
        );
        let mut preview_ring = self.wrist_ring.clone();
        preview_ring.step(
            &WristSummonInput {
                cam_pos: orient * Vec3::new(-0.34, -0.09, -0.02),
                cam_rot: orient,
                desktop_right_hold: right_hold_ring,
                desktop_tab_just_pressed: tab_just_pressed,
                pointer_origin: Some(Vec3::ZERO),
                pointer_dir: Some(orient * hover_ray_cam),
                ..Default::default()
            },
            0.0,
        );
        if !desktop_input.left_drag_active {
            self.desktop_control_gesture = false;
        }
        if click_just_pressed && (guide_captured || preview_ring.hovered_bead_id.is_some()) {
            self.desktop_control_gesture = true;
        }
        let mut safe_input = desktop_input.clone();
        if self.desktop_control_gesture {
            safe_input.left_drag_active = false;
            safe_input.mouse_delta_rad = glam::DVec2::ZERO;
        }
        let desktop_input = &safe_input;
        let click_just_pressed = click_just_pressed && !guide_captured;
        let physical_for_pick = cosmos_render::universe_scene::environment_is_physical(
            &self.flight.pose, self.active_experience_for_frame(&[], None));
        let mut visible_pre = self
            .spatial
            .query_visible(
                &self.flight.pose,
                cosmos_render::universe_scene::spatial_query_view_projection(camera_for_pick,
                    &self.flight.pose, physical_for_pick),
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

        let cam_orient_pre = self.flight.pose.orient.as_quat();
        let active_experience = self.active_experience_for_frame(&visible_pre, None);
        cosmos_render::universe_scene::restore_physical_body_query_positions(
            &self.spatial, &self.flight.pose, active_experience, physical_for_pick, &mut visible_pre);
        let focus_entity_pre = pick_best_entity_along_ray(&visible_pre,
            cam_orient_pre * hover_ray_cam, 6.0, &self.flight.pose, active_experience);
        let focus_target = focus_entity_pre
            .map(|vis| {
                let pt_cam = cam_orient_pre.conjugate().as_dquat() * vis.pose.pos.as_dvec3();
                FocusTarget::from_visible(vis, Some(pt_cam))
            })
            .or_else(|| Some(FocusTarget::from_ray(hover_ray_cam.as_dvec3(), 3.0, None)));

        let journey_active = self.is_journey_active();
        if !journey_active {
            let _ = self.flight.step_desktop(
                &self.spatial,
                desktop_input,
                focus_target.as_ref(),
                dt_sec as f64,
            );
        }

        let orient_pre = self.flight.pose.orient.as_quat();
        let cam_ray_pre = orient_pre * hover_ray_cam.normalize_or(Vec3::NEG_Z);
        let manual_override = ManualOverrideInput {
            move_local: Vec3::new(
                desktop_input.move_axes.x as f32,
                desktop_input.move_axes.y as f32,
                -desktop_input.move_axes.z as f32,
            ),
            yaw_delta_rad: if desktop_input.left_drag_active || desktop_input.free_flight_held {
                -desktop_input.mouse_delta_rad.x
            } else {
                0.0
            },
            pitch_delta_rad: if desktop_input.left_drag_active || desktop_input.free_flight_held {
                -desktop_input.mouse_delta_rad.y
            } else {
                0.0
            },
            scale_delta_du: -desktop_input.scroll_delta * 0.28,
        };
        let prompt_click_allowed = self.wrist_ring.hovered_bead_id.is_none();
        let pending_journey_target = self.step_journey_director_and_recorder(
            &manual_override,
            orient_pre,
            Some(Vec3::ZERO),
            Some(cam_ray_pre),
            click_just_pressed && prompt_click_allowed,
            dt_sec,
        );

        let orient = self.flight.pose.orient.as_quat();
        let cam_ray_world = orient * hover_ray_cam.normalize_or(Vec3::NEG_Z);
        let wrist_input = WristSummonInput {
            is_vr: false,
            // Dock the desktop virtual wrist ring in the lower-left periphery so it never
            // occludes the centered planet/nebula or active bilingual narration subtitle.
            cam_pos: orient * Vec3::new(-0.34, -0.09, -0.02),
            cam_rot: orient,
            desktop_right_hold: right_hold_ring,
            desktop_tab_just_pressed: tab_just_pressed,
            pointer_origin: Some(Vec3::ZERO),
            pointer_dir: Some(cam_ray_world),
            select_triggered: click_just_pressed,
            ..Default::default()
        };

        let next_camera = RenderCamera::mono(
            orient,
            camera_for_pick.fov_y_rad,
            camera_for_pick.aspect,
            camera_for_pick.near_plane,
        );

        let pkt = self.step_common(
            &next_camera,
            &wrist_input,
            hover_ray_cam,
            0.0,
            None,
            dt_sec,
            width,
            height,
            None,
        );
        if let Some(sel) = pending_journey_target {
            let visible_post = self
                .spatial
                .query_visible(
                    &self.flight.pose,
                    cosmos_render::universe_scene::spatial_query_view_projection(&next_camera,
                        &self.flight.pose, physical_for_pick),
                    0.05_f32.to_radians(),
                    Some(256),
                )
                .map(|q| q.visible)
                .unwrap_or_default();
            self.trigger_journey_narration_selector(&sel, &visible_post);
        }
        pkt
    }

    /// Advances OpenXR VR stereo frame using exact tracked HMD pose, controller/hand joints, and haptics.
    #[allow(clippy::too_many_arguments)]
    pub fn step_vr_frame(
        &mut self,
        vr_input: &VrFlightInput,
        wrist_input: &WristSummonInput,
        vr_camera: &RenderCamera,
        aim_ray_cam: Vec3,
        ray_angular_extra_deg_per_sec: f32,
        dt_sec: f32,
        width: u32,
        height: u32,
    ) -> FramePacket {
        let guide_captured = self.step_guide_pointer(
            vr_camera,
            wrist_input.pointer_origin.unwrap_or(Vec3::ZERO),
            wrist_input.pointer_dir.unwrap_or(vr_camera.cam_forward),
            wrist_input.select_triggered,
        );
        let mut safe_wrist = wrist_input.clone();
        safe_wrist.select_triggered &= !guide_captured;
        let mut preview_ring = self.wrist_ring.clone();
        let mut probe = safe_wrist.clone();
        probe.select_triggered = false;
        preview_ring.step(&probe, 0.0);
        if vr_input.trigger_pull <= 0.1 {
            self.vr_control_gesture = false;
        }
        if (guide_captured || preview_ring.hovered_bead_id.is_some())
            && (wrist_input.select_triggered || vr_input.trigger_pull > 0.1)
        {
            self.vr_control_gesture = true;
        }
        let mut safe_flight = vr_input.clone();
        if self.vr_control_gesture {
            safe_flight.trigger_pull = 0.0;
        }
        let vr_input = &safe_flight;
        let wrist_input = &safe_wrist;
        if vr_input.home_pressed {
            self.trigger_home();
        }
        let untracked_visible = self.spatial.query_visible(
            &self.flight.pose, vr_camera.cyclopean_view_projection(), 0.05_f32.to_radians(), Some(512),
        ).map(|q| q.visible).unwrap_or_default();
        let physical_environment = cosmos_render::universe_scene::environment_is_physical(
            &self.flight.pose, self.active_experience_for_frame(&untracked_visible, None),
        );
        let render_pose = self.tracked_render_pose(physical_environment);
        let mut visible_pre = self
            .spatial
            .query_visible(
                &render_pose,
                cosmos_render::universe_scene::spatial_query_view_projection(vr_camera,
                    &render_pose, physical_environment),
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

        let cam_orient_pre = self.flight.pose.orient.as_quat();
        let active_experience = self.active_experience_for_frame(&visible_pre, None);
        cosmos_render::universe_scene::restore_physical_body_query_positions(
            &self.spatial, &render_pose, active_experience, physical_environment, &mut visible_pre);
        let focus_entity_pre = pick_best_entity_along_ray(&visible_pre,
            cam_orient_pre * aim_ray_cam, 6.0, &render_pose, active_experience);
        let focus_target = focus_entity_pre
            .map(|vis| {
                let pt_cam = cam_orient_pre.conjugate().as_dquat() * vis.pose.pos.as_dvec3();
                FocusTarget::from_visible(vis, Some(pt_cam))
            })
            .or_else(|| Some(FocusTarget::from_ray(aim_ray_cam.as_dvec3(), 2.5, None)));

        let journey_active = self.is_journey_active();
        if !journey_active {
            let _ = self
                .flight
                .step_vr(&self.spatial, vr_input, focus_target.as_ref(), dt_sec as f64);
        }

        // Intentional thumbstick/vertical buttons ONLY; HMD head rotation and 6DoF roomscale translation
        // are NEVER fed into ManualOverrideInput, preserving unrestricted head-turn freedom.
        let manual_override = ManualOverrideInput {
            move_local: Vec3::new(
                vr_input.left_stick.x as f32,
                vr_input.vertical_axis as f32,
                -vr_input.left_stick.y as f32,
            ),
            yaw_delta_rad: -vr_input.right_stick.x * (dt_sec as f64) * 1.65,
            pitch_delta_rad: vr_input.pitch_axis * (dt_sec as f64) * 1.25,
            scale_delta_du: vr_input.right_stick.y * (dt_sec as f64) * 1.35,
        };
        let prompt_click_allowed = self.wrist_ring.hovered_bead_id.is_none();
        let pending_journey_target = self.step_journey_director_and_recorder(
            &manual_override,
            if vr_camera.cam_forward.length_squared() > 0.0 {
                self.flight.pose.orient.as_quat()
            } else {
                cam_orient_pre
            },
            wrist_input.pointer_origin,
            wrist_input.pointer_dir,
            wrist_input.select_triggered && prompt_click_allowed,
            dt_sec,
        );

        let pkt = self.step_common(
            vr_camera,
            wrist_input,
            aim_ray_cam,
            ray_angular_extra_deg_per_sec,
            Some(&self.flight.pose.orient.as_quat()),
            dt_sec,
            width,
            height,
            None,
        );
        if let Some(sel) = pending_journey_target {
            self.trigger_journey_narration_selector(&sel, &visible_pre);
        }
        pkt
    }

    /// Deterministic frame build used by `--headless-render` after teleporting camera to a showcase scale.
    #[allow(clippy::too_many_arguments)]
    pub fn step_verification_frame(
        &mut self,
        camera: &RenderCamera,
        aim_ray_cam: Vec3,
        override_cfg: Option<&VerificationNarrationOverride>,
        dt_sec: f32,
        width: u32,
        height: u32,
    ) -> FramePacket {
        if let Some(ov) = override_cfg
            && let Some(r) = ov.force_playback_rate
        {
            self.wrist_ring.set_playback_rate(r);
            self.audio.set_playback_rate(r);
        }
        if let Some(ov) = override_cfg
            && let Some(v) = ov.force_scale_vel
        {
            self.flight.scale_vel = v;
        }
        let summon = override_cfg.is_some_and(|o| o.summon_wrist_ring);
        let orient = self.flight.pose.orient.as_quat();
        let wrist_input = WristSummonInput {
            is_vr: false,
            cam_pos: orient * Vec3::new(-0.34, -0.09, -0.02),
            cam_rot: orient,
            desktop_right_hold: summon,
            ..Default::default()
        };

        self.step_common(
            camera,
            &wrist_input,
            aim_ray_cam,
            0.0,
            None,
            dt_sec,
            width,
            height,
            override_cfg,
        )
    }

    /// Forces narration FSM directly into `Narrating` on the top ray-focused entity for deterministic captures.
    pub fn settle_and_start_focus_narration(&mut self, camera: &RenderCamera, aim_ray_cam: Vec3) {
        self.settle_and_start_experience_narration(camera, aim_ray_cam, None);
    }

    /// Forces narration FSM directly into `Narrating`, optionally scoping focus to `preferred_exp_id`
    /// so deterministic verification captures lock onto the intended cosmic archetype.
    pub fn settle_and_start_experience_narration(
        &mut self,
        camera: &RenderCamera,
        aim_ray_cam: Vec3,
        preferred_exp_id: Option<u8>,
    ) {
        self.settle_and_start_target_narration(camera, aim_ray_cam, preferred_exp_id, None, None);
    }

    /// Forces narration FSM directly into `Narrating`, optionally locking onto an explicit
    /// `(preferred_exp_id, preferred_zone_index, preferred_feature_index)` documentary target.
    pub fn settle_and_start_target_narration(
        &mut self,
        camera: &RenderCamera,
        aim_ray_cam: Vec3,
        preferred_exp_id: Option<u8>,
        preferred_zone_index: Option<u16>,
        preferred_feature_index: Option<u16>,
    ) {
        self.flight.linear_vel_cam = DVec3::ZERO;
        self.flight.angular_vel_rad = DVec3::ZERO;
        self.flight.scale_vel = 0.0;
        // Reset FSM first so any previous target clears cleanly.
        self.fsm = RestNarrationFsm::new(FsmConfig::default());
        self.active_session = None;
        self.audio.stop_narration();

        let physical = cosmos_render::universe_scene::environment_is_physical(
            &self.flight.pose, self.active_experience_for_frame(&[], preferred_exp_id));
        let mut visible = self
            .spatial
            .query_visible(
                &self.flight.pose,
                cosmos_render::universe_scene::spatial_query_view_projection(camera, &self.flight.pose, physical),
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

        let active_experience = self.active_experience_for_frame(&visible, preferred_exp_id);
        cosmos_render::universe_scene::restore_physical_body_query_positions(
            &self.spatial, &self.flight.pose, active_experience, physical, &mut visible);

        if let Some(exp_id) = preferred_exp_id
            && (preferred_zone_index.is_some() || preferred_feature_index.is_some())
        {
            let target_zone = preferred_zone_index.unwrap_or(0);
            let exact_vis = visible.iter().find(|v| {
                v.experience_id == exp_id
                    && v.zone_index == target_zone
                    && match preferred_feature_index {
                        Some(fi) => v.kind == EntityKind::FeatureNode && v.feature_index == fi,
                        None => matches!(
                            v.kind,
                            EntityKind::ZoneShell
                                | EntityKind::CelestialBody
                                | EntityKind::GalacticLandmark
                        ),
                    }
            });
            let entity_id = exact_vis
                .map(|v| v.id)
                .or_else(|| {
                    preferred_feature_index
                        .and_then(|fi| self.spatial.find_feature_entity(exp_id, target_zone, fi))
                        .map(|e| e.id)
                })
                .or_else(|| self.spatial.find_zone_entity(exp_id, target_zone).map(|e| e.id))
                .unwrap_or(u64::MAX - exp_id as u64);
            let target = TargetIdentity {
                entity_id,
                experience_id: exp_id,
                zone_index: target_zone,
                feature_index: preferred_feature_index.unwrap_or(u16::MAX),
            };
            let hint = exact_vis
                .or_else(|| {
                    visible
                        .iter()
                        .find(|v| v.experience_id == exp_id && v.zone_index == target_zone)
                })
                .map(|v| v.pose.pos.as_dvec3());
            self.start_narration_for_target_internal(target, &visible, hint, false);
            return;
        }

        let cam_orient = camera_basis_quat(camera);
        let sector_pref =
            preferred_exp_id.or_else(|| sector_experience_preference(self.flight.pose.sector_id));
        let candidates = build_focus_candidates_filtered(&visible, cam_orient, sector_pref,
            preferred_exp_id, &self.flight.pose, active_experience);
        let ray = aim_ray_cam.normalize_or(Vec3::NEG_Z);

        // Drive RestNarrationFsm cleanly across 10 x 100ms stationary steps (crossing 200ms -> 900ms lock!).
        for _ in 0..10 {
            let out = self.fsm.step(&FsmInput {
                raw_ray_dir_cam: ray,
                linear_speed: 0.0,
                scale_speed: 0.0,
                dt_sec: 0.1,
                candidates: &candidates,
                narration_finished: false,
            });
            if let Some(target) = out.start_narration_for {
                let hit_dist = visible
                    .iter()
                    .find(|v| v.id == target.entity_id)
                    .map(|v| v.pose.distance_render_units.max(0.25))
                    .unwrap_or(2.5);
                let hint = if matches!(target.experience_id, 1 | 2) {
                    None
                } else {
                    Some(ray.as_dvec3() * hit_dist)
                };
                self.start_narration_for_target_internal(target, &visible, hint, true);
                return;
            }
        }

        if let Some(exp_id) = preferred_exp_id {
            self.start_narration_for_target_internal(
                TargetIdentity {
                    entity_id: u64::MAX - exp_id as u64,
                    experience_id: exp_id,
                    zone_index: 0,
                    feature_index: u16::MAX,
                },
                &visible,
                None,
                true,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn step_common(
        &mut self,
        camera: &RenderCamera,
        wrist_input: &WristSummonInput,
        ray_cam: Vec3,
        _extra_ang: f32,
        vr_hint: Option<&glam::Quat>,
        dt_sec: f32,
        width: u32,
        height: u32,
        verification_override: Option<&VerificationNarrationOverride>,
    ) -> FramePacket {
        let dt = dt_sec.clamp(0.0005, 0.1);
        self.time_sec += dt;

        // 1) Update ScaleMeridianArc & WristResonanceRing.
        self.scale_meridian
            .step(self.flight.pose.scale_exp, self.flight.scale_vel, dt);

        let mut haptic_pulse = self.guide_haptic.take();
        if let Some(action) = self.wrist_ring.step(wrist_input, dt) {
            match action {
                DiegeticAction::SetPlaybackRate(rate) => {
                    self.audio.snap_playback_rate_bead(rate, true);
                    haptic_pulse = Some((0.45, 22_000_000, 220.0));
                }
                DiegeticAction::SetLocale(loc) => {
                    if self.locale != loc {
                        self.locale = loc;
                        self.restart_active_session_for_locale_or_rate();
                    }
                    haptic_pulse = Some((0.55, 30_000_000, 180.0));
                }
                DiegeticAction::TriggerEarthHome => {
                    if self.is_journey_active() {
                        self.journey_confirm_exit();
                    }
                    self.trigger_home();
                    haptic_pulse = Some((0.7, 45_000_000, 140.0));
                }
                DiegeticAction::ToggleJourneyPlayPause => {
                    self.toggle_journey_play_pause();
                    haptic_pulse = Some((0.55, 28_000_000, 210.0));
                }
                DiegeticAction::NextJourneyBeat => {
                    self.journey_next_beat();
                    haptic_pulse = Some((0.45, 20_000_000, 240.0));
                }
                DiegeticAction::RecordJourneyWaypoint => {
                    let _ = self.record_manual_waypoint();
                    haptic_pulse = Some((0.60, 30_000_000, 260.0));
                }
                DiegeticAction::ToggleJourneyLiveTrace => {
                    let _ = self.toggle_live_trace_recording();
                    haptic_pulse = Some((0.65, 35_000_000, 195.0));
                }
                DiegeticAction::ExportRecordedJourney => {
                    let _ = self.export_recorded_journey_interactive();
                    haptic_pulse = Some((0.75, 45_000_000, 175.0));
                }
                DiegeticAction::ToggleHelp => self.toggle_spatial_help(),
            }
        }
        if self.wrist_ring.hovered_bead_id != self.prev_hovered_bead
            && self.wrist_ring.hovered_bead_id.is_some()
        {
            haptic_pulse = haptic_pulse.or(Some((0.25, 14_000_000, 260.0)));
        }
        self.prev_hovered_bead = self.wrist_ring.hovered_bead_id;
        let ring_visible = self.wrist_ring.open_alpha > 0.1;
        if self.guide_is_vr && ring_visible && !self.vr_ring_visible {
            self.reanchor_vr_guide();
        }
        self.vr_ring_visible = ring_visible;
        self.sync_narration_pause();

        // Resolve the experience and its unit policy from the untracked flight
        // anchor. A lean across Earth's surface cannot switch the scene's units.
        let mut render_pose = self.flight.pose;
        let physical_for_query = cosmos_render::universe_scene::environment_is_physical(
            &render_pose, self.active_experience_for_frame(&[], verification_override.map(|o| o.active_experience_id)));
        let mut visible = self
            .spatial
            .query_visible(
                &render_pose,
                cosmos_render::universe_scene::spatial_query_view_projection(camera, &render_pose, physical_for_query),
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

        let cam_orient = self.flight.pose.orient.as_quat();
        let active_experience_id = self.active_experience_for_frame(
            &visible, verification_override.map(|o| o.active_experience_id),
        );
        let physical_environment_units = cosmos_render::universe_scene::environment_is_physical(
            &self.flight.pose, active_experience_id,
        );
        if vr_hint.is_some() {
            render_pose = self.tracked_render_pose(physical_environment_units);
            visible = self.spatial.query_visible(
                &render_pose, cosmos_render::universe_scene::spatial_query_view_projection(camera,
                    &render_pose, physical_environment_units), 0.05_f32.to_radians(), Some(512),
            ).map(|q| q.visible).unwrap_or_default();
        }
        cosmos_render::universe_scene::restore_physical_body_query_positions(
            &self.spatial, &render_pose, active_experience_id, physical_environment_units, &mut visible);

        // Update zone drone synth gently when changing active cosmic experience/zone.
        self.sync_zone_ambient_audio(active_experience_id, &visible);

        // 3) Rest-to-Learn FSM step.
        let forced_exp = verification_override.map(|o| o.active_experience_id);
        let sector_pref =
            forced_exp.or_else(|| sector_experience_preference(self.flight.pose.sector_id));
        let candidates =
            build_focus_candidates_filtered(&visible, cam_orient, sector_pref, forced_exp,
                &render_pose, active_experience_id);
        let session_finished = self
            .active_session
            .as_ref()
            .is_some_and(|s| s.phase == SegmentPhase::Completed);

        let controls_focused = self.narration_paused
            || self.guide_pointer_captured
            || self.wrist_ring.hovered_bead_id.is_some();
        let fsm_out = if controls_focused && verification_override.is_none() {
            self.fsm.snapshot()
        } else {
            self.fsm.step(&FsmInput {
                raw_ray_dir_cam: ray_cam,
                linear_speed: if verification_override.is_some() {
                    0.0
                } else {
                    self.flight.linear_speed() as f32
                },
                scale_speed: if verification_override.is_some() {
                    0.0
                } else {
                    self.flight.scale_vel.abs() as f32
                },
                dt_sec: dt,
                candidates: &candidates,
                narration_finished: session_finished,
            })
        };

        let now_kind = fsm_out.state.kind();
        if now_kind == FsmStateKind::Locked && self.prev_fsm_kind != FsmStateKind::Locked {
            haptic_pulse = haptic_pulse.or(Some((0.35, 25_000_000, 190.0)));
        }
        self.prev_fsm_kind = now_kind;

        let journey_playing_rail = self
            .journey_director
            .as_ref()
            .is_some_and(|d| d.is_active());

        if fsm_out.should_cancel_audio && verification_override.is_none() && !journey_playing_rail {
            self.audio.stop_narration();
            self.memory.invalidate_pending();
            if let Some(sess) = &self.active_session
                && let Some(nid) = sess.active_music_note_id
            {
                self.audio.music_note_off(nid);
            }
            self.active_session = None;
        }

        if let Some(target_id) = fsm_out.start_narration_for
            && verification_override.is_none()
            && !journey_playing_rail
        {
            let point_hint = Some(fsm_out.smoothed_ray_dir_cam.as_dvec3() * 2.5);
            self.start_narration_for_target(target_id, &visible, point_hint);
        }

        // Keep spatial emitter direction + dwell audio feedback fresh in head-local coordinates.
        self.audio.set_dwell_progress(fsm_out.dwell_progress);
        if let Some(ref mut sess) = self.active_session {
            if let Some(vis) = visible.iter().find(|v| v.id == sess.target.entity_id) {
                sess.anchor_pos_cam = vis.pose.pos;
                sess.surface_dist_m = Some((vis.pose.distance_render_units as f32).max(0.4));
            }
            let dir_world = sess.anchor_pos_cam.normalize_or(Vec3::NEG_Z);
            let dir_head = camera_basis_quat(camera).conjugate() * dir_world;
            self.audio.set_emitter_direction(dir_head);
        }

        // Advance bilingual Spanish -> Translation playlist against real SyncVoice mixer clock.
        self.advance_audio_sequencer();

        // Pump headless software mixer when running in `AudioOutputMode::Headless` so
        // `AudioReactiveUniforms` and `SyncVoiceClockState` reflect real synthesized/decoded PCM.
        if self.audio.is_headless() {
            let frames = ((48_000.0 * dt).round() as usize).clamp(512, 2048);
            let mut pcm = vec![0.0_f32; frames * 2];
            let _ = self.audio.render_headless_frames(&mut pcm);
        }

        // 4) Build BilingualNarrationSpec (plus optional diegetic bead/meridian MSDF labels).
        let audio_uniforms = to_render_audio_uniforms(self.audio.reactive_uniforms());
        let documentary = self.build_documentary_focus_state(
            active_experience_id,
            &fsm_out,
            &visible,
            &audio_uniforms,
            verification_override,
        );
        let mut narration_spec =
            self.build_bilingual_spec(&fsm_out, active_experience_id, verification_override);
        if self.guide_enabled
            && self.guide_page != GuidePage::Hidden
            && verification_override.is_none()
        {
            narration_spec = None;
        }
        if let Some(ref mut spec) = narration_spec {
            spec.anchor_world_pos = resolve_narration_card_anchor(
                &visible,
                active_experience_id,
                spec.anchor_world_pos,
                verification_override,
            );
            if physical_environment_units {
                let meters_per_unit = 10.0_f64.powf(render_pose.scale_exp) as f32;
                let owner = if active_experience_id == 3 {
                    self.spatial.find_experience_anchor(3)
                } else {
                    self.spatial.find_zone_entity(4, documentary.active_zone_index)
                };
                if let Some(center) = owner.and_then(|entity| {
                    cosmos_render::universe_scene::physical_entity_position_m(&self.spatial, &render_pose, entity)
                }) {
                    let radius = if active_experience_id == 3 { 6_371_000.0 } else {
                        cosmos_spatial::solar_body_radius_m(documentary.active_zone_index).unwrap_or(0.0) as f32
                    };
                    spec.anchor_world_pos = center;
                    spec.surface_distance_m = Some((center.length() - radius).max(0.0));
                } else {
                    spec.anchor_world_pos *= meters_per_unit;
                    spec.surface_distance_m = spec.surface_distance_m.map(|distance| distance * meters_per_unit);
                }
            }
        }
        let focused_id = if verification_override.is_some() {
            self.active_session
                .as_ref()
                .map(|s| s.target.entity_id)
                .or_else(|| fsm_out.state.target().map(|t| t.entity_id))
        } else {
            fsm_out.state.target().map(|t| t.entity_id)
        };
        let mut ambient_inputs = build_ambient_label_inputs(
            &self.catalog,
            &visible,
            self.flight.pose.scale_exp,
            active_experience_id,
            documentary.active_zone_index,
            focused_id,
            self.locale,
        );
        if cosmos_render::universe_scene::is_close_solar_observer(&self.flight.pose, active_experience_id) {
            // Narrative and user-requested holograms remain available, but
            // catalog labels are not objects floating inside the solar plasma.
            ambient_inputs.clear();
        }
        if physical_environment_units {
            let meters_per_unit = 10.0_f64.powf(render_pose.scale_exp) as f32;
            for label in &mut ambient_inputs {
                // Glyph vergence remains in its comfort band, while its tether
                // ends at the actual distant planet instead of a miniature proxy.
                label.world_pos_cam_rel = self.spatial.iter_entities().find(|entity| entity.id == label.id)
                    .and_then(|entity| cosmos_render::universe_scene::physical_entity_position_m(&self.spatial, &render_pose, entity))
                    .unwrap_or(label.world_pos_cam_rel * meters_per_unit);
            }
        }
        self.append_diegetic_msdf_labels(camera, &mut ambient_inputs);
        let guide = if verification_override.is_none() {
            self.guide_frame(camera)
        } else {
            GuideFrame::default()
        };

        let cam_basis = CameraBasis {
            eye_center: Vec3::ZERO,
            forward: camera.cam_forward,
            right: camera.cam_right,
            up: camera.cam_up,
            fov_y_rad: camera.fov_y_rad,
            aspect: camera.aspect,
            u_cam: self.flight.pose.scale_exp,
            horizon_up: Vec3::Y,
        };

        let mut blockers = collect_external_blockers(
            &self.wrist_ring,
            &visible,
            active_experience_id,
            focused_id,
            camera,
        );
        blockers.extend(guide.blockers(camera));
        if let Some(ref prompt) = self.active_peripheral_prompt
            && prompt.alpha > 0.05
        {
            for pos in [prompt.follow_world, prompt.exit_world] {
                if let Some((az, el)) = project_world_to_angular(pos, camera) {
                    blockers.push(AngularRect {
                        az_center: az,
                        el_center: el,
                        half_az: 0.085,
                        half_el: 0.048,
                    });
                }
            }
        }
        let mut typo = self.typo_layout.build_frame_with_blockers(
            &self.atlas,
            &cam_basis,
            &ambient_inputs,
            narration_spec.as_ref(),
            &blockers,
            dt,
        );
        append_wrist_ring_tethers_and_labels(&self.wrist_ring, &self.atlas, camera, &mut typo);
        append_journey_tethers_and_labels(
            self.active_peripheral_prompt.as_ref(),
            &self.journey_recorder,
            &self.spatial.tree,
            &render_pose,
            &self.atlas,
            camera,
            &mut typo,
        );
        // This describes the observing instrument, not the lesson content.
        // Both manual selection and journey targets use the same shader mode.
        // Append before desktop foreground UI so these glyphs retain scene depth.
        let observing_mode = if verification_override.is_none() && active_experience_id == 4
            && documentary.active_zone_index == 0
        {
            match documentary.active_feature_index {
                0 => Some(SolarObservationMode::Interior),
                1 | 2 => Some(SolarObservationMode::VisibleDetail),
                3 | 4 => Some(SolarObservationMode::Euv),
                255 | u16::MAX => Some(SolarObservationMode::VisibleComposite),
                _ => None,
            }
        } else { None };
        self.append_solar_observation_disclosure(observing_mode, &mut typo);
        guide.append_typography(&self.atlas, camera, &mut typo);

        // 5) Build RenderSceneFrame (compute, far shell, volumetric heroes, diegetic pearls).
        let diegetic = DiegeticUiInput {
            wrist_pos: None, // Emitted directly with exact WristResonanceRing bead poses below.
            wrist_orient: self.wrist_ring.anchor_rot,
            playback_rate: self.wrist_ring.active_playback_rate(),
            locale_idx: self.locale as u8,
            focus_ray: cam_orient * fsm_out.smoothed_ray_dir_cam,
            dwell_progress: fsm_out.dwell_progress,
            show_focus_shimmer: true,
        };

        // Cold start (`u = 6.60`) and Surface (`Z00..=Z06`) present an intact Earth; deep mantle cutaway opens below `u = 6.42` for interior zones (`Z07..` or overview).
        let earth_cutaway_blend = verification_override
            .and_then(|o| o.force_earth_cutaway)
            .unwrap_or_else(|| {
                if active_experience_id == 3
                    && self.flight.pose.scale_exp < 6.42
                    && documentary.active_zone_index > 6
                {
                    (1.0 - smoothstep(5.40, 6.42, self.flight.pose.scale_exp) as f32).clamp(0.0, 1.0)
                } else {
                    0.0
                }
            });

        let mut scene = self.scene_builder.build(
            &self.spatial,
            &SceneBuildInput {
                cam_pose: render_pose,
                physical_environment_units: Some(physical_environment_units),
                camera: *camera,
                viewport_width: width,
                viewport_height: height,
                time_sec: self.time_sec,
                dt_sec: dt,
                scale_speed: self.flight.scale_vel as f32,
                active_experience_id,
                active_region_phase: self.time_sec * 0.25,
                focused_entity_id: focused_id,
                earth_cutaway_blend,
                fovea_rad: 0.34,
                particle_budget: 32_768,
                audio: audio_uniforms,
                diegetic,
                documentary,
                post: PostSettings::default(),
            },
        );

        append_diegetic_sprites(
            &self.wrist_ring,
            &self.scale_meridian,
            self.active_peripheral_prompt.as_ref(),
            &self.journey_recorder,
            &self.spatial.tree,
            &render_pose,
            camera,
            self.time_sec,
            &mut scene.sprites,
        );

        FramePacket {
            scene,
            typo,
            active_experience_id,
            scale_exp: self.flight.pose.scale_exp,
            hovered_entity_id: focused_id,
            fsm_state: now_kind,
            haptic_pulse,
            guide,
        }
    }

    /// Clears transient narration, meridian, wrist-ring, and temporal typography state between
    /// deterministic verification captures so ephemeral HUD elements never bleed across shots.
    pub fn reset_verification_state(&mut self) {
        self.memory.reset();
        self.fsm = RestNarrationFsm::new(self.fsm.config.clone());
        self.flight.linear_vel_cam = DVec3::ZERO;
        self.flight.angular_vel_rad = DVec3::ZERO;
        self.flight.scale_vel = 0.0;
        self.flight.pinch.active = false;
        self.flight.home_spline = None;
        self.wrist_ring = WristResonanceRing::default();
        self.scale_meridian = ScaleMeridianArc::default();
        self.active_peripheral_prompt = None;
        self.journey_recorder.clear();
        if let Some(ref mut dir) = self.journey_director
            && dir.is_active()
        {
            let _ = dir.exit_to_free_flight(&self.spatial.tree);
        }
        let max_ambient_labels = self.typo_layout.max_ambient_labels;
        let max_vertical_nudge_rad = self.typo_layout.max_vertical_nudge_rad;
        let fade_tau_sec = self.typo_layout.fade_tau_sec;
        self.typo_layout = TypoLayoutEngine::default();
        self.typo_layout.max_ambient_labels = max_ambient_labels;
        self.typo_layout.max_vertical_nudge_rad = max_vertical_nudge_rad;
        self.typo_layout.fade_tau_sec = fade_tau_sec;
        self.time_sec = 0.0;
        self.active_session = None;
        self.last_zone_sound_key = None;
        self.prev_fsm_kind = FsmStateKind::Moving;
        self.prev_hovered_bead = None;
        self.audio.stop_narration();
        self.audio.set_zone_sound(None, -1);
    }

    fn start_narration_for_target(
        &mut self,
        target: TargetIdentity,
        visible: &[VisibleEntity],
        point_hint_cam: Option<DVec3>,
    ) {
        self.start_narration_for_target_internal(target, visible, point_hint_cam, true);
    }

    fn start_narration_for_target_internal(
        &mut self,
        target: TargetIdentity,
        visible: &[VisibleEntity],
        point_hint_cam: Option<DVec3>,
        allow_signature_redirect: bool,
    ) {
        let vis = visible.iter().find(|v| v.id == target.entity_id);
        let (mut focus, anchor_pos_cam, surface_dist_m) = if let Some(v) = vis {
            (
                resolve_focus_for_visible(
                    &self.spatial,
                    v,
                    point_hint_cam,
                    self.flight.pose.scale_exp,
                    &self.flight.pose,
                ),
                v.pose.pos,
                Some((v.pose.distance_render_units as f32).max(0.5)),
            )
        } else {
            let anchor_pos = visible
                .iter()
                .find(|v| {
                    v.experience_id == target.experience_id
                        && (target.zone_index == u16::MAX || v.zone_index == target.zone_index)
                })
                .map(|v| v.pose.pos)
                .unwrap_or(Vec3::new(0.0, 0.0, -2.5));
            let focus = match target.experience_id {
                1 => TargetFocus::ChromaticLocal(DVec3::new(0.0, 0.25, 0.0)),
                2 => TargetFocus::MusicFilament {
                    midi: 69,
                    timbre_idx: 0,
                },
                exp => {
                    let feature_index = (target.feature_index != u16::MAX).then_some(target.feature_index);
                    TargetFocus::CatalogItem {
                        experience_id: exp.clamp(3, 25),
                        zone_index: target.zone_index,
                        feature_index,
                        is_macro_zoomed_out: feature_index.is_none(),
                    }
                }
            };
            (focus, anchor_pos, None)
        };

        if let Some(v) = vis
            && v.experience_id == 4 && v.kind == EntityKind::CelestialBody
        {
            // Body records carry feature_index=0 as indexing metadata. Looking
            // at the whole Sun requests its overview, not the core cutaway.
            // Explicit feature nodes and menu/tour target selectors stay exact.
            focus = TargetFocus::CatalogItem {
                experience_id: 4, zone_index: v.zone_index,
                feature_index: None, is_macro_zoomed_out: true,
            };
        }

        // Generic ExperienceAnchor targets default to zone_index 0 (often a transition gate in
        // Apps 05..14); steer them to each experience's signature core zone instead unless a caller
        // pinned an explicit zone.
        let is_generic_anchor =
            vis.is_none_or(|v| v.kind == EntityKind::ExperienceAnchor || v.zone_index == u16::MAX);
        if allow_signature_redirect
            && is_generic_anchor
            && let TargetFocus::CatalogItem {
                experience_id,
                zone_index,
                feature_index: None,
                ..
            } = &mut focus
            && (*zone_index == 0 || *zone_index == u16::MAX)
        {
            *zone_index = resolve_signature_zone_index(&self.catalog, *experience_id);
        }

        let Ok(plan) = self.resolver.resolve(&self.catalog, &focus) else {
            return;
        };
        let _ = plan;
        self.start_narration_from_focus(target, focus, anchor_pos_cam, surface_dist_m);
    }

    fn start_narration_from_focus(
        &mut self,
        target: TargetIdentity,
        focus: TargetFocus,
        anchor_pos_cam: Vec3,
        surface_dist_m: Option<f32>,
    ) {
        let Ok(plan) = self.resolver.resolve(&self.catalog, &focus) else {
            return;
        };
        self.audio.stop_narration();
        if let Some(ref prev) = self.active_session
            && let Some(nid) = prev.active_music_note_id
        {
            self.audio.music_note_off(nid);
        }

        let memory_state = plan.begin_memory(&mut self.memory, self.locale);

        let mut active_music_note_id = None;
        if let Some(ref mt) = plan.music_trigger {
            let note_id = target.entity_id ^ 0x4D55_5349_4300;
            self.audio
                .music_note_on(note_id, mt.midi, mt.timbre_index as usize, 0.0);
            active_music_note_id = Some(note_id);
        }

        // Prefetch upcoming Spanish & translation audio assets off-thread.
        let mut prefetch = Vec::new();
        for (i, seg) in plan.segments.iter().enumerate() {
            if memory_state.skipped.get(i).copied().unwrap_or(false) {
                continue;
            }
            prefetch.extend(seg.assets.playlist_for(Locale::EsEs));
            prefetch.extend(seg.assets.playlist_for(self.locale));
        }
        if !prefetch.is_empty() {
            self.audio.prefetch_neighbors(&self.catalog, &prefetch);
        }

        let first_idx = plan
            .segments
            .iter()
            .enumerate()
            .find(|(i, _)| !memory_state.skipped.get(*i).copied().unwrap_or(false))
            .map(|(i, _)| i)
            .unwrap_or(plan.segments.len());

        let mut sess = ActiveNarrationSession {
            target,
            anchor_pos_cam,
            surface_dist_m,
            plan,
            memory_state,
            seg_idx: first_idx,
            phase: SegmentPhase::Spanish,
            playlist: Vec::new(),
            playlist_pos: 0,
            active_asset_idx: None,
            last_es_text: String::new(),
            active_music_note_id,
        };

        if first_idx >= sess.plan.segments.len() {
            sess.phase = SegmentPhase::Completed;
            if let Some(last) = sess.plan.segments.last() {
                sess.last_es_text = last.spoken_text(Locale::EsEs).to_owned();
            }
            if let Some(ref mut dir) = self.journey_director {
                dir.notify_beat_narration_finished();
            }
        } else {
            self.start_current_segment_phase(&mut sess);
        }

        self.active_session = Some(sess);
    }

    fn restart_active_session_for_locale_or_rate(&mut self) {
        let Some(prev) = self.active_session.take() else {
            return;
        };
        let memory_state = prev.plan.begin_memory(&mut self.memory, self.locale);
        let seg_idx = prev.seg_idx.min(prev.plan.segments.len().saturating_sub(1));
        let mut next = ActiveNarrationSession {
            memory_state,
            seg_idx,
            phase: SegmentPhase::Spanish,
            playlist: Vec::new(),
            playlist_pos: 0,
            active_asset_idx: None,
            ..prev
        };
        self.start_current_segment_phase(&mut next);
        self.active_session = Some(next);
    }

    fn start_current_segment_phase(&self, sess: &mut ActiveNarrationSession) {
        while sess.seg_idx < sess.plan.segments.len() {
            if sess
                .memory_state
                .skipped
                .get(sess.seg_idx)
                .copied()
                .unwrap_or(false)
            {
                sess.seg_idx += 1;
                sess.phase = SegmentPhase::Spanish;
                continue;
            }
            let seg = &sess.plan.segments[sess.seg_idx];
            if sess.phase == SegmentPhase::Spanish {
                sess.last_es_text = non_empty_es(seg);
                let list = seg.assets.playlist_for(Locale::EsEs);
                if !list.is_empty() {
                    sess.playlist = list;
                    sess.playlist_pos = 0;
                    self.play_playlist_head(sess);
                    return;
                }
                sess.phase = SegmentPhase::Translation;
            }
            if sess.phase == SegmentPhase::Translation {
                let list = seg.assets.playlist_for(self.locale);
                if !list.is_empty() {
                    sess.playlist = list;
                    sess.playlist_pos = 0;
                    self.play_playlist_head(sess);
                    return;
                }
                // Nothing spoken in translation; advance cleanly.
                sess.seg_idx += 1;
                sess.phase = SegmentPhase::Spanish;
            }
        }
        sess.phase = SegmentPhase::Completed;
        sess.active_asset_idx = None;
    }

    fn play_playlist_head(&self, sess: &mut ActiveNarrationSession) {
        if let Some(&asset_idx) = sess.playlist.get(sess.playlist_pos) {
            let dir = sess.anchor_pos_cam.normalize_or(Vec3::NEG_Z);
            if self.audio.play_asset(&self.catalog, asset_idx, dir).is_ok() {
                sess.active_asset_idx = Some(asset_idx);
            } else {
                sess.active_asset_idx = None;
            }
        }
    }

    fn advance_audio_sequencer(&mut self) {
        if self.narration_paused {
            return;
        }
        let clock = self.audio.playback_clock();
        let Some(mut sess) = self.active_session.take() else {
            return;
        };
        if sess.phase == SegmentPhase::Completed {
            self.active_session = Some(sess);
            return;
        }

        let current_asset = sess.active_asset_idx;
        let clip_finished = match current_asset {
            Some(expected) => clock.asset_idx == Some(expected) && clock.is_finished,
            None => true,
        };

        if clip_finished {
            sess.playlist_pos += 1;
            if sess.playlist_pos < sess.playlist.len() {
                self.play_playlist_head(&mut sess);
            } else {
                match sess.phase {
                    SegmentPhase::Spanish => {
                        sess.phase = SegmentPhase::Translation;
                        self.start_current_segment_phase(&mut sess);
                        if sess.phase == SegmentPhase::Completed
                            && let Some(ref mut dir) = self.journey_director
                        {
                            dir.notify_beat_narration_finished();
                        }
                    }
                    SegmentPhase::Translation => {
                        let _ = self.memory.mark_pair_completed(
                            &sess.memory_state,
                            sess.seg_idx,
                            true,
                            true,
                        );
                        sess.seg_idx += 1;
                        sess.phase = SegmentPhase::Spanish;
                        self.start_current_segment_phase(&mut sess);
                        if let Some(ref mut dir) = self.journey_director {
                            if sess.phase == SegmentPhase::Completed
                                || sess.seg_idx >= sess.plan.segments.len()
                            {
                                dir.notify_beat_narration_finished();
                            } else {
                                dir.notify_translation_sentence_finished(&self.spatial.tree);
                            }
                        }
                    }
                    SegmentPhase::Completed => {}
                }
            }
        }

        self.active_session = Some(sess);
    }

    fn sync_zone_ambient_audio(&mut self, exp_id: u8, visible: &[VisibleEntity]) {
        let zone_idx = visible
            .iter()
            .find(|v| v.experience_id == exp_id && v.zone_index != u16::MAX)
            .map(|v| v.zone_index)
            .unwrap_or(0);
        let key = (exp_id, zone_idx);
        if self.last_zone_sound_key == Some(key) {
            return;
        }
        self.last_zone_sound_key = Some(key);
        let zones = self.catalog.zones(exp_id);
        if let Some(zone) = zones.get(zone_idx as usize).or_else(|| zones.first()) {
            let sound = (zone.has_sound != 0).then_some(zone.sound);
            self.audio.set_zone_sound(sound, zone.scene_index);
        } else {
            self.audio.set_zone_sound(None, exp_id as i32);
        }
    }

    fn current_live_cue_cursor(
        &self,
        sess: &ActiveNarrationSession,
        verification_override: Option<&VerificationNarrationOverride>,
    ) -> f32 {
        if let Some(ov) = verification_override {
            ov.phase_reveal.clamp(0.0, 1.0)
        } else if let Some(st) = self.audio.poll_sync_clock(&self.catalog)
            && Some(st.asset_idx) == sess.active_asset_idx
            && let Some(asset_rec) = self.catalog.audio_asset(st.asset_idx)
        {
            cue_reveal_cursor_for_asset(&self.catalog, asset_rec, st.source_ms)
        } else {
            1.0
        }
    }

    fn build_documentary_focus_state(
        &self,
        active_experience_id: u8,
        fsm_out: &FsmOutput,
        visible: &[VisibleEntity],
        audio: &RenderAudioUniforms,
        verification_override: Option<&VerificationNarrationOverride>,
    ) -> DocumentaryFocusState {
        // The flagship's visual sequence belongs to its authored beat, not to
        // the current audio sentence. It remains intact with muted/missing audio.
        if matches!(active_experience_id, 3 | 4) && verification_override.is_none()
            && let Some(director) = self.journey_director.as_ref().filter(|d| {
                d.is_active() && d.manifest().id == "cosmic_highlights_tour"
            })
            && let BeatTargetSelector::CatalogZone { experience_id, zone_index, feature_index, .. } = director.active_beat().target
            && experience_id == active_experience_id
        {
            let zone = self.catalog.zones(active_experience_id).get(zone_index as usize);
            return DocumentaryFocusState {
                active_zone_index: zone_index,
                active_feature_index: feature_index.unwrap_or(u16::MAX),
                scene_index: zone.map_or(-1, |z| z.scene_index),
                zone_accent_rgb: zone.map_or([0.0; 3], |z| srgb_to_linear_hdr(z.color_rgb)),
                telemetry: zone.map_or([0.0; 4], |z| [z.range_start, z.range_end, z.phys_low, z.phys_high]),
                ..Default::default()
            };
        }
        let matching_sess = self
            .active_session
            .as_ref()
            .filter(|s| s.plan.experience_id == active_experience_id);

        if let Some(sess) = matching_sess {
            let cue_progress = self.current_live_cue_cursor(sess, verification_override);
            let active_weight = if verification_override.is_some() {
                1.0
            } else if fsm_out.cancel_fade_alpha > 0.005 {
                fsm_out.cancel_fade_alpha.clamp(0.0, 1.0)
            } else if sess.phase == SegmentPhase::Completed {
                0.35
            } else {
                1.0
            };
            let snap = sess.plan.documentary_snapshot(
                Some(sess.seg_idx),
                cue_progress,
                active_weight,
            );
            let mut active_zone_index = snap.active_zone_index;
            let mut active_feature_index = snap.active_feature_index;
            let mut scene_index = snap.scene_index;
            let mut zone_accent_rgb = snap.zone_accent_rgb;
            let mut telemetry = snap.telemetry;

            if active_experience_id == 2 && verification_override.is_none() && audio.synth_b[2] > 0.01 {
                telemetry[3] = telemetry[3].max(audio.synth_b[2].clamp(0.0, 1.0));
            }
            if let Some(ov) = verification_override {
                if let Some(fz) = ov.force_zone_index {
                    if fz != active_zone_index && ov.force_feature_index.is_none() {
                        active_feature_index = u16::MAX;
                    }
                    active_zone_index = fz;
                    if let Some(zone) = self.catalog.zones(active_experience_id).get(fz as usize) {
                        scene_index = zone.scene_index;
                        zone_accent_rgb = srgb_to_linear_hdr(zone.color_rgb);
                        if active_experience_id >= 3 {
                            telemetry = [zone.range_start, zone.range_end, zone.phys_low, zone.phys_high];
                        }
                    }
                }
                if let Some(ff) = ov.force_feature_index {
                    active_feature_index = ff;
                }
            }
            let cue_tags_mask = if active_feature_index != u16::MAX {
                snap.cue_tags_mask | 0x01
            } else {
                snap.cue_tags_mask
            };
            return DocumentaryFocusState {
                active_zone_index,
                active_feature_index,
                scene_index,
                narration_cue_progress: snap.narration_cue_progress,
                narration_active_weight: snap.narration_active_weight,
                zone_accent_rgb,
                telemetry,
                cue_tags_mask,
                submode_override: snap.submode_override,
                emphasis_boost: snap.emphasis_boost,
            };
        }

        let hovered_target = fsm_out
            .state
            .target()
            .filter(|t| t.experience_id == active_experience_id);
        if hovered_target.is_none() && verification_override.is_none()
            && (cosmos_render::universe_scene::earth_environment_is_physical(&self.flight.pose, active_experience_id)
                || cosmos_render::universe_scene::is_close_solar_observer(&self.flight.pose, active_experience_id))
        {
            // Invisible catalog selectors are not a request to turn on field,
            // weather or surface teaching overlays in the natural opening view.
            return DocumentaryFocusState { active_zone_index: 0, ..Default::default() };
        }
        let hovered_vis = hovered_target
            .and_then(|t| visible.iter().find(|v| v.id == t.entity_id));
        let fallback_vis = hovered_vis.or_else(|| {
            visible
                .iter()
                .filter(|v| v.experience_id == active_experience_id && v.zone_index != u16::MAX)
                .max_by(|a, b| {
                    let tier = |k: EntityKind| match k {
                        EntityKind::FeatureNode
                        | EntityKind::ChromaticSample
                        | EntityKind::MusicalFilament => 3,
                        EntityKind::CelestialBody
                        | EntityKind::GalacticLandmark
                        | EntityKind::ZoneShell => 2,
                        _ => 1,
                    };
                    tier(a.kind)
                        .cmp(&tier(b.kind))
                        .then_with(|| a.salience_score.total_cmp(&b.salience_score))
                })
        });

        let active_zone_index = verification_override
            .and_then(|o| o.force_zone_index)
            .or_else(|| hovered_target.map(|t| t.zone_index))
            .or_else(|| fallback_vis.map(|v| v.zone_index))
            .unwrap_or(u16::MAX);
        let active_feature_index = verification_override
            .and_then(|o| o.force_feature_index)
            .or_else(|| hovered_target.and_then(|t| {
                (active_experience_id != 4 || hovered_vis.is_some_and(|v| v.kind == EntityKind::FeatureNode))
                    .then_some(t.feature_index)
            }))
            .or_else(|| fallback_vis.filter(|v| active_experience_id != 4 || v.kind == EntityKind::FeatureNode)
                .map(|v| v.feature_index))
            .unwrap_or(u16::MAX);

        let zones = self.catalog.zones(active_experience_id);
        let zone_opt = (active_zone_index != u16::MAX)
            .then(|| zones.get(active_zone_index as usize))
            .flatten();
        let scene_index = zone_opt.map_or(-1, |z| z.scene_index);
        let zone_accent_rgb = if let Some(z) = zone_opt {
            srgb_to_linear_hdr(z.color_rgb)
        } else {
            let accent = cosmos_render::experience_accent_linear(active_experience_id);
            [accent.x, accent.y, accent.z]
        };

        let telemetry = match active_experience_id {
            1 => {
                if let Some(vis) = fallback_vis
                    && let TargetFocus::ChromaticLocal(local) = resolve_focus_for_visible(
                        &self.spatial,
                        vis,
                        None,
                        self.flight.pose.scale_exp,
                        &self.flight.pose,
                    )
                {
                    let (h, s, l) = prism_local_to_hsl(local);
                    [
                        h as f32,
                        (s as f32 / 100.0).clamp(0.0, 1.0),
                        (l as f32 / 100.0).clamp(0.0, 1.0),
                        (vis.zone_index % 4) as f32,
                    ]
                } else {
                    [205.0, 0.82, 0.54, 0.0]
                }
            }
            2 => {
                if let Some(vis) = fallback_vis
                    && let TargetFocus::MusicFilament { midi, timbre_idx } = resolve_focus_for_visible(
                        &self.spatial,
                        vis,
                        None,
                        self.flight.pose.scale_exp,
                        &self.flight.pose,
                    )
                {
                    let freq = 440.0 * 2.0_f32.powf((midi as f32 - 69.0) / 12.0);
                    [midi as f32, timbre_idx as f32, freq, audio.synth_b[2].clamp(0.0, 1.0)]
                } else {
                    [
                        69.0,
                        audio.synth_b[1],
                        440.0,
                        audio.synth_b[2].clamp(0.0, 1.0),
                    ]
                }
            }
            _ => zone_opt
                .or_else(|| zones.first())
                .map_or([0.0; 4], |z| [z.range_start, z.range_end, z.phys_low, z.phys_high]),
        };

        let fallback_tags: [&str; 2] = [
            if active_feature_index != u16::MAX {
                "role:item"
            } else {
                ""
            },
            if active_experience_id == 3 && (7..=13).contains(&active_zone_index) {
                "unit:depth"
            } else if active_experience_id == 3 && active_zone_index <= 5 {
                "unit:altitude"
            } else {
                ""
            },
        ];
        DocumentaryFocusState {
            active_zone_index,
            active_feature_index,
            scene_index,
            narration_cue_progress: verification_override.map_or(0.0, |o| o.phase_reveal.clamp(0.0, 1.0)),
            narration_active_weight: fsm_out
                .cancel_fade_alpha
                .max(fsm_out.dwell_progress * 0.25)
                .clamp(0.0, 1.0),
            zone_accent_rgb,
            telemetry,
            cue_tags_mask: encode_cue_tags_mask(&fallback_tags),
            submode_override: 0,
            emphasis_boost: 1.0,
        }
    }

    fn build_bilingual_spec(
        &self,
        fsm_out: &FsmOutput,
        exp_id: u8,
        verification_override: Option<&VerificationNarrationOverride>,
    ) -> Option<BilingualNarrationSpec> {
        let opacity = if fsm_out.cancel_fade_alpha > 0.005 {
            fsm_out.cancel_fade_alpha
        } else {
            1.0
        };

        let sess = self.active_session.as_ref()?;
        let seg_count = sess.plan.segments.len();
        let active_idx = sess.seg_idx.min(seg_count.saturating_sub(1));
        let seg = sess.plan.segments.get(active_idx)?;

        let mut remembered_lines = Vec::new();
        for (i, s) in sess.plan.segments.iter().enumerate() {
            if (sess.memory_state.skipped.get(i).copied().unwrap_or(false)
                || i < active_idx)
                && matches!(
                    s.role,
                    SegmentRole::EvidenceClass
                        | SegmentRole::RegionHeading
                        | SegmentRole::SharedContext
                )
            {
                let text = s.display_text.trim();
                if !text.is_empty() {
                    remembered_lines.push(text.to_owned());
                }
            }
        }

        let live_cursor = self.current_live_cue_cursor(sess, verification_override);

        let es_line = non_empty_es(seg);
        let tr_line = seg.spoken_text(self.locale).trim().to_owned();

        let phase = if let Some(ov) = verification_override {
            if ov.show_spanish_ghost_translation {
                BilingualPhase::TranslationOverGhost {
                    trans_reveal: live_cursor,
                }
            } else {
                BilingualPhase::SpanishActive {
                    es_reveal: live_cursor,
                }
            }
        } else {
            match sess.phase {
                SegmentPhase::Spanish => BilingualPhase::SpanishActive {
                    es_reveal: live_cursor,
                },
                SegmentPhase::Translation => BilingualPhase::TranslationOverGhost {
                    trans_reveal: live_cursor,
                },
                SegmentPhase::Completed => BilingualPhase::Complete,
            }
        };

        let exp_title = self
            .catalog
            .experience(exp_id)
            .map(|e| self.catalog.str_slice(e.label))
            .unwrap_or("Cosmos");
        let loc_tag = match self.locale {
            Locale::EnUs => "ENGLISH",
            Locale::FiFi => "SUOMI",
            Locale::EsEs => "ESPAÑOL",
        };
        let speed_tag = format!("{:.2}x", self.wrist_ring.active_playback_rate());
        let eyebrow = Some(format!(
            "ES → {loc_tag} · {} · {speed_tag}",
            exp_title.to_uppercase()
        ));

        let active_heading = match seg.role {
            SegmentRole::RegionHeading | SegmentRole::Item => None,
            _ => {
                let region_heading = sess
                    .plan
                    .segments
                    .iter()
                    .find(|s| s.role == SegmentRole::RegionHeading)
                    .map(|s| s.display_text.trim())
                    .filter(|h| !h.is_empty() && !h.eq_ignore_ascii_case(es_line.trim()));
                if let Some(h) = region_heading {
                    Some(h.to_owned())
                } else {
                    let disp = seg.display_text.trim();
                    (!disp.is_empty() && !disp.eq_ignore_ascii_case(es_line.trim()))
                        .then(|| disp.to_owned())
                }
            }
        };

        let accent = cosmos_render::experience_accent_linear(exp_id);
        Some(BilingualNarrationSpec {
            anchor_world_pos: sess.anchor_pos_cam,
            surface_distance_m: sess.surface_dist_m,
            eyebrow,
            remembered_lines,
            active_heading,
            spanish_text: es_line,
            translation_text: (!tr_line.is_empty()).then_some(tr_line),
            footer_text: sess.plan.footer_banner.clone(),
            phase,
            accent_rgb: [accent.x, accent.y, accent.z],
            opacity,
            ..Default::default()
        })
    }

    fn append_diegetic_msdf_labels(
        &self,
        camera: &RenderCamera,
        ambient_inputs: &mut Vec<AmbientLabelInput>,
    ) {
        if self.scale_meridian.visible && self.scale_meridian.opacity > 0.08 {
            let orient = scale_meridian_basis_quat(camera);
            let radial_right = orient * Vec3::X;
            for tick in self.scale_meridian.sample_ticks(Vec3::ZERO, orient, 1.45) {
                if !tick.is_major && tick.scale_exp_tick != self.flight.pose.scale_exp.round() as i32
                {
                    continue;
                }
                let mut lbl = AmbientLabelInput::landmark(
                    950_000 + (tick.scale_exp_tick + 100) as u64,
                    format!("10^{:+} m", tick.scale_exp_tick),
                    tick.world_pos + radial_right * 0.055,
                    40.0,
                    -50.0,
                    50.0,
                );
                lbl.role = FontRole::UiSans;
                lbl.target_angle_rad = 0.72_f32.to_radians();
                lbl.anchor_mode = AnchorMode::WorldExact;
                lbl.color_hdr = [0.65, 1.05, 1.35, tick.alpha];
                ambient_inputs.push(lbl);
            }
        }
    }

    /// Helper exposing current spoken Spanish text length or sync cue metadata cleanly.
    pub fn active_cue_asset(&self) -> Option<&AudioAssetRecord> {
        let idx = self.active_session.as_ref()?.active_asset_idx?;
        self.catalog.audio_asset(idx)
    }
}

/// Converts `cosmos_audio::AudioReactiveUniforms` into identical 64-byte `cosmos_render` uniforms.
#[inline]
pub fn to_render_audio_uniforms(a: cosmos_audio::AudioReactiveUniforms) -> RenderAudioUniforms {
    RenderAudioUniforms {
        levels_a: a.levels_a,
        levels_b: a.levels_b,
        synth_a: a.synth_a,
        synth_b: a.synth_b,
    }
}

#[cfg(windows)]
fn try_copy_to_clipboard_best_effort(text: &str) {
    use std::io::Write;
    use std::process::{Command, Stdio};
    if let Ok(mut child) = Command::new("clip")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
    {
        if let Some(mut stdin) = child.stdin.take() {
            let _ = stdin.write_all(text.as_bytes());
        }
        let _ = child.wait();
    }
}

#[cfg(not(windows))]
fn try_copy_to_clipboard_best_effort(_text: &str) {}

fn non_empty_es(seg: &NarrationSegment) -> String {
    let spoken = seg.spoken_text(Locale::EsEs).trim();
    if spoken.is_empty() {
        seg.display_text.trim().to_owned()
    } else {
        spoken.to_owned()
    }
}

fn infer_active_experience(pose: &cosmos_math::ScalePose64, visible: &[VisibleEntity]) -> u8 {
    // Orbit is a place, not whichever shell happens to cross the current head
    // view. Keep its physical-unit policy stable until navigation changes sector.
    if cosmos_render::universe_scene::earth_environment_is_physical(pose, 3) {
        return 3;
    }
    // 1. Direct sector match (exact for 01 Colors Prism, 02 Music Harp, 17 Mind, 25 Lotus, etc.).
    let placements = all_experience_placements();
    if let Some(p) = placements.iter().find(|p| p.sector_id == pose.sector_id)
        && pose.sector_id != SectorId::EARTH_ORBIT
        && pose.sector_id != SectorId::ORION_SPUR
    {
        return p.id.0;
    }

    // 2. Highest-salience hero or foreground visible entity inside active view cone.
    if let Some(best) = visible
        .iter()
        .find(|v| matches!(v.kind, EntityKind::ExperienceAnchor) && v.pose.pos.length() < 12.0)
        .or_else(|| visible.first())
    {
        return best.experience_id;
    }

    // 3. Fallback by continuous scale exponent `u_cam`.
    let u = pose.scale_exp;
    if u < -22.0 {
        14
    } else if u < -4.0 {
        13
    } else if u < 6.9 {
        3
    } else if u < 13.8 {
        4
    } else if u < 21.2 {
        5
    } else if u < 22.8 {
        6
    } else if u < 24.5 {
        7
    } else if u < 26.0 {
        8
    } else if u < 26.7 {
        9
    } else if u < 27.3 {
        10
    } else {
        25
    }
}

fn physical_body_angular_radius(vis: &VisibleEntity, physical: bool, active_experience: u8) -> Option<f32> {
    if !physical || vis.experience_id != active_experience || vis.kind != EntityKind::CelestialBody {
        return None;
    }
    let distance = vis.pose.pos.length();
    let ratio = vis.pose.rel_scale / distance;
    Some(if ratio >= 1.0 { std::f32::consts::PI } else { ratio.clamp(0.0, 1.0).asin() })
}

fn pick_best_entity_along_ray<'a>(
    visible: &'a [VisibleEntity],
    ray_world: Vec3,
    max_angle_deg: f32,
    observer: &cosmos_math::ScalePose64,
    active_experience: u8,
) -> Option<&'a VisibleEntity> {
    let ray = ray_world.normalize_or(Vec3::NEG_Z);
    let max_rad = max_angle_deg.to_radians();
    let mut best: Option<(&VisibleEntity, f32)> = None;

    for vis in visible {
        if !cosmos_render::universe_scene::solar_observer_allows_entity(observer, active_experience, vis) {
            continue;
        }
        let len = vis.pose.pos.length();
        let physical = cosmos_render::universe_scene::environment_is_physical(observer, active_experience);
        if !len.is_finite() || len <= if physical { 0.0 } else { 1e-4 } {
            continue;
        }
        let dir = vis.pose.pos / len;
        let theta = dir.dot(ray).clamp(-1.0, 1.0).acos();
        let radius = physical_body_angular_radius(vis, physical, active_experience)
            .unwrap_or(vis.pose.angular_rad * 0.5);
        let eff_cone = max_rad.max(radius + 0.03);
        if theta > eff_cone {
            continue;
        }
        let score = vis.salience_score * (-(theta / eff_cone.max(1e-4)).powi(2)).exp();
        if best.is_none_or(|(_, s)| score > s) {
            best = Some((vis, score));
        }
    }
    best.map(|(v, _)| v)
}

#[inline]
fn camera_basis_quat(camera: &RenderCamera) -> glam::Quat {
    glam::Quat::from_mat3(&glam::Mat3::from_cols(
        camera.cam_right,
        camera.cam_up,
        -camera.cam_forward,
    ))
    .normalize()
}

#[inline]
fn sector_experience_preference(sector_id: SectorId) -> Option<u8> {
    match sector_id {
        SectorId::COLORS_PRISM => Some(1),
        SectorId::MUSIC_HARP => Some(2),
        SectorId::EARTH_ORBIT | SectorId::EARTH_CORE | SectorId::EARTH_SURFACE => Some(3),
        SectorId::SOLAR_SYSTEM => Some(4),
        SectorId::MILKY_WAY => Some(5),
        SectorId::LOCAL_GROUP => Some(6),
        SectorId::LANIAKEA => Some(7),
        SectorId::COSMIC_WEB => Some(8),
        SectorId::OBSERVABLE_CMB => Some(9),
        SectorId::EARLY_INFLATION => Some(10),
        SectorId::HYPER_LOTUS_11_EVIDENCE => Some(11),
        SectorId::HYPER_LOTUS_12_FUTURE => Some(12),
        SectorId::QUANTUM_FIELD => Some(13),
        SectorId::PLANCK_ORIGIN => Some(14),
        SectorId::COMPLEXITY_HELIX => Some(15),
        SectorId::LIFE_BIOSPHERE => Some(16),
        SectorId::MIND_CATHEDRAL => Some(17),
        SectorId::CULTURE_WEB => Some(18),
        SectorId::TECH_RING => Some(19),
        SectorId::FUTURES_ARCS => Some(20),
        SectorId::HYPER_LOTUS_23_PERSPECTIVES => Some(23),
        SectorId::HYPER_LOTUS_24_MEANING => Some(24),
        SectorId::HYPER_LOTUS_25_HORIZON => Some(25),
        _ => None,
    }
}

fn project_world_to_angular(pos: Vec3, camera: &RenderCamera) -> Option<(f32, f32)> {
    let len = pos.length();
    if len <= 1e-4 {
        return None;
    }
    let dir = pos / len;
    let fwd = camera.cam_forward.normalize_or(Vec3::NEG_Z);
    if dir.dot(fwd) <= 0.15 {
        return None;
    }
    let right = camera.cam_right.normalize_or(Vec3::X);
    let up = camera.cam_up.normalize_or(Vec3::Y);
    let az = dir.dot(right).atan2(dir.dot(fwd));
    let el = dir.dot(up).clamp(-0.999, 0.999).asin();
    Some((az, el))
}

/// Keeps narration card leader tethers anchored onto the sculpted 3D stage centerpiece
/// (`EntityKind::ExperienceAnchor`) whenever zone/feature sub-states (`11..=25`, `23..=24`)
/// modulate the central sculpture rather than spawning separate outer-spiral meshes.
fn resolve_narration_card_anchor(
    visible: &[VisibleEntity],
    exp_id: u8,
    fallback_anchor: Vec3,
    verification_override: Option<&VerificationNarrationOverride>,
) -> Vec3 {
    let center_stage_verification =
        verification_override.is_some() && matches!(exp_id, 11..=25);
    if (center_stage_verification || matches!(exp_id, 23 | 24))
        && let Some(anchor) = visible
            .iter()
            .find(|v| v.experience_id == exp_id && v.kind == EntityKind::ExperienceAnchor)
    {
        return anchor.pose.pos;
    }
    fallback_anchor
}

fn push_entity_blocker(
    blockers: &mut Vec<AngularRect>,
    entity: &VisibleEntity,
    camera: &RenderCamera,
) {
    let Some((az, el)) = project_world_to_angular(entity.pose.pos, camera) else {
        return;
    };
    let (half_az, half_el) = if entity.kind == EntityKind::CelestialBody
        && entity.experience_id == 4
        && entity.zone_index == 7
    {
        // The ring proxy is 2.25 times the surface proxy, whose solid sphere is
        // at local radius .84. Project its actual bound; angular_rad stores a
        // full diameter, and a fixed angular cap can uncover nearby rings.
        let radius_over_distance = entity.pose.rel_scale
            / (entity.pose.distance_render_units as f32).max(1e-6);
        (
            (radius_over_distance * 2.25 / 0.84).clamp(0.0, 1.0).asin() + 3.2_f32.to_radians(),
            (radius_over_distance * 1.55).clamp(0.0, 1.0).asin() + 3.0_f32.to_radians(),
        )
    } else if entity.kind == EntityKind::ExperienceAnchor
        && matches!(
            entity.experience_id,
            1 | 2 | 5 | 12 | 16 | 19 | 20 | 21 | 22 | 23 | 24
        )
    {
        // Multi-part lateral stages (Prism+Bicone, Sgr A* disk, flanking planets, twin gears, 6-lens ring, balance scales).
        (
            (entity.pose.angular_rad * 1.48).clamp(0.12, 0.34) + 3.2_f32.to_radians(),
            entity.pose.angular_rad.clamp(0.10, 0.26) + 2.8_f32.to_radians(),
        )
    } else {
        let r = entity.pose.angular_rad.clamp(0.06, 0.30) + 2.8_f32.to_radians();
        (r, r)
    };
    blockers.push(AngularRect {
        az_center: az,
        el_center: el,
        half_az,
        half_el,
    });
}

fn collect_external_blockers(
    wrist_ring: &WristResonanceRing,
    visible: &[VisibleEntity],
    active_experience_id: u8,
    focused_id: Option<u64>,
    camera: &RenderCamera,
) -> Vec<AngularRect> {
    let mut blockers = Vec::with_capacity(5);

    // Protect the entire Wrist Resonance Ring + radial bead micro-labels from any label or card overlap.
    if wrist_ring.open_alpha > 0.05 && !wrist_ring.beads.is_empty() {
        let mut min_az = f32::INFINITY;
        let mut max_az = f32::NEG_INFINITY;
        let mut min_el = f32::INFINITY;
        let mut max_el = f32::NEG_INFINITY;
        let mut any = false;
        for bead in &wrist_ring.beads {
            if let Some((az, el)) = project_world_to_angular(bead.world_pos, camera) {
                min_az = min_az.min(az);
                max_az = max_az.max(az);
                min_el = min_el.min(el);
                max_el = max_el.max(el);
                any = true;
            }
        }
        if any {
            let pad_az = 5.5_f32.to_radians();
            let pad_el = 4.2_f32.to_radians();
            blockers.push(AngularRect {
                az_center: 0.5 * (min_az + max_az),
                el_center: 0.5 * (min_el + max_el),
                half_az: 0.5 * (max_az - min_az) + pad_az,
                half_el: 0.5 * (max_el - min_el) + pad_el,
            });
        }
    }

    // Protect the active experience's centerpiece geometry + focused planet/landmark so narration cards
    // and ambient tags float clear around extended rings, accretion disks, and flanking bodies.
    let active_anchor = (active_experience_id != 4)
        .then(|| {
            visible.iter().find(|v| {
                v.experience_id == active_experience_id && v.kind == EntityKind::ExperienceAnchor
            })
        })
        .flatten();
    if let Some(anchor) = active_anchor {
        push_entity_blocker(&mut blockers, anchor, camera);
    }

    let fwd = camera.cam_forward.normalize_or(Vec3::NEG_Z);
    let hero_cand = focused_id
        .and_then(|id| visible.iter().find(|v| v.id == id))
        .or_else(|| {
            visible
                .iter()
                .filter(|v| {
                    matches!(
                        v.kind,
                        EntityKind::CelestialBody | EntityKind::ExperienceAnchor | EntityKind::ZoneShell
                    )
                })
                .filter_map(|v| {
                    let dir = v.pose.pos.normalize_or_zero();
                    let cos_a = dir.dot(fwd);
                    (cos_a > 0.80).then_some((v, cos_a))
                })
                .max_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(v, _)| v)
        });

    if let Some(hero) = hero_cand
        && active_anchor.is_none_or(|a| a.id != hero.id)
        && matches!(
            hero.kind,
            EntityKind::CelestialBody | EntityKind::ExperienceAnchor | EntityKind::GalacticLandmark
        )
    {
        push_entity_blocker(&mut blockers, hero, camera);
    }

    blockers
}

fn selector_from_plan(plan: &NarrationPlan) -> BeatTargetSelector {
    match plan.experience_id {
        1 => BeatTargetSelector::ChromaticRgb {
            rgb: plan.accent_rgb,
        },
        2 => {
            if let Some(ref mt) = plan.music_trigger {
                BeatTargetSelector::MusicFilament {
                    midi: mt.midi,
                    timbre_idx: mt.timbre_index,
                }
            } else {
                BeatTargetSelector::MusicFilament {
                    midi: 69,
                    timbre_idx: 0,
                }
            }
        }
        exp if (3..=25).contains(&exp) && plan.zone_index != u16::MAX => {
            BeatTargetSelector::CatalogZone {
                experience_id: exp,
                zone_index: plan.zone_index,
                feature_index: plan.feature_index,
                is_macro_zoomed_out: plan.is_macro_overview,
            }
        }
        _ => BeatTargetSelector::None,
    }
}

struct PearlLabelStyle {
    color_hdr_rgb: [f32; 3],
    is_active: bool,
    alpha: f32,
}

fn push_world_pearl_label(
    atlas: &MsdfAtlas,
    camera: &RenderCamera,
    world_pos: Vec3,
    label: &str,
    radial_offset: Vec3,
    style: PearlLabelStyle,
    typo: &mut TypoFrameOutput,
) {
    let shaped = atlas.shape_text(FontRole::UiSans, label, None);
    if shaped.glyphs.is_empty() {
        return;
    }
    let right = camera.cam_right.normalize_or(Vec3::X);
    let up = camera.cam_up.normalize_or(Vec3::Y);
    let to_eye = -camera.cam_forward.normalize_or(Vec3::NEG_Z);
    let anchor = world_pos + radial_offset + to_eye * 0.004;

    let dist = anchor.length().max(0.2);
    let em_h = dist * (if style.is_active { 0.58_f32 } else { 0.42_f32 }).to_radians();
    let baseline_y_m = -0.35 * shaped.ascent_em * em_h;
    let color_hdr = if style.is_active {
        [
            (style.color_hdr_rgb[0] * 1.15).max(0.85),
            (style.color_hdr_rgb[1] * 1.15).max(0.85),
            (style.color_hdr_rgb[2] * 1.15).max(0.75),
            0.96 * style.alpha,
        ]
    } else {
        [0.64, 0.76, 0.90, 0.62 * style.alpha]
    };
    let halo_color = [
        0.002,
        0.003,
        0.006,
        if style.is_active { 0.85 } else { 0.60 } * style.alpha,
    ];
    let mode = if style.is_active {
        GlyphInkMode::DwellFocusedHeading as u8 as f32
    } else {
        GlyphInkMode::SpatialAnchor as u8 as f32
    };

    for sg in &shaped.glyphs {
        if !sg.metrics.has_outline {
            continue;
        }
        let line_w_em = shaped.lines[sg.line_index].width_em;
        let cx_world = ((sg.x_em - 0.5 * line_w_em)
            + 0.5 * (sg.metrics.plane_bounds[0] + sg.metrics.plane_bounds[2]))
            * em_h;
        let cy_world = baseline_y_m
            + (sg.y_em + 0.5 * (sg.metrics.plane_bounds[1] + sg.metrics.plane_bounds[3])) * em_h;
        let hw = 0.5 * (sg.metrics.plane_bounds[2] - sg.metrics.plane_bounds[0]) * em_h;
        let hh = 0.5 * (sg.metrics.plane_bounds[3] - sg.metrics.plane_bounds[1]) * em_h;
        if hw <= 1e-6 || hh <= 1e-6 {
            continue;
        }
        let pos = anchor + right * cx_world + up * cy_world;
        typo.glyphs.push(GlyphInstance {
            pos_center: [pos.x, pos.y, pos.z, 0.005],
            right_vec: [right.x * hw, right.y * hw, right.z * hw, sg.metrics.px_range],
            up_vec: [up.x * hh, up.y * hh, up.z * hh, 0.12],
            uv_rect: sg.metrics.uv_rect,
            color_hdr,
            halo_color,
            ink_params: [0.0, 1.0, 2.0, mode],
        });
    }
}

fn append_wrist_ring_tethers_and_labels(
    wrist_ring: &WristResonanceRing,
    atlas: &MsdfAtlas,
    camera: &RenderCamera,
    typo: &mut TypoFrameOutput,
) {
    if wrist_ring.open_alpha <= 0.05 || wrist_ring.beads.len() < 12 {
        return;
    }
    let alpha = wrist_ring.open_alpha.clamp(0.0, 1.0);
    let ring_color = [0.22, 0.72, 1.15, 0.12];

    // Outer 9 playback speed beads (`3..=11`) along the 200-deg astrolabe arc.
    for pair in wrist_ring.beads[3..12].windows(2) {
        let p0 = pair[0].world_pos;
        let p1 = pair[1].world_pos;
        typo.tethers.push(TetherInstance {
            p0_width: [p0.x, p0.y, p0.z, 0.0012],
            p1_alpha: [p1.x, p1.y, p1.z, 0.52 * alpha],
            color_hdr: ring_color,
        });
    }
    // Lower cradle linking arc endpoints -> EN (`1`) -> HOME (`0`) -> FI (`2`).
    for &(a, b) in &[(3usize, 1usize), (1, 0), (0, 2), (2, 11)] {
        let p0 = wrist_ring.beads[a].world_pos;
        let p1 = wrist_ring.beads[b].world_pos;
        typo.tethers.push(TetherInstance {
            p0_width: [p0.x, p0.y, p0.z, 0.0010],
            p1_alpha: [p1.x, p1.y, p1.z, 0.36 * alpha],
            color_hdr: [0.16, 0.52, 0.88, 0.15],
        });
    }
    // Outer lower Story-Journey & Recording arc tethers (`12..17`).
    if wrist_ring.beads.len() >= 17 {
        for &(a, b) in &[(3usize, 13usize), (13, 12), (11, 14), (14, 15), (15, 16)] {
            let p0 = wrist_ring.beads[a].world_pos;
            let p1 = wrist_ring.beads[b].world_pos;
            typo.tethers.push(TetherInstance {
                p0_width: [p0.x, p0.y, p0.z, 0.0011],
                p1_alpha: [p1.x, p1.y, p1.z, 0.42 * alpha],
                color_hdr: [0.25, 0.82, 1.10, 0.14],
            });
        }
    }

    // Subtle zero-overlap radial micro-labels for all beads, kept clear of ambient label budget.
    let center = wrist_ring.beads[0].world_pos;
    let right = camera.cam_right.normalize_or(Vec3::X);
    let up = camera.cam_up.normalize_or(Vec3::Y);

    for bead in &wrist_ring.beads {
        let is_active = bead.selected || bead.highlighted;
        let radial_2d = if bead.id == 0 {
            -up
        } else {
            let delta = bead.world_pos - center;
            let dx = delta.dot(right);
            let dy = delta.dot(up);
            let mag = (dx * dx + dy * dy).sqrt();
            if mag > 1e-4 {
                (right * dx + up * dy) / mag
            } else {
                up
            }
        };
        let label_offset = radial_2d * if bead.id == 0 { 0.015 } else { 0.0165 }
            + right * (radial_2d.dot(right) * 0.006);
        push_world_pearl_label(
            atlas,
            camera,
            bead.world_pos,
            &bead.label,
            label_offset,
            PearlLabelStyle {
                color_hdr_rgb: bead.color_hdr,
                is_active,
                alpha,
            },
            typo,
        );
    }
}

fn append_journey_tethers_and_labels(
    prompt: Option<&PeripheralPromptVisual>,
    recorder: &JourneyRecorder,
    tree: &cosmos_math::SectorTree,
    cam_pose: &cosmos_math::ScalePose64,
    atlas: &MsdfAtlas,
    camera: &RenderCamera,
    typo: &mut TypoFrameOutput,
) {
    if let Some(p) = prompt
        && p.alpha > 0.05
    {
        // Guide tether linking FOLLOW <-> EXIT and ghost rail checkpoint when visible in front of eye.
        typo.tethers.push(TetherInstance {
            p0_width: [p.follow_world.x, p.follow_world.y, p.follow_world.z, 0.0018],
            p1_alpha: [p.exit_world.x, p.exit_world.y, p.exit_world.z, 0.55 * p.alpha],
            color_hdr: [0.35, 1.05, 1.35, 0.25],
        });
        if p.ghost_world.z < -0.15 && p.ghost_world.length() < 40.0 {
            typo.tethers.push(TetherInstance {
                p0_width: [p.follow_world.x, p.follow_world.y, p.follow_world.z, 0.0016],
                p1_alpha: [
                    p.ghost_world.x,
                    p.ghost_world.y,
                    p.ghost_world.z,
                    0.42 * p.alpha,
                ],
                color_hdr: [0.40, 1.35, 1.15, 0.22],
            });
        }
        let up = camera.cam_up.normalize_or(Vec3::Y);
        let right = camera.cam_right.normalize_or(Vec3::X);
        push_world_pearl_label(
            atlas,
            camera,
            p.follow_world,
            "FOLLOW",
            up * 0.042 - right * 0.015,
            PearlLabelStyle {
                color_hdr_rgb: [0.45, 1.85, 1.45],
                is_active: p.hovered == Some(PeripheralChoice::Follow),
                alpha: p.alpha,
            },
            typo,
        );
        push_world_pearl_label(
            atlas,
            camera,
            p.exit_world,
            "EXIT",
            -up * 0.042 + right * 0.015,
            PearlLabelStyle {
                color_hdr_rgb: [1.95, 0.85, 0.55],
                is_active: p.hovered == Some(PeripheralChoice::Exit),
                alpha: p.alpha,
            },
            typo,
        );
    }

    // Project recorded waypoint trail tethers cleanly into camera space.
    if recorder.waypoint_count() >= 2 {
        let mut prev_pos: Option<Vec3> = None;
        for wp in recorder.waypoints() {
            if let Ok(rel) = wp.pose.project_entity(tree, cam_pose, 0.1) {
                let p = rel.pos;
                if p.is_finite() && p.length() < 200.0 {
                    if let Some(prev) = prev_pos {
                        typo.tethers.push(TetherInstance {
                            p0_width: [prev.x, prev.y, prev.z, 0.0022],
                            p1_alpha: [p.x, p.y, p.z, 0.48],
                            color_hdr: [1.35, 0.95, 1.85, 0.20],
                        });
                    }
                    prev_pos = Some(p);
                }
            }
        }
    }
}

fn build_focus_candidates_filtered(
    visible: &[VisibleEntity],
    cam_orient: glam::Quat,
    sector_exp_pref: Option<u8>,
    required_exp: Option<u8>,
    observer: &cosmos_math::ScalePose64,
    active_experience: u8,
) -> Vec<FocusCandidate> {
    let inv_orient = cam_orient.conjugate();
    let max_take = if sector_exp_pref.is_some() { 256 } else { 96 };
    let build = |filter_exp: Option<u8>| -> Vec<FocusCandidate> {
        visible
            .iter()
            .filter(|v| filter_exp.is_none_or(|e| v.experience_id == e))
            .filter(|v| cosmos_render::universe_scene::solar_observer_allows_entity(observer, active_experience, v))
            .take(max_take)
            .filter_map(|v| {
                let len = v.pose.pos.length();
                let physical = cosmos_render::universe_scene::environment_is_physical(observer, active_experience);
                if !len.is_finite() || len <= if physical { 0.0 } else { 1e-4 } {
                    return None;
                }
                if matches!(sector_exp_pref, Some(1 | 2)) && Some(v.experience_id) != sector_exp_pref
                {
                    return None;
                }
                let tier_w = match v.kind {
                    EntityKind::FeatureNode
                    | EntityKind::ChromaticSample
                    | EntityKind::MusicalFilament => 1.35,
                    EntityKind::ZoneShell
                    | EntityKind::CelestialBody
                    | EntityKind::GalacticLandmark => 1.25,
                    EntityKind::ExperienceAnchor => 1.05,
                    EntityKind::SyntheticCluster => 0.85,
                };
                let pref_mul = if Some(v.experience_id) == sector_exp_pref {
                    2.8
                } else {
                    1.0
                };
                Some(FocusCandidate {
                    entity_id: v.id,
                    dir_cam: inv_orient * (v.pose.pos / len),
                    // The index stores full approximate diameters. A physical
                    // sphere's true tangent cone must not select empty sky
                    // beyond its limb; retain legacy targeting for other scenes.
                    angular_radius_deg: physical_body_angular_radius(v, physical, active_experience)
                        .unwrap_or(v.pose.angular_rad).to_degrees(),
                    salience_tier: tier_w,
                    visibility_weight: (v.salience_score * pref_mul).clamp(0.1, 8.0),
                    experience_id: v.experience_id,
                    zone_index: v.zone_index,
                    feature_index: v.feature_index,
                })
            })
            .collect()
    };
    if let Some(req) = required_exp {
        let filtered = build(Some(req));
        if !filtered.is_empty() {
            return filtered;
        }
    }
    build(None)
}

#[allow(clippy::too_many_arguments)]
fn append_diegetic_sprites(
    wrist_ring: &WristResonanceRing,
    scale_meridian: &ScaleMeridianArc,
    peripheral_prompt: Option<&PeripheralPromptVisual>,
    recorder: &JourneyRecorder,
    tree: &cosmos_math::SectorTree,
    cam_pose: &cosmos_math::ScalePose64,
    camera: &RenderCamera,
    _time_sec: f32,
    sprites: &mut Vec<SpriteInstance>,
) {
    if wrist_ring.summon_mode != SummonMode::Hidden || wrist_ring.open_alpha > 0.02 {
        let alpha = wrist_ring.open_alpha.clamp(0.0, 1.0);
        let denom = wrist_ring.beads.len().max(1) as f32;
        for (i, bead) in wrist_ring.beads.iter().enumerate() {
            let gain = if bead.selected || bead.highlighted {
                1.45
            } else {
                0.82
            };
            let sub_param = if bead.selected || bead.highlighted {
                1.0
            } else {
                0.0
            };
            sprites.push(SpriteInstance {
                pos_size: [
                    bead.world_pos.x,
                    bead.world_pos.y,
                    bead.world_pos.z,
                    (bead.radius * 0.44).max(0.0042),
                ],
                axis_aspect: [0.0, 1.0, 0.0, 1.0],
                color_alpha: [
                    bead.color_hdr[0] * gain,
                    bead.color_hdr[1] * gain,
                    bead.color_hdr[2] * gain,
                    0.96 * alpha,
                ],
                tags_f: [4.0, (i as f32) / denom, sub_param, 0.0],
            });
        }
    }

    if let Some(p) = peripheral_prompt
        && p.alpha > 0.02
    {
        let follow_gain = if p.hovered == Some(PeripheralChoice::Follow) {
            1.45
        } else {
            0.95
        };
        let exit_gain = if p.hovered == Some(PeripheralChoice::Exit) {
            1.45
        } else {
            0.95
        };
        sprites.push(SpriteInstance {
            pos_size: [
                p.follow_world.x,
                p.follow_world.y,
                p.follow_world.z,
                (p.radius_m * 0.46).max(0.011),
            ],
            axis_aspect: [0.0, 1.0, 0.0, 1.0],
            color_alpha: [
                0.45 * follow_gain,
                1.85 * follow_gain,
                1.45 * follow_gain,
                0.96 * p.alpha,
            ],
            tags_f: [
                4.0,
                0.25,
                if p.hovered == Some(PeripheralChoice::Follow) {
                    1.0
                } else {
                    0.0
                },
                0.0,
            ],
        });
        sprites.push(SpriteInstance {
            pos_size: [
                p.exit_world.x,
                p.exit_world.y,
                p.exit_world.z,
                (p.radius_m * 0.46).max(0.011),
            ],
            axis_aspect: [0.0, 1.0, 0.0, 1.0],
            color_alpha: [
                1.95 * exit_gain,
                0.85 * exit_gain,
                0.55 * exit_gain,
                0.96 * p.alpha,
            ],
            tags_f: [
                4.0,
                0.85,
                if p.hovered == Some(PeripheralChoice::Exit) {
                    1.0
                } else {
                    0.0
                },
                0.0,
            ],
        });
    }

    for wp in recorder.waypoints() {
        if let Ok(rel) = wp.pose.project_entity(tree, cam_pose, 0.1)
            && rel.pos.is_finite()
            && rel.pos.length() < 120.0
        {
            sprites.push(SpriteInstance {
                pos_size: [rel.pos.x, rel.pos.y, rel.pos.z, 0.018],
                axis_aspect: [0.0, 1.0, 0.0, 1.0],
                color_alpha: [1.45, 0.95, 2.05, 0.88],
                tags_f: [4.0, 0.65, 0.0, 0.0],
            });
        }
    }

    if scale_meridian.visible && scale_meridian.opacity > 0.02 {
        let orient = scale_meridian_basis_quat(camera);
        for tick in scale_meridian.sample_ticks(Vec3::ZERO, orient, 1.45) {
            sprites.push(SpriteInstance {
                pos_size: [
                    tick.world_pos.x,
                    tick.world_pos.y,
                    tick.world_pos.z,
                    if tick.is_major { 0.016 } else { 0.009 },
                ],
                axis_aspect: [0.0, 1.0, 0.0, 1.0],
                color_alpha: [0.55, 1.15, 1.45, tick.alpha],
                tags_f: [4.0, tick.sub_param, 1.0, 0.0],
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmos_math::{ScalePose64, DEFAULT_DESKTOP_NEAR_RENDER_UNITS};
    use cosmos_render::RenderCamera;
    use glam::DQuat;

    fn test_runtime() -> UniverseRuntime {
        let assets = UniverseAssets::load().expect("load runtime assets");
        UniverseRuntime::new(assets, AudioOutputMode::Headless).expect("init runtime")
    }

    fn guide_test_step(
        rt: &mut UniverseRuntime,
        action: Option<GuideAction>,
        drag: bool,
    ) -> FramePacket {
        let cam = RenderCamera::mono(
            rt.flight.pose.orient.as_quat(),
            72.0_f32.to_radians(),
            16.0 / 9.0,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );
        let ray = action
            .map(|action| {
                let frame = rt.guide_frame(&cam);
                let b = frame
                    .buttons
                    .iter()
                    .find(|b| b.action == action)
                    .expect("visible action");
                Vec3::new(b.rect.az_center.tan(), b.rect.el_center.tan(), -1.0).normalize()
            })
            .unwrap_or(Vec3::NEG_Z);
        rt.step_desktop_frame(
            &DesktopFlightInput {
                left_drag_active: drag,
                mouse_delta_rad: if drag {
                    glam::DVec2::new(0.4, 0.2)
                } else {
                    glam::DVec2::ZERO
                },
                ..Default::default()
            },
            ray,
            false,
            false,
            action.is_some(),
            &cam,
            0.05,
            1280,
            720,
        )
    }

    #[test]
    fn guide_tour_pause_resume_help_skip_and_explore_preserve_intent() {
        let mut rt = test_runtime();
        rt.enable_spatial_guide(false);
        assert_eq!(rt.guide_page, GuidePage::Welcome);
        guide_test_step(&mut rt, Some(GuideAction::Highlights), true);
        assert!(rt.is_journey_active());
        assert!(
            rt.journey_director
                .as_ref()
                .unwrap()
                .override_state()
                .is_zero(),
            "click must not orbit"
        );
        for _ in 0..100 {
            guide_test_step(&mut rt, None, false);
        }
        assert!(
            rt.active_session.is_some(),
            "tour must start real narration"
        );
        guide_test_step(&mut rt, Some(GuideAction::ToggleTour), false);
        let frozen_pose = rt.flight.pose;
        let frozen_clock = rt.audio.playback_clock();
        let beat = rt.journey_director.as_ref().unwrap().active_beat_index();
        let segment = rt.active_session.as_ref().unwrap().seg_idx;
        for _ in 0..120 {
            guide_test_step(&mut rt, None, false);
        }
        assert_eq!(rt.flight.pose, frozen_pose);
        assert_eq!(rt.audio.playback_clock().source_ms, frozen_clock.source_ms);
        assert_eq!(rt.active_session.as_ref().unwrap().seg_idx, segment);
        assert_eq!(
            rt.journey_director.as_ref().unwrap().active_beat_index(),
            beat
        );
        guide_test_step(&mut rt, Some(GuideAction::ToggleTour), false);
        guide_test_step(&mut rt, None, false);
        assert!(rt.audio.playback_clock().source_ms > frozen_clock.source_ms);
        assert_eq!(
            rt.journey_director.as_ref().unwrap().play_state(),
            JourneyPlayState::Playing
        );
        assert_eq!(
            rt.flight.pose, frozen_pose,
            "resuming a static orbital observation must not add camera drift"
        );
        rt.toggle_spatial_help();
        assert_eq!(rt.guide_page, GuidePage::Help);
        assert_eq!(
            rt.journey_director.as_ref().unwrap().play_state(),
            JourneyPlayState::Paused
        );
        guide_test_step(&mut rt, Some(GuideAction::CloseHelp), false);
        assert!(
            rt.narration_paused,
            "closing help must not restart motion or speech"
        );
        guide_test_step(&mut rt, Some(GuideAction::Next), false);
        assert_eq!(
            rt.journey_director.as_ref().unwrap().active_beat_index(),
            beat + 1
        );
        assert_eq!(
            rt.journey_director.as_ref().unwrap().play_state(),
            JourneyPlayState::Paused
        );
        assert!(
            rt.active_session.is_none(),
            "skip must discard the previous clip"
        );
        guide_test_step(&mut rt, Some(GuideAction::Previous), false);
        assert_eq!(
            rt.journey_director.as_ref().unwrap().active_beat_index(),
            beat
        );
        guide_test_step(&mut rt, Some(GuideAction::ToggleTour), false);
        rt.pause_for_focus_loss();
        assert_eq!(
            rt.journey_director.as_ref().unwrap().play_state(),
            JourneyPlayState::Paused
        );
        let before_exit = rt.flight.pose;
        guide_test_step(&mut rt, Some(GuideAction::Explore), true);
        assert!(!rt.is_journey_active());
        assert_eq!(
            rt.flight.pose, before_exit,
            "Explore click must not drag the camera"
        );
        assert!(!rt.narration_paused);
        assert_eq!(rt.guide_page, GuidePage::Hidden);
    }

    #[test]
    fn tour_navigation_releases_music_notes() {
        let mut rt = test_runtime();
        let mut pcm = vec![0.0; 4096];
        for transition in 0..6 {
            rt.activate_journey("cosmic_highlights_tour", true).unwrap();
            rt.start_narration_from_focus(
                TargetIdentity {
                    entity_id: 0x1234,
                    experience_id: 2,
                    zone_index: u16::MAX,
                    feature_index: 69,
                },
                TargetFocus::MusicFilament {
                    midi: 69,
                    timbre_idx: 2,
                },
                Vec3::NEG_Z * 2.0,
                None,
            );
            assert!(
                rt.active_session
                    .as_ref()
                    .unwrap()
                    .active_music_note_id
                    .is_some()
            );
            for _ in 0..8 {
                rt.audio.render_headless_frames(&mut pcm);
            }
            let sustained_energy = rt.audio.reactive_uniforms().synth_b[2];
            assert!(
                sustained_energy > 0.01,
                "music stop must sustain a real synth note"
            );
            match transition {
                0 => rt.journey_next_beat(),
                1 => rt.journey_prev_beat(),
                2 => rt.journey_next_chapter(),
                3 => rt.journey_prev_chapter(),
                4 => rt.activate_journey("grand_cosmic_odyssey", true).unwrap(),
                _ => rt.journey_confirm_exit(),
            }
            assert!(rt.active_session.is_none());
            for _ in 0..40 {
                rt.audio.render_headless_frames(&mut pcm);
            }
            assert!(
                rt.audio.reactive_uniforms().synth_b[2] < sustained_energy * 0.01,
                "transition {transition} must release the old music note"
            );
        }
    }

    #[test]
    fn earth_orbit_ownership_survives_every_authored_stop_and_head_turn() {
        let mut rt = test_runtime();
        rt.activate_journey("cosmic_highlights_tour", true).unwrap();
        let beats = rt.journey_director.as_ref().unwrap().manifest().chapters[0].beats.clone();
        assert_eq!(beats.len(), 6);
        let lean = Vec3::new(0.1, 0.0, 0.0);
        rt.update_vr_tracking(Vec3::ZERO, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        rt.update_vr_tracking(lean, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        for (index, beat) in beats.iter().enumerate() {
            let pose = beat.pose.to_pose64(&beat.beat_id).unwrap();
            rt.journey_director.as_mut().unwrap().start(&rt.spatial.tree, &pose, index);
            rt.flight.pose = pose;
            rt.active_session = None;
            for turn in [0.0, std::f32::consts::PI] {
                let orientation = pose.orient.as_quat() * glam::Quat::from_rotation_y(turn);
                let mut camera = RenderCamera::mono(orientation, 72.0_f32.to_radians(), 1.0, 0.002);
                camera.is_stereo = true;
                let packet = rt.step_common(&camera, &WristSummonInput::default(), Vec3::NEG_Z,
                    0.0, Some(&pose.orient.as_quat()), 1.0 / 120.0, 384, 384, None);
                assert_eq!(packet.active_experience_id, 3, "{} turn {turn}", beat.beat_id);
                let earth = packet.scene.heroes_surface.iter().find(|hero| hero.kind_tags[0] == 0)
                    .expect("the same physical Earth remains in the scene when looking away");
                assert!((earth.pos_radius[3] * 0.84 - 6_371_000.0).abs() < 5.0);
                let expected_center = (-pose.local_pos * 10.0_f64.powf(6.8)).as_vec3() - lean;
                assert!(Vec3::from_slice(&earth.pos_radius).distance(expected_center) < 32.0,
                    "{} must retain physical Earth coordinates", beat.beat_id);
            }
        }
        // Explicit verification has higher priority than the active route.
        assert_eq!(rt.active_experience_for_frame(&[], Some(18)), 18);
        let next_beat = rt.journey_director.as_ref().unwrap().manifest().chapters[1].beats[0].clone();
        let from_earth = rt.flight.pose;
        rt.journey_director.as_mut().unwrap().start(&rt.spatial.tree, &from_earth, 6);
        assert_eq!(rt.active_experience_for_frame(&[], None), 3,
            "selecting the next chapter must not shrink Earth before navigation begins");
        rt.flight.pose = next_beat.pose.to_pose64(&next_beat.beat_id).unwrap();
        assert_eq!(rt.active_experience_for_frame(&[], None), 4,
            "completed navigation to the Solar System releases Earth ownership");
        rt.journey_confirm_exit();
        rt.flight.pose = beats[3].pose.to_pose64(&beats[3].beat_id).unwrap();
        assert_eq!(rt.active_experience_for_frame(&[], None), 3,
            "free orbital exploration also survives an empty head-view query");
        // Navigating into a different owning sector must release Earth ownership.
        rt.flight.pose = ScalePose64::new(SectorId::CULTURE_WEB, DVec3::new(0.0, 0.0, 3.0),
            6.2, DQuat::IDENTITY);
        assert_eq!(rt.active_experience_for_frame(&[], None), 18);
    }

    #[test]
    fn orbital_head_translation_stays_in_real_meters_across_zoom_levels() {
        let mut rt = test_runtime();
        let lean = Vec3::new(0.1, 0.2, 0.3);
        rt.update_vr_tracking(Vec3::ZERO, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        rt.update_vr_tracking(lean, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        for scale in [5.65, 6.6, 7.3] {
            rt.flight.pose = cosmos_math::ScalePose64::new(
                SectorId::EARTH_ORBIT, DVec3::new(0.0, 0.0, 3.0), scale, glam::DQuat::IDENTITY,
            );
            let tracked = rt.tracked_render_pose(true);
            let displacement_meters = (tracked.local_pos - rt.flight.pose.local_pos) * 10.0_f64.powf(6.8);
            assert!(displacement_meters.distance(lean.as_dvec3()) < 1e-7,
                "head motion must remain physical meters at observer scale {scale}");
        }
        let forced_sibling_policy = cosmos_render::universe_scene::earth_environment_is_physical(&rt.flight.pose, 18);
        assert!(!forced_sibling_policy);
        let tracked = rt.tracked_render_pose(forced_sibling_policy);
        let mut expected = rt.flight.pose;
        expected.translate_render_units(lean.as_dvec3(), &rt.spatial.tree).unwrap();
        assert!(tracked.local_pos.distance(expected.local_pos) < 1e-12,
            "forced experiences must use the same explicit unit policy for tracking and scene assembly");
        for scale in [6.5, 8.8, 11.2] {
            rt.flight.pose = ScalePose64::new(SectorId::SOLAR_SYSTEM,
                DVec3::new(0.0, 0.0, 0.006), scale, DQuat::IDENTITY);
            let physical = cosmos_render::universe_scene::environment_is_physical(&rt.flight.pose, 4);
            assert!(physical);
            let tracked = rt.tracked_render_pose(physical);
            let meters = (tracked.local_pos - rt.flight.pose.local_pos)
                * 10.0_f64.powf(cosmos_spatial::SOLAR_SECTOR_SCALE_EXP);
            assert!(meters.distance(lean.as_dvec3()) < 1e-5,
                "a tracked head movement near the Sun remains measured in meters");
        }
    }

    #[test]
    fn solar_highlights_preserve_physical_owner_and_visuals_without_audio() {
        let mut rt = test_runtime();
        rt.activate_journey("cosmic_highlights_tour", true).unwrap();
        let chapter = rt.journey_director.as_ref().unwrap().manifest().chapters[1].clone();
        let solar: Vec<_> = chapter.beats.iter().enumerate().filter(|(_, beat)| {
            matches!(beat.target, BeatTargetSelector::CatalogZone { experience_id: 4, zone_index: 0, .. })
        }).collect();
        assert_eq!(solar.len(), 6);
        for (index, beat) in solar {
            let pose = beat.pose.to_pose64(&beat.beat_id).unwrap();
            rt.journey_director.as_mut().unwrap().start(&rt.spatial.tree, &pose, index + 6);
            rt.flight.pose = pose;
            rt.active_session = None;
            for turn in [0.0, std::f32::consts::PI] {
                let rotation = pose.orient.as_quat() * glam::Quat::from_rotation_y(turn);
                let mut camera = RenderCamera::mono(rotation, 72.0_f32.to_radians(), 1.0, 0.002);
                camera.is_stereo = true;
                let packet = rt.step_common(&camera, &WristSummonInput::default(), Vec3::NEG_Z,
                    0.0, Some(&pose.orient.as_quat()), 1.0 / 120.0, 384, 384, None);
                assert_eq!(packet.active_experience_id, 4, "{} must remain a Solar observation", beat.beat_id);
                let sun = packet.scene.heroes_surface.iter().find(|hero| hero.kind_tags[1] == 1400).unwrap();
                assert!((sun.pos_radius[3] * 0.84 - cosmos_spatial::SUN_RADIUS_M as f32).abs() < 256.0);
                let expected = if let BeatTargetSelector::CatalogZone { feature_index, .. } = beat.target {
                    u32::from(feature_index.unwrap_or(u16::MAX)) & 0xff
                } else { unreachable!() };
                assert_eq!((sun.kind_tags[2] >> 16) & 0xff, expected,
                    "Sun visuals are authored by the observation, not the narration sentence");
                assert_eq!(packet.scene.particle_draw_count, 0);
            }
        }
        assert_eq!(rt.active_experience_for_frame(&[], Some(18)), 18);
        rt.journey_confirm_exit();
        assert_eq!(rt.active_experience_for_frame(&[], None), 4,
            "free exploration keeps the same Sun even after the authored tour ends");
        let sun = rt.spatial.iter_entities().find(|entity| entity.experience_id == 4
            && entity.kind == EntityKind::CelestialBody && entity.zone_index == 0).unwrap();
        let pose = rt.spatial.navigation_pose_for_entity(sun, DVec3::Z).unwrap().0;
        rt.flight.pose = pose;
        let camera = RenderCamera::mono(pose.orient.as_quat(), 72.0_f32.to_radians(), 1.0, 0.02);
        let visible = rt.spatial.query_visible(&pose, camera.cyclopean_view_projection(), 0.001, None).unwrap().visible;
        let hover = FsmOutput {
            state: cosmos_narration::FsmState::Settling { elapsed_ms: 100.0,
                target: TargetIdentity { entity_id: sun.id, experience_id: 4, zone_index: 0, feature_index: 0 } },
            dwell_progress: 0.0, smoothed_ray_dir_cam: Vec3::NEG_Z, ray_angular_deg_per_sec: 0.0,
            start_narration_for: None, should_cancel_audio: false, cancel_fade_remaining_ms: 0.0,
            cancel_fade_alpha: 0.0,
        };
        rt.active_session = None;
        let doc = rt.build_documentary_focus_state(4, &hover, &visible, &RenderAudioUniforms::default(), None);
        assert_eq!(doc.active_zone_index, 0);
        assert_eq!(doc.active_feature_index, u16::MAX,
            "hovering the Sun itself cannot silently request an interior cutaway");
    }

    #[test]
    fn solar_navigation_focus_uses_real_depth_after_far_shell_projection() {
        let mut index = cosmos_spatial::build_default_universe_index();
        let sun_id = 999_411;
        index.insert_entity(cosmos_spatial::SpatialEntity {
            id: sun_id, experience_id: 4, zone_index: 0, feature_index: 0,
            sector_id: SectorId::SOLAR_SYSTEM, local_pos: DVec3::ZERO,
            orient: DQuat::IDENTITY, radius_m: cosmos_spatial::SUN_RADIUS_M,
            scale_exp: cosmos_spatial::SUN_RADIUS_M.log10(), kind: EntityKind::CelestialBody,
            salience_weight: 1.0,
        }).unwrap();
        let expected_distance_m = 3.0 * cosmos_spatial::SUN_RADIUS_M;
        let pose = ScalePose64::new(SectorId::SOLAR_SYSTEM,
            DVec3::Z * expected_distance_m * 10.0_f64.powf(-cosmos_spatial::SOLAR_SECTOR_SCALE_EXP),
            0.0, DQuat::IDENTITY);
        let camera = RenderCamera::mono(glam::Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);
        let mut visible = index.query_visible(&pose,
            cosmos_render::universe_scene::spatial_query_view_projection(&camera, &pose, true),
            0.001, None).unwrap().visible;
        let proxy = visible.iter().find(|v| v.id == sun_id).unwrap();
        assert!(proxy.pose.is_far_shell, "fixture must exercise the index's distant proxy");
        assert!((proxy.pose.pos.length() - cosmos_math::FAR_SHELL_PROXY_RADIUS).abs() < 0.001);
        let angular_radius = physical_body_angular_radius(proxy, true, 4).unwrap();
        cosmos_render::universe_scene::restore_physical_body_query_positions(&index, &pose, 18, false, &mut visible);
        assert!(visible.iter().find(|v| v.id == sun_id).unwrap().pose.is_far_shell,
            "an explicit nonphysical experience override keeps normalized query semantics");
        cosmos_render::universe_scene::restore_physical_body_query_positions(&index, &pose, 4, true, &mut visible);
        let sun = pick_best_entity_along_ray(&visible, Vec3::NEG_Z, 6.0, &pose, 4).unwrap();
        assert_eq!(sun.id, sun_id);
        assert!(!sun.pose.is_far_shell);
        assert!((physical_body_angular_radius(sun, true, 4).unwrap() - angular_radius).abs() < 1e-6);
        let focus = FocusTarget::from_visible(sun, None);
        assert!((focus.focus_point_cam.length() - expected_distance_m).abs() < 256.0,
            "focal scroll and orbit need the actual Sun center, not a 5km proxy");
        assert!((focus.bounding_radius_render_units - cosmos_spatial::SUN_RADIUS_M).abs() < 64.0);
        assert!((focus.distance_render_units - (expected_distance_m - cosmos_spatial::SUN_RADIUS_M * 0.25)).abs() < 256.0,
            "magnetic throttle must use the same physical depth as navigation");
    }

    #[test]
    fn solar_free_roam_ray_focus_matches_the_visible_body_in_desktop_and_vr() {
        fn step(rt: &mut UniverseRuntime, vr: bool) -> FramePacket {
            let camera = RenderCamera::mono(rt.flight.pose.orient.as_quat(),
                72.0_f32.to_radians(), 1.0, 0.02);
            if vr {
                rt.step_vr_frame(&VrFlightInput::default(), &WristSummonInput::default(),
                    &camera, Vec3::NEG_Z, 0.0, 0.05, 384, 384)
            } else {
                rt.step_desktop_frame(&DesktopFlightInput::default(), Vec3::NEG_Z,
                    false, false, false, &camera, 0.05, 384, 384)
            }
        }
        let mut assets = UniverseAssets::load().expect("load runtime assets");
        // This regression exercises real picking, narration selection and scene
        // assembly. Keep their headless audio clock, but leave HRTF resampling
        // and convolution to the dedicated audio tests instead of paying their
        // full debug initialization and DSP cost in every navigation scenario.
        assets.hrir_path = PathBuf::new();
        let mut rt = UniverseRuntime::new(assets, AudioOutputMode::Headless).unwrap();
        for (vr, scale) in [(false, 0.0), (true, 0.0), (false, 4.0), (true, 4.0),
            (false, 8.8), (true, 8.8), (false, 13.5), (true, 13.5)] {
            rt.journey_confirm_exit();
            rt.fsm = RestNarrationFsm::new(FsmConfig::default());
            rt.active_session = None;
            rt.flight.pose = ScalePose64::new(SectorId::SOLAR_SYSTEM,
                DVec3::Z*3.0*cosmos_spatial::SUN_RADIUS_M*10.0_f64.powf(-cosmos_spatial::SOLAR_SECTOR_SCALE_EXP),
                scale, DQuat::IDENTITY);
            let sun_id = rt.spatial.find_zone_entity(4, 0).unwrap().id;
            let mut focused = false;
            for _ in 0..30 {
                let packet = step(&mut rt, vr);
                assert_eq!(packet.active_experience_id, 4);
                if let Some(id) = packet.hovered_entity_id {
                    assert_eq!(id, sun_id, "invisible selectors cannot intercept a ray aimed at the Sun");
                    focused = true;
                }
                let sun = packet.scene.heroes_surface.iter().find(|h| h.kind_tags[1] == 1400).unwrap();
                assert_eq!((sun.kind_tags[2]>>16)&0xff, 255,
                    "resting on the whole Sun cannot request its core cutaway");
            }
            assert!(focused, "the rendered Sun remains an implicit narration target");
            assert_eq!(rt.active_session.as_ref().unwrap().plan.feature_index, None);

            // The Sun is partly in view, but the gaze ray lies outside its
            // 19.47-degree tangent cone at three radii. A full diameter used as
            // the focus radius incorrectly selects the Sun through this sky.
            rt.flight.pose.orient = DQuat::from_rotation_y(30.0_f64.to_radians());
            rt.fsm = RestNarrationFsm::new(FsmConfig::default());
            rt.active_session = None;
            for _ in 0..30 {
                let packet = step(&mut rt, vr);
                assert!(packet.hovered_entity_id.is_none(), "empty sky beside the limb cannot start narration");
            }

            // Look directly at a real indexed feature that is deliberately not
            // rendered in this physical observation. It must not catch a ray.
            let hidden = rt.spatial.find_feature_entity(4, 0, 0).unwrap().local_pos;
            let direction = (hidden-rt.flight.pose.local_pos).normalize();
            rt.flight.pose.orient = DQuat::from_rotation_arc(DVec3::NEG_Z, direction);
            rt.fsm = RestNarrationFsm::new(FsmConfig::default());
            rt.active_session = None;
            for _ in 0..30 {
                let packet = step(&mut rt, vr);
                assert_eq!(packet.active_experience_id, 4);
                assert!(packet.hovered_entity_id.is_none(), "an invisible catalog feature cannot start narration");
            }
            assert!(rt.active_session.is_none());

            // Explicit target selection bypasses implicit visibility filtering.
            let camera = RenderCamera::mono(rt.flight.pose.orient.as_quat(),
                72.0_f32.to_radians(), 1.0, 0.02);
            rt.settle_and_start_target_narration(&camera, Vec3::NEG_Z, Some(4), Some(0), Some(0));
            assert_eq!(rt.active_session.as_ref().unwrap().plan.feature_index, Some(0));
            rt.settle_and_start_target_narration(&camera, Vec3::NEG_Z, Some(4), Some(0), Some(3));
            assert_eq!(rt.active_session.as_ref().unwrap().plan.feature_index, Some(3));
        }
    }

    #[test]
    fn earth_highlights_visuals_do_not_depend_on_audio_or_arbitrary_catalog_nodes() {
        let mut rt = test_runtime();
        let camera = RenderCamera::mono(glam::Quat::IDENTITY, 72.0_f32.to_radians(), 1.0, 0.02);
        let visible = rt.spatial.query_visible(&rt.flight.pose, camera.cyclopean_view_projection(), 0.001, None).unwrap().visible;
        assert!(visible.iter().any(|v| v.kind == EntityKind::FeatureNode));
        let idle = FsmOutput {
            state: cosmos_narration::FsmState::Moving,
            dwell_progress: 0.0, smoothed_ray_dir_cam: Vec3::NEG_Z,
            ray_angular_deg_per_sec: 0.0, start_narration_for: None,
            should_cancel_audio: false, cancel_fade_remaining_ms: 0.0, cancel_fade_alpha: 0.0,
        };
        let quiet = rt.build_documentary_focus_state(3, &idle, &visible, &RenderAudioUniforms::default(), None);
        assert_eq!((quiet.active_zone_index, quiet.active_feature_index), (0, u16::MAX));
        rt.activate_journey("cosmic_highlights_tour", true).unwrap();
        for (beat, expected) in [(0, (0, u16::MAX)), (3, (0, 0)), (4, (0, 1)), (5, (2, 0))] {
            rt.journey_director.as_mut().unwrap().start(&rt.spatial.tree, &rt.flight.pose, beat);
            rt.active_session = None;
            let state = rt.build_documentary_focus_state(3, &idle, &visible, &RenderAudioUniforms::default(), None);
            assert_eq!((state.active_zone_index, state.active_feature_index), expected,
                "authored Earth beat must survive absent/muted audio");
        }
    }

    #[test]
    fn solar_observation_disclosures_are_room_anchored_without_flat_panels() {
        let mut rt = test_runtime();
        rt.enable_spatial_guide(true);
        rt.guide_page = GuidePage::Hidden;
        rt.update_vr_tracking(Vec3::ZERO, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        rt.activate_journey("cosmic_highlights_tour", true).unwrap();
        let director = rt.journey_director.as_mut().unwrap();
        let (index, pose) = director.manifest().flatten_beats().iter()
            .enumerate().find_map(|(i, (_, _, beat))| (beat.beat_id == "solar_limb_eruption")
                .then(|| (i, beat.pose.to_pose64(&beat.beat_id).unwrap()))).unwrap();
        rt.flight.pose = pose;
        director.start(&rt.spatial.tree, &pose, index);
        let camera = RenderCamera::mono(pose.orient.as_quat(), 72.0_f32.to_radians(), 1.0, 0.002);
        rt.step_vr_frame(&VrFlightInput::default(), &WristSummonInput::default(),
            &camera, Vec3::NEG_Z, 0.0, 0.0, 800, 800);
        assert!(rt.solar_observation_anchor.is_some(),
            "actual EUV journey mode must disclose its observing filter even with the wrist UI closed");
        let mut before = TypoFrameOutput::default();
        rt.append_solar_observation_disclosure(Some(SolarObservationMode::Euv), &mut before);
        assert!(!before.glyphs.is_empty());
        assert!(before.foreground_start.is_none());
        assert!(before.glyphs.iter().all(|g| {
            let position = Vec3::from_array(g.pos_center[..3].try_into().unwrap());
            (2.0..3.0).contains(&position.length())
                && g.ink_params[3] == GlyphInkMode::SpatialAnchor as u8 as f32
        }), "the disclosure must have comfortable real depth and no foreground backplate");
        let lean = Vec3::new(0.20, -0.05, 0.03);
        rt.update_vr_tracking(lean, glam::Quat::from_rotation_y(0.9), glam::Quat::IDENTITY);
        let mut after = TypoFrameOutput::default();
        rt.append_solar_observation_disclosure(Some(SolarObservationMode::Euv), &mut after);
        for (a, b) in before.glyphs.iter().zip(&after.glyphs) {
            let a_pos = Vec3::from_array(a.pos_center[..3].try_into().unwrap());
            let b_pos = Vec3::from_array(b.pos_center[..3].try_into().unwrap());
            assert!((b_pos + lean - a_pos).length() < 1e-5);
            assert_eq!(a.right_vec, b.right_vec, "head turning must not rotate the label");
            assert_eq!(a.up_vec, b.up_vec);
        }
        let mut inactive = TypoFrameOutput::default();
        rt.append_solar_observation_disclosure(None, &mut inactive);
        assert!(inactive.glyphs.is_empty() && rt.solar_observation_anchor.is_none());
        for locale in [Locale::EnUs, Locale::FiFi, Locale::EsEs] {
            rt.locale = locale;
            for mode in [SolarObservationMode::VisibleComposite, SolarObservationMode::VisibleDetail] {
                let text = mode.disclosure(locale);
                let illustrative = match locale {
                    Locale::FiFi => "havainnollistava plasma",
                    Locale::EsEs => "plasma ilustrativo",
                    _ => "illustrative plasma",
                };
                assert!(text.contains(illustrative), "the visible composite must disclose added plasma in {locale:?}");
                let mut label = TypoFrameOutput::default();
                rt.append_solar_observation_disclosure(Some(mode), &mut label);
                assert!(!label.glyphs.is_empty() && label.foreground_start.is_none());
                assert!(label.glyphs.iter().all(|g| g.ink_params[3] == GlyphInkMode::SpatialAnchor as u8 as f32));
            }
            for (beat_id, expected) in [
                ("exp04_z000_sun", SolarObservationMode::VisibleComposite),
                ("solar_energy_interior", SolarObservationMode::Interior),
                ("solar_granulation_horizon", SolarObservationMode::VisibleDetail),
                ("solar_active_region", SolarObservationMode::VisibleDetail),
                ("solar_limb_eruption", SolarObservationMode::Euv),
                ("solar_escaping_wind", SolarObservationMode::Euv),
            ] {
                let director = rt.journey_director.as_mut().unwrap();
                let (index, pose) = director.manifest().flatten_beats().iter()
                    .enumerate().find_map(|(i, (_, _, beat))| (beat.beat_id == beat_id)
                        .then(|| (i, beat.pose.to_pose64(&beat.beat_id).unwrap()))).unwrap();
                rt.flight.pose = pose;
                director.start(&rt.spatial.tree, &pose, index);
                rt.step_vr_frame(&VrFlightInput::default(), &WristSummonInput::default(),
                    &camera, Vec3::NEG_Z, 0.0, 0.0, 800, 800);
                assert_eq!(rt.solar_observation_anchor.unwrap().0, expected,
                    "{beat_id} must disclose its actual observing transform in {locale:?}");
            }
        }
    }

    #[test]
    fn vr_guide_trigger_is_consumed_until_release() {
        let mut rt = test_runtime();
        rt.enable_spatial_guide(true);
        let cam = RenderCamera::mono(
            glam::Quat::IDENTITY,
            90.0_f32.to_radians(),
            1.0,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );
        let frame = rt.guide_frame(&cam);
        let b = frame
            .buttons
            .iter()
            .find(|b| b.action == GuideAction::Explore)
            .unwrap();
        let origin = Vec3::new(0.25, -0.15, 0.1);
        let target = frame.hologram.unwrap().button_center(b);
        let mut hologram_text = TypoFrameOutput::default();
        frame.append_typography(&rt.atlas, &cam, &mut hologram_text);
        assert!(
            hologram_text.foreground_start.is_none(),
            "VR must use physical scene depth"
        );
        assert!(
            hologram_text
                .glyphs
                .iter()
                .all(|g| g.ink_params[3] != cosmos_typo::GlyphInkMode::ControlBackdrop as u8 as f32),
            "VR must have no flat backplates"
        );
        assert!(
            hologram_text.tethers.len() > 400,
            "controls must have volumetric strokes"
        );
        rt.update_vr_tracking(Vec3::ZERO, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        let before = rt.step_vr_frame(
            &VrFlightInput::default(),
            &WristSummonInput::default(),
            &cam,
            Vec3::NEG_Z,
            0.0,
            0.016,
            800,
            800,
        );
        let lean = Vec3::new(0.2, 0.0, 0.0);
        rt.update_vr_tracking(lean, glam::Quat::from_rotation_y(0.7), glam::Quat::IDENTITY);
        let after = rt.step_vr_frame(
            &VrFlightInput::default(),
            &WristSummonInput::default(),
            &cam,
            Vec3::NEG_Z,
            0.0,
            0.016,
            800,
            800,
        );
        let hero_center = |packet: &FramePacket| {
            let hero = packet
                .scene
                .heroes_surface
                .iter()
                .max_by(|a, b| a.pos_radius[3].total_cmp(&b.pos_radius[3]))
                .expect("Earth surface");
            Vec3::from_array([hero.pos_radius[0], hero.pos_radius[1], hero.pos_radius[2]])
        };
        assert!(
            (hero_center(&before) - hero_center(&after) - lean).length() < 1e-4,
            "room-scale leaning must change the world view as well as the hologram"
        );
        let moved = rt.guide_frame(&cam);
        let h = moved.hologram.unwrap();
        let moved_button = moved
            .buttons
            .iter()
            .find(|b| b.action == GuideAction::Explore)
            .unwrap();
        assert!(
            (h.button_center(moved_button) + lean - target).length() < 1e-5,
            "head movement must not drag the hologram"
        );
        assert_eq!(
            h.orientation,
            glam::Quat::IDENTITY,
            "head turns must not rotate the hologram"
        );
        rt.update_vr_tracking(Vec3::ZERO, glam::Quat::IDENTITY, glam::Quat::IDENTITY);
        let mut wrist = WristSummonInput {
            pointer_origin: Some(origin),
            pointer_dir: Some((target - origin).normalize()),
            select_triggered: true,
            ..Default::default()
        };
        let mut flight = VrFlightInput {
            trigger_pull: 1.0,
            dominant_hand_forward_cam: DVec3::NEG_Z,
            ..Default::default()
        };
        let initial = rt.flight.pose;
        for _ in 0..8 {
            let packet = rt.step_vr_frame(&flight, &wrist, &cam, Vec3::NEG_Z, 0.0, 0.05, 800, 800);
            assert!(
                packet.typo.foreground_start.is_none(),
                "hidden VR guidance must retain world depth"
            );
            wrist.select_triggered = false;
        }
        assert_eq!(rt.guide_page, GuidePage::Hidden);
        assert_eq!(
            rt.flight.pose, initial,
            "holding the selection trigger must not thrust"
        );
        flight.trigger_pull = 0.0;
        rt.step_vr_frame(&flight, &wrist, &cam, Vec3::NEG_Z, 0.0, 0.05, 800, 800);
        flight.trigger_pull = 1.0;
        rt.step_vr_frame(&flight, &wrist, &cam, Vec3::NEG_Z, 0.0, 0.05, 800, 800);
        assert_ne!(
            rt.flight.pose, initial,
            "a new trigger gesture must still fly"
        );
    }

    #[test]
    fn signature_zone_mapping_covers_apps_05_through_14() {
        let assets = UniverseAssets::load().expect("load assets");
        let cases = [
            (5u8, "galactic-center"),
            (6, "local-group-center"),
            (7, "laniakea-basin"),
            (8, "galactic-filaments"),
            (9, "cmb"),
            (10, "inflation"),
            (11, "observed"),
            (12, "black-holes"),
            (13, "atoms"),
            (14, "planck"),
        ];
        for (exp_id, expected_slug) in cases {
            let z_idx = resolve_signature_zone_index(&assets.catalog, exp_id);
            let zone = &assets.catalog.zones(exp_id)[z_idx as usize];
            assert_eq!(
                assets.catalog.str_slice(zone.id),
                expected_slug,
                "exp {exp_id} resolved unexpected signature zone"
            );
        }
    }

    #[test]
    fn verification_reset_meridian_labels_headings_and_tethers() {
        use cosmos_narration::FsmState;

        let mut rt = test_runtime();
        let cam = RenderCamera::mono(
            glam::Quat::IDENTITY,
            66.0_f32.to_radians(),
            16.0 / 9.0,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );

        // 1. Scale meridian at Earth (02b) emits rotated non-blocked `10^+XX m` labels.
        rt.flight.pose = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            DVec3::new(0.0, 0.25, 2.2),
            6.55,
            DQuat::IDENTITY,
        );
        let meridian_ovr = VerificationNarrationOverride {
            active_experience_id: 3,
            phase_reveal: 0.78,
            show_spanish_ghost_translation: true,
            summon_wrist_ring: false,
            force_playback_rate: None,
            force_scale_vel: Some(2.4),
            ..Default::default()
        };
        let mut meridian_pkt = rt.step_verification_frame(
            &cam,
            Vec3::NEG_Z,
            Some(&meridian_ovr),
            1.0 / 60.0,
            1600,
            900,
        );
        for _ in 0..8 {
            meridian_pkt = rt.step_verification_frame(
                &cam,
                Vec3::NEG_Z,
                Some(&meridian_ovr),
                1.0 / 60.0,
                1600,
                900,
            );
        }
        assert!(rt.scale_meridian.visible);
        assert!(rt.scale_meridian.opacity > 0.5);
        // Verify ambient `10^+` labels survived Earth blocker.
        let mut meridian_labels = Vec::new();
        rt.append_diegetic_msdf_labels(&cam, &mut meridian_labels);
        assert!(
            meridian_labels.len() >= 3,
            "expected meridian scale ticks around Earth, got {}",
            meridian_labels.len()
        );

        // 2. Verification reset completely clears meridian & narration state before Saturn (03).
        rt.reset_verification_state();
        assert!(!rt.scale_meridian.visible);
        assert_eq!(rt.scale_meridian.opacity, 0.0);
        let saturn_pkt = rt.step_verification_frame(&cam, Vec3::NEG_Z, None, 1.0 / 60.0, 1600, 900);
        assert!(!rt.scale_meridian.visible);
        assert!(rt.active_session.is_none());
        assert!(
            saturn_pkt.typo.tethers.iter().all(|t| t.p0_width[3] < 0.0016),
            "narration tether leaked across reset_verification_state"
        );
        let _ = meridian_pkt;

        // 3. Check Apps 05, 06, 08, 13 produce authentic signature zones & zero duplicate headings.
        let checks = [
            (5u8, SectorId::MILKY_WAY, 20.5, DVec3::new(0.0, 0.8, 3.0)),
            (6u8, SectorId::LOCAL_GROUP, 22.0, DVec3::new(0.0, 0.6, 2.8)),
            (8u8, SectorId::COSMIC_WEB, 25.3, DVec3::new(0.0, 0.4, 2.5)),
            (13u8, SectorId::QUANTUM_FIELD, -10.5, DVec3::new(0.0, 0.3, 2.4)),
        ];
        for (exp_id, sector, scale_exp, local_pos) in checks {
            rt.reset_verification_state();
            rt.flight.pose = ScalePose64::new(sector, local_pos, scale_exp, DQuat::IDENTITY);
            let ovr = VerificationNarrationOverride {
                active_experience_id: exp_id,
                phase_reveal: 0.6,
                show_spanish_ghost_translation: false,
                summon_wrist_ring: false,
                force_playback_rate: None,
                force_scale_vel: None,
                ..Default::default()
            };
            rt.settle_and_start_experience_narration(&cam, Vec3::NEG_Z, Some(exp_id));
            let pkt = rt.step_verification_frame(
                &cam,
                Vec3::NEG_Z,
                Some(&ovr),
                1.0 / 60.0,
                1600,
                900,
            );
            let sess = rt.active_session.as_ref().expect("active session");
            let fsm_out = FsmOutput {
                state: FsmState::Narrating {
                    target: sess.target,
                    active_ms: 0.0,
                },
                dwell_progress: 1.0,
                smoothed_ray_dir_cam: Vec3::NEG_Z,
                ray_angular_deg_per_sec: 0.0,
                start_narration_for: None,
                should_cancel_audio: false,
                cancel_fade_remaining_ms: 0.0,
                cancel_fade_alpha: 0.0,
            };
            let spec = rt
                .build_bilingual_spec(&fsm_out, exp_id, Some(&ovr))
                .expect("bilingual spec");
            if let Some(h) = &spec.active_heading {
                assert!(
                    !h.trim().eq_ignore_ascii_case(spec.spanish_text.trim()),
                    "duplicate heading on exp {exp_id}: {h}"
                );
            }
            println!(
                "exp={exp_id} region={} heading={:?} es={} tethers={} wrist_open={:.3}",
                sess.plan.region_id,
                spec.active_heading,
                spec.spanish_text,
                pkt.typo.tethers.len(),
                rt.wrist_ring.open_alpha
            );
            for (ti, t) in pkt.typo.tethers.iter().enumerate() {
                println!(
                    "  tether[{ti}] p0={:?} p1={:?} alpha={:.3}",
                    &t.p0_width[0..3],
                    &t.p1_alpha[0..3],
                    t.p1_alpha[3]
                );
            }
        }
    }

    #[test]
    fn documentary_focus_state_tracks_active_zone_feature_and_telemetry() {
        let mut rt = test_runtime();
        let cam = RenderCamera::mono(
            glam::Quat::IDENTITY,
            72.0_f32.to_radians(),
            16.0 / 9.0,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );

        // App 03 Earth Inner Core (zone 13): explicit target + cutaway telemetry.
        rt.flight.pose = ScalePose64::new(
            SectorId::EARTH_ORBIT,
            DVec3::new(0.95, 0.45, 2.48),
            6.34,
            DQuat::IDENTITY,
        );
        rt.settle_and_start_target_narration(&cam, Vec3::NEG_Z, Some(3), Some(13), None);
        let ovr_earth = VerificationNarrationOverride {
            active_experience_id: 3,
            phase_reveal: 0.65,
            show_spanish_ghost_translation: true,
            summon_wrist_ring: false,
            force_zone_index: Some(13),
            force_earth_cutaway: Some(1.0),
            ..Default::default()
        };
        let vis_earth = rt
            .spatial
            .query_visible(
                &rt.flight.pose,
                cam.eyes[0].view_proj,
                0.05_f32.to_radians(),
                Some(256),
            )
            .map(|q| q.visible)
            .unwrap_or_default();
        let idle_fsm = FsmOutput {
            state: cosmos_narration::FsmState::Locked {
                target: rt.active_session.as_ref().expect("earth session").target,
                elapsed_ms: 0.0,
            },
            dwell_progress: 1.0,
            smoothed_ray_dir_cam: Vec3::NEG_Z,
            ray_angular_deg_per_sec: 0.0,
            start_narration_for: None,
            should_cancel_audio: false,
            cancel_fade_remaining_ms: 0.0,
            cancel_fade_alpha: 0.0,
        };
        let doc_earth = rt.build_documentary_focus_state(
            3,
            &idle_fsm,
            &vis_earth,
            &RenderAudioUniforms::default(),
            Some(&ovr_earth),
        );
        let earth_core = &rt.catalog.zones(3)[13];
        assert_eq!(doc_earth.active_zone_index, 13);
        assert_eq!(doc_earth.active_feature_index, 0);
        assert!((doc_earth.narration_cue_progress - 0.65).abs() < 1e-5);
        assert!((doc_earth.narration_active_weight - 1.0).abs() < 1e-5);
        assert_eq!(
            doc_earth.telemetry,
            [
                earth_core.range_start,
                earth_core.range_end,
                earth_core.phys_low,
                earth_core.phys_high,
            ]
        );

        // App 20 Futures Kardashev (zone 16) macro overview leaves active_feature_index = u16::MAX.
        rt.reset_verification_state();
        rt.flight.pose = ScalePose64::new(
            SectorId::FUTURES_ARCS,
            DVec3::new(-0.92, 1.15, 4.10),
            11.85,
            DQuat::IDENTITY,
        );
        rt.settle_and_start_target_narration(&cam, Vec3::NEG_Z, Some(20), Some(16), None);
        let ovr_futures = VerificationNarrationOverride {
            active_experience_id: 20,
            phase_reveal: 0.68,
            force_zone_index: Some(16),
            ..Default::default()
        };
        let doc_futures = rt.build_documentary_focus_state(
            20,
            &idle_fsm,
            &[],
            &RenderAudioUniforms::default(),
            Some(&ovr_futures),
        );
        assert_eq!(doc_futures.active_zone_index, 16);
        assert_eq!(doc_futures.active_feature_index, u16::MAX);

        // App 23 Perspectives: exact zone + feature lock propagation.
        rt.reset_verification_state();
        rt.flight.pose = ScalePose64::new(
            SectorId::HYPER_LOTUS_23_PERSPECTIVES,
            DVec3::new(-0.55, 2.40, -2.70),
            27.6,
            DQuat::IDENTITY,
        );
        rt.settle_and_start_target_narration(&cam, Vec3::NEG_Z, Some(23), Some(22), Some(2));
        let ovr_persp = VerificationNarrationOverride {
            active_experience_id: 23,
            phase_reveal: 0.72,
            force_zone_index: Some(22),
            force_feature_index: Some(2),
            ..Default::default()
        };
        let doc_persp = rt.build_documentary_focus_state(
            23,
            &idle_fsm,
            &[],
            &RenderAudioUniforms::default(),
            Some(&ovr_persp),
        );
        assert_eq!(doc_persp.active_zone_index, 22);
        assert_eq!(doc_persp.active_feature_index, 2);

        // App 01 Colors & App 02 Music telemetry stay within calibrated ranges even when idle.
        rt.reset_verification_state();
        let doc_color_idle = rt.build_documentary_focus_state(
            1,
            &idle_fsm,
            &[],
            &RenderAudioUniforms::default(),
            None,
        );
        assert!((0.0..=1.0).contains(&doc_color_idle.telemetry[1]));
        assert!((0.0..=1.0).contains(&doc_color_idle.telemetry[2]));

        let mut music_audio = RenderAudioUniforms::default();
        music_audio.synth_b[1] = 2.0;
        music_audio.synth_b[2] = 0.78;
        let doc_music_idle =
            rt.build_documentary_focus_state(2, &idle_fsm, &[], &music_audio, None);
        assert!((doc_music_idle.telemetry[2] - 440.0).abs() < 1e-3);
        assert!((doc_music_idle.telemetry[3] - 0.78).abs() < 1e-5);
    }

    #[test]
    fn meaning_tether_targets_centerpiece_and_saturn_blocker_covers_cassini_rings() {
        let mut rt = test_runtime();
        let cam_id = RenderCamera::mono(
            glam::Quat::IDENTITY,
            72.0_f32.to_radians(),
            16.0 / 9.0,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );

        // App 24 Meaning (`zone 18, feature 0`): card tether anchor resolves to central ExperienceAnchor(24).
        rt.flight.pose = ScalePose64::new(
            SectorId::HYPER_LOTUS_24_MEANING,
            DVec3::new(0.0, 0.0, 3.9),
            27.6,
            DQuat::IDENTITY,
        );
        let vis24 = rt
            .spatial
            .query_visible(
                &rt.flight.pose,
                cam_id.eyes[0].view_proj,
                0.05_f32.to_radians(),
                Some(256),
            )
            .map(|q| q.visible)
            .unwrap_or_default();
        let anchor24 = vis24
            .iter()
            .find(|v| v.experience_id == 24 && v.kind == EntityKind::ExperienceAnchor)
            .expect("ExperienceAnchor(24) visible");
        let ovr24 = VerificationNarrationOverride {
            active_experience_id: 24,
            phase_reveal: 0.55,
            force_zone_index: Some(18),
            force_feature_index: Some(0),
            ..Default::default()
        };
        let resolved = resolve_narration_card_anchor(
            &vis24,
            24,
            Vec3::new(3.5, 2.5, -4.0),
            Some(&ovr24),
        );
        assert!(
            (resolved - anchor24.pose.pos).length() < 1e-4,
            "expected App 24 anchor {:?}, got {:?}",
            anchor24.pose.pos,
            resolved
        );

        // App 04 Saturn (`zone 7`): blocker extends wide enough horizontally to protect Cassini rings.
        rt.reset_verification_state();
        let saturn_entity = rt.spatial.iter_entities().find(|entity| {
            entity.experience_id == 4 && entity.kind == EntityKind::CelestialBody && entity.zone_index == 7
        }).unwrap();
        rt.flight.pose = rt.spatial.navigation_pose_for_entity(saturn_entity, DVec3::new(0.2, 0.35, 1.0)).unwrap().0;
        let cam_sat = RenderCamera::mono(
            rt.flight.pose.orient.as_quat(),
            72.0_f32.to_radians(),
            16.0 / 9.0,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );
        let vis04 = rt
            .spatial
            .query_visible(
                &rt.flight.pose,
                cam_sat.eyes[0].view_proj,
                0.05_f32.to_radians(),
                Some(256),
            )
            .map(|q| q.visible)
            .unwrap_or_default();
        let saturn = vis04
            .iter()
            .find(|v| {
                v.experience_id == 4
                    && v.kind == EntityKind::CelestialBody
                    && v.zone_index == 7
            })
            .expect("Saturn visible from its physical observation pose");
        let blockers = collect_external_blockers(
            &rt.wrist_ring,
            &vis04,
            4,
            Some(saturn.id),
            &cam_sat,
        );
        assert!(
            blockers
                .iter()
                .any(|b| b.half_az > (saturn.pose.rel_scale * 2.25 / 0.84
                    / saturn.pose.distance_render_units as f32).asin()),
            "expected wide Saturn Cassini ring blocker (planet ang={:.3}, blockers={blockers:?})",
            saturn.pose.angular_rad
        );
    }

    #[test]
    fn runtime_journey_preserves_head_turn_spawns_exit_pearls_and_exports_waypoints() {
        let mut rt = test_runtime();
        rt.activate_journey("cosmic_highlights_tour", true)
            .expect("activate highlights");
        assert!(rt.is_journey_active());

        let cam_vr_a = RenderCamera::mono(
            glam::Quat::IDENTITY,
            90.0_f32.to_radians(),
            1.1,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );
        let cam_vr_rotated = RenderCamera::mono(
            glam::Quat::from_rotation_y(85.0_f32.to_radians()),
            90.0_f32.to_radians(),
            1.1,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );
        let wrist_idle = WristSummonInput::default();
        let vr_idle = VrFlightInput::default();

        // Step several frames while turning the HMD head by 85 degrees: journey remains active,
        // manual override stays zero, and navigation pose only tracks the journey rail.
        let _ = rt.step_vr_frame(
            &vr_idle,
            &wrist_idle,
            &cam_vr_a,
            Vec3::NEG_Z,
            0.0,
            0.05,
            800,
            800,
        );
        let _ = rt.step_vr_frame(
            &vr_idle,
            &wrist_idle,
            &cam_vr_rotated,
            Vec3::NEG_Z,
            0.0,
            0.05,
            800,
            800,
        );
        assert!(rt.is_journey_active());
        assert!(!rt.is_journey_paused_by_leash());
        assert!(
            rt.journey_director
                .as_ref()
                .unwrap()
                .override_state()
                .is_zero()
        );

        // Sustained left thumbstick push triggers PausedByLeash and renders FOLLOW/EXIT outside fovea.
        let vr_stick = VrFlightInput {
            left_stick: glam::DVec2::new(1.0, 0.0),
            ..Default::default()
        };
        let mut last_pkt = None;
        for _ in 0..24 {
            last_pkt = Some(rt.step_vr_frame(
                &vr_stick,
                &wrist_idle,
                &cam_vr_a,
                Vec3::NEG_Z,
                0.0,
                0.08,
                800,
                800,
            ));
        }
        assert!(rt.is_journey_paused_by_leash());
        let pkt = last_pkt.unwrap();
        assert!(
            pkt.scene.sprites.len() >= 2,
            "expected FOLLOW and EXIT 3D pearls"
        );
        assert!(pkt.typo.tethers.iter().any(|t| t.p1_alpha[3] > 0.1));

        // Confirm Exit detaches cleanly without pop and saves bookmark.
        rt.journey_confirm_exit();
        assert!(!rt.is_journey_active());
        assert!(rt.journey_director.as_ref().unwrap().bookmark().is_some());

        // Record manual waypoints & trace and export to temporary dir.
        let tmp_dir = std::env::temp_dir().join(format!(
            "hypercosmos_journey_test_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&tmp_dir);
        rt.user_journeys_dir = tmp_dir.clone();
        assert_eq!(rt.record_manual_waypoint(), 1);
        assert!(rt.toggle_live_trace_recording());
        let _ = rt.toggle_live_trace_recording();
        let exported = rt.export_recorded_journey().expect("export user journey");
        assert!(exported.json_path.is_file());
        assert!(exported.share_uri_path.is_file());
        assert!(exported.share_uri.starts_with(cosmos_journey::SHARE_URI_PREFIX));
        let _ = std::fs::remove_dir_all(&tmp_dir);
    }
}
