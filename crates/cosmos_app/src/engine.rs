use std::sync::Arc;

use anyhow::Result;
use cosmos_audio::{AudioEngine, AudioOutputMode};
use cosmos_manifest::{AudioAssetRecord, Locale, UniverseCatalog};
use cosmos_math::{smoothstep, SectorId};
use cosmos_narration::{
    srgb_to_linear_hdr, FocusCandidate, FsmConfig, FsmInput, FsmOutput, FsmStateKind,
    MemoryPlanState, NarrationPlan, NarrationSegment, ResolverConfig, RestNarrationFsm,
    SegmentRole, SharedNarrationMemory, TargetFocus, TargetIdentity, TargetResolver,
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

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum SegmentPhase {
    Spanish,
    Translation,
    Completed,
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
    pub locale: Locale,
    pub time_sec: f32,
    active_session: Option<ActiveNarrationSession>,
    last_zone_sound_key: Option<(u8, u16)>,
    prev_fsm_kind: FsmStateKind,
    prev_hovered_bead: Option<u32>,
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
            locale: Locale::EnUs,
            time_sec: 0.0,
            active_session: None,
            last_zone_sound_key: None,
            prev_fsm_kind: FsmStateKind::Moving,
            prev_hovered_bead: None,
        })
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
        let visible_pre = self
            .spatial
            .query_visible(
                &self.flight.pose,
                camera_for_pick.eyes[0].view_proj,
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

        let cam_orient_pre = self.flight.pose.orient.as_quat();
        let focus_entity_pre =
            pick_best_entity_along_ray(&visible_pre, cam_orient_pre * hover_ray_cam, 6.0);
        let focus_target = focus_entity_pre
            .map(|vis| {
                let pt_cam = cam_orient_pre.conjugate().as_dquat() * vis.pose.pos.as_dvec3();
                FocusTarget::from_visible(vis, Some(pt_cam))
            })
            .or_else(|| Some(FocusTarget::from_ray(hover_ray_cam.as_dvec3(), 3.0, None)));

        let _ = self.flight.step_desktop(
            &self.spatial,
            desktop_input,
            focus_target.as_ref(),
            dt_sec as f64,
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

        self.step_common(
            &next_camera,
            &wrist_input,
            hover_ray_cam,
            0.0,
            None,
            dt_sec,
            width,
            height,
            None,
        )
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
        let visible_pre = self
            .spatial
            .query_visible(
                &self.flight.pose,
                vr_camera.eyes[0].view_proj,
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

        let cam_orient_pre = self.flight.pose.orient.as_quat();
        let focus_entity_pre =
            pick_best_entity_along_ray(&visible_pre, cam_orient_pre * aim_ray_cam, 6.0);
        let focus_target = focus_entity_pre
            .map(|vis| {
                let pt_cam = cam_orient_pre.conjugate().as_dquat() * vis.pose.pos.as_dvec3();
                FocusTarget::from_visible(vis, Some(pt_cam))
            })
            .or_else(|| Some(FocusTarget::from_ray(aim_ray_cam.as_dvec3(), 2.5, None)));

        let _ = self
            .flight
            .step_vr(&self.spatial, vr_input, focus_target.as_ref(), dt_sec as f64);

        self.step_common(
            vr_camera,
            wrist_input,
            aim_ray_cam,
            ray_angular_extra_deg_per_sec,
            Some(&self.flight.pose.orient.as_quat()),
            dt_sec,
            width,
            height,
            None,
        )
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

        let visible = self
            .spatial
            .query_visible(
                &self.flight.pose,
                camera.eyes[0].view_proj,
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

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
        let candidates = build_focus_candidates_filtered(&visible, cam_orient, sector_pref, preferred_exp_id);
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
        _vr_hint: Option<&glam::Quat>,
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

        let mut haptic_pulse: Option<(f32, i64, f32)> = None;
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
                    self.trigger_home();
                    haptic_pulse = Some((0.7, 45_000_000, 140.0));
                }
            }
        }
        if self.wrist_ring.hovered_bead_id != self.prev_hovered_bead
            && self.wrist_ring.hovered_bead_id.is_some()
        {
            haptic_pulse = haptic_pulse.or(Some((0.25, 14_000_000, 260.0)));
        }
        self.prev_hovered_bead = self.wrist_ring.hovered_bead_id;

        // 2) Spatial octree frustum query.
        let visible = self
            .spatial
            .query_visible(
                &self.flight.pose,
                camera.eyes[0].view_proj,
                0.05_f32.to_radians(),
                Some(512),
            )
            .map(|q| q.visible)
            .unwrap_or_default();

        let cam_orient = self.flight.pose.orient.as_quat();
        let active_experience_id = verification_override
            .map(|o| o.active_experience_id)
            .unwrap_or_else(|| infer_active_experience(&self.flight.pose, &visible));

        // Update zone drone synth gently when changing active cosmic experience/zone.
        self.sync_zone_ambient_audio(active_experience_id, &visible);

        // 3) Rest-to-Learn FSM step.
        let forced_exp = verification_override.map(|o| o.active_experience_id);
        let sector_pref =
            forced_exp.or_else(|| sector_experience_preference(self.flight.pose.sector_id));
        let candidates =
            build_focus_candidates_filtered(&visible, cam_orient, sector_pref, forced_exp);
        let session_finished = self
            .active_session
            .as_ref()
            .is_some_and(|s| s.phase == SegmentPhase::Completed);

        let fsm_out = self.fsm.step(&FsmInput {
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
        });

        let now_kind = fsm_out.state.kind();
        if now_kind == FsmStateKind::Locked && self.prev_fsm_kind != FsmStateKind::Locked {
            haptic_pulse = haptic_pulse.or(Some((0.35, 25_000_000, 190.0)));
        }
        self.prev_fsm_kind = now_kind;

        if fsm_out.should_cancel_audio && verification_override.is_none() {
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
            let dir_head = cam_orient.conjugate() * dir_world;
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
        let mut narration_spec =
            self.build_bilingual_spec(&fsm_out, active_experience_id, verification_override);
        if let Some(ref mut spec) = narration_spec {
            spec.anchor_world_pos = resolve_narration_card_anchor(
                &visible,
                active_experience_id,
                spec.anchor_world_pos,
                verification_override,
            );
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
            focused_id,
            self.locale,
        );
        self.append_diegetic_msdf_labels(camera, &mut ambient_inputs);

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

        let blockers = collect_external_blockers(
            &self.wrist_ring,
            &visible,
            active_experience_id,
            focused_id,
            camera,
        );
        let mut typo = self.typo_layout.build_frame_with_blockers(
            &self.atlas,
            &cam_basis,
            &ambient_inputs,
            narration_spec.as_ref(),
            &blockers,
            dt,
        );
        append_wrist_ring_tethers_and_labels(&self.wrist_ring, &self.atlas, camera, &mut typo);

        // 5) Build RenderSceneFrame (compute, far shell, volumetric heroes, diegetic pearls).
        let audio_uniforms = to_render_audio_uniforms(self.audio.reactive_uniforms());
        // Cold start (`u = 6.60`) presents an intact dramatic Earth; deep mantle cutaway opens smoothly below `u = 6.42`.
        let earth_cutaway_blend = if active_experience_id == 3 && self.flight.pose.scale_exp < 6.42
        {
            (1.0 - smoothstep(5.40, 6.42, self.flight.pose.scale_exp) as f32).clamp(0.0, 1.0)
        } else {
            0.0
        };

        let diegetic = DiegeticUiInput {
            wrist_pos: None, // Emitted directly with exact WristResonanceRing bead poses below.
            wrist_orient: self.wrist_ring.anchor_rot,
            playback_rate: self.wrist_ring.active_playback_rate(),
            locale_idx: self.locale as u8,
            focus_ray: cam_orient * fsm_out.smoothed_ray_dir_cam,
            dwell_progress: fsm_out.dwell_progress,
            show_focus_shimmer: true,
        };

        let documentary = self.build_documentary_focus_state(
            active_experience_id,
            &fsm_out,
            &visible,
            &audio_uniforms,
            verification_override,
        );
        let earth_cutaway_blend = verification_override
            .and_then(|o| o.force_earth_cutaway)
            .unwrap_or(earth_cutaway_blend);

        let mut scene = self.scene_builder.build(
            &self.spatial,
            &SceneBuildInput {
                cam_pose: self.flight.pose,
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
            return DocumentaryFocusState {
                active_zone_index,
                active_feature_index,
                scene_index,
                narration_cue_progress: snap.narration_cue_progress,
                narration_active_weight: snap.narration_active_weight,
                zone_accent_rgb,
                telemetry,
            };
        }

        let hovered_target = fsm_out
            .state
            .target()
            .filter(|t| t.experience_id == active_experience_id);
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
            .or_else(|| hovered_target.map(|t| t.feature_index))
            .or_else(|| fallback_vis.map(|v| v.feature_index))
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

fn non_empty_es(seg: &NarrationSegment) -> String {
    let spoken = seg.spoken_text(Locale::EsEs).trim();
    if spoken.is_empty() {
        seg.display_text.trim().to_owned()
    } else {
        spoken.to_owned()
    }
}

fn infer_active_experience(pose: &cosmos_math::ScalePose64, visible: &[VisibleEntity]) -> u8 {
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

fn pick_best_entity_along_ray(
    visible: &[VisibleEntity],
    ray_world: Vec3,
    max_angle_deg: f32,
) -> Option<&VisibleEntity> {
    let ray = ray_world.normalize_or(Vec3::NEG_Z);
    let max_rad = max_angle_deg.to_radians();
    let mut best: Option<(&VisibleEntity, f32)> = None;

    for vis in visible {
        let len = vis.pose.pos.length();
        if len < 1e-4 {
            continue;
        }
        let dir = vis.pose.pos / len;
        let theta = dir.dot(ray).clamp(-1.0, 1.0).acos();
        let eff_cone = max_rad.max(vis.pose.angular_rad * 0.5 + 0.03);
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
        // Saturn's tilted Cassini/Encke rings extend ~2.25x past the planet globe radius.
        (
            (entity.pose.angular_rad * 2.65).clamp(0.14, 0.44) + 3.2_f32.to_radians(),
            (entity.pose.angular_rad * 1.55).clamp(0.10, 0.32) + 3.0_f32.to_radians(),
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

    // Subtle zero-overlap radial micro-labels for all 12 beads, kept clear of ambient label budget.
    let center = wrist_ring.beads[0].world_pos;
    let right = camera.cam_right.normalize_or(Vec3::X);
    let up = camera.cam_up.normalize_or(Vec3::Y);
    let to_eye = -camera.cam_forward.normalize_or(Vec3::NEG_Z);

    for bead in &wrist_ring.beads {
        let is_active = bead.selected || bead.highlighted;
        let shaped = atlas.shape_text(FontRole::UiSans, &bead.label, None);
        if shaped.glyphs.is_empty() {
            continue;
        }

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
        let anchor = bead.world_pos + label_offset + to_eye * 0.004;

        let dist = anchor.length().max(0.2);
        let em_h = dist * (if is_active { 0.58_f32 } else { 0.42_f32 }).to_radians();
        let baseline_y_m = -0.35 * shaped.ascent_em * em_h;
        let color_hdr = if is_active {
            [
                (bead.color_hdr[0] * 1.15).max(0.85),
                (bead.color_hdr[1] * 1.15).max(0.85),
                (bead.color_hdr[2] * 1.15).max(0.75),
                0.96 * alpha,
            ]
        } else {
            [0.64, 0.76, 0.90, 0.62 * alpha]
        };
        let halo_color = [0.002, 0.003, 0.006, if is_active { 0.85 } else { 0.60 } * alpha];
        let mode = if is_active {
            GlyphInkMode::DwellFocusedHeading as u8 as f32
        } else {
            GlyphInkMode::SpatialAnchor as u8 as f32
        };

        for sg in &shaped.glyphs {
            if !sg.metrics.has_outline {
                continue;
            }
            let line_w_em = shaped.lines[sg.line_index].width_em;
            let cx_world = ((sg.x_em - 0.5 * line_w_em) + 0.5 * (sg.metrics.plane_bounds[0] + sg.metrics.plane_bounds[2])) * em_h;
            let cy_world = baseline_y_m + (sg.y_em + 0.5 * (sg.metrics.plane_bounds[1] + sg.metrics.plane_bounds[3])) * em_h;
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
}

fn build_focus_candidates_filtered(
    visible: &[VisibleEntity],
    cam_orient: glam::Quat,
    sector_exp_pref: Option<u8>,
    required_exp: Option<u8>,
) -> Vec<FocusCandidate> {
    let inv_orient = cam_orient.conjugate();
    let max_take = if sector_exp_pref.is_some() { 256 } else { 96 };
    let build = |filter_exp: Option<u8>| -> Vec<FocusCandidate> {
        visible
            .iter()
            .filter(|v| filter_exp.is_none_or(|e| v.experience_id == e))
            .take(max_take)
            .filter_map(|v| {
                let len = v.pose.pos.length();
                if len < 1e-4 {
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
                    angular_radius_deg: v.pose.angular_rad.to_degrees(),
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

fn append_diegetic_sprites(
    wrist_ring: &WristResonanceRing,
    scale_meridian: &ScaleMeridianArc,
    camera: &RenderCamera,
    _time_sec: f32,
    sprites: &mut Vec<SpriteInstance>,
) {
    if wrist_ring.summon_mode != SummonMode::Hidden || wrist_ring.open_alpha > 0.02 {
        let alpha = wrist_ring.open_alpha.clamp(0.0, 1.0);
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
                tags_f: [4.0, (i as f32) / 12.0, sub_param, 0.0],
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
        let eye = DVec3::new(72.0, 36.0, 207.0);
        let fwd = (DVec3::new(48.0, 4.0, -20.0) - eye).normalize();
        let cam_x = fwd.cross(DVec3::Y).normalize();
        let cam_z = -fwd;
        let cam_y = cam_z.cross(cam_x).normalize();
        let rot = DQuat::from_mat3(&glam::DMat3::from_cols(cam_x, cam_y, cam_z)).normalize();
        rt.flight.pose = ScalePose64::new(SectorId::SOLAR_SYSTEM, eye, 12.8, rot);
        let cam_sat = RenderCamera::mono(
            rot.as_quat(),
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
            .expect("Saturn visible in 03 showcase pose");
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
                .any(|b| b.half_az > saturn.pose.angular_rad * 2.0),
            "expected wide Saturn Cassini ring blocker (planet ang={:.3}, blockers={blockers:?})",
            saturn.pose.angular_rad
        );
    }
}
