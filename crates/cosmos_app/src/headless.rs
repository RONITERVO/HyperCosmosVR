use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::Result;
use cosmos_audio::AudioOutputMode;
use cosmos_manifest::Locale;
use cosmos_math::{ScalePose64, SectorId, DEFAULT_DESKTOP_NEAR_RENDER_UNITS};
use cosmos_render::{CosmosRenderer, HDR_SCENE_FORMAT, DEPTH_FORMAT, RenderCamera};
use cosmos_typo::{MsdfPipelineConfig, MsdfTextRenderer, TextPassUniforms};
use glam::{DMat3, DQuat, DVec3, Vec3};

use crate::assets::UniverseAssets;
use crate::engine::{UniverseRuntime, VerificationNarrationOverride};

/// Output telemetry for a single deterministic headless verification shot.
#[derive(Clone, Debug)]
pub struct HeadlessShotReport {
    pub filename: String,
    pub path: PathBuf,
    pub experience_id: u8,
    pub scale_exp: f64,
    pub render_ms: f64,
    pub glyph_count: u32,
    pub tether_count: u32,
    pub pure_black_pixels: usize,
    pub non_black_pixels: usize,
}

/// Summary benchmark metrics across the 62+ order logarithmic trajectory sweep.
#[derive(Clone, Debug)]
pub struct BenchmarkSweepReport {
    pub frame_count: usize,
    pub min_ms: f64,
    pub mean_ms: f64,
    pub p99_ms: f64,
    pub max_ms: f64,
    pub adapter_name: String,
}

struct ShowcaseShot {
    filename: &'static str,
    sector_id: SectorId,
    local_pos: DVec3,
    look_target: DVec3,
    scale_exp: f64,
    experience_id: u8,
    zone_index: Option<u16>,
    feature_index: Option<u16>,
    earth_cutaway: Option<f32>,
    locale: Locale,
    playback_rate: f32,
    ghost_translation: bool,
    reveal: f32,
    scale_vel: Option<f64>,
    warmup_frames: u32,
    summon_journey_prompt: bool,
}

impl ShowcaseShot {
    #[allow(clippy::too_many_arguments)]
    fn new(
        filename: &'static str,
        sector_id: SectorId,
        local_pos: DVec3,
        look_target: DVec3,
        scale_exp: f64,
        experience_id: u8,
        locale: Locale,
        playback_rate: f32,
        ghost_translation: bool,
        reveal: f32,
        scale_vel: Option<f64>,
        warmup_frames: u32,
    ) -> Self {
        Self {
            filename,
            sector_id,
            local_pos,
            look_target,
            scale_exp,
            experience_id,
            zone_index: None,
            feature_index: None,
            earth_cutaway: None,
            locale,
            playback_rate,
            ghost_translation,
            reveal,
            scale_vel,
            warmup_frames,
            summon_journey_prompt: false,
        }
    }

    fn with_journey_prompt(mut self) -> Self {
        self.summon_journey_prompt = true;
        self
    }

    fn with_zone(mut self, zone_index: u16) -> Self {
        self.zone_index = Some(zone_index);
        self
    }

    fn with_feature(mut self, zone_index: u16, feature_index: u16) -> Self {
        self.zone_index = Some(zone_index);
        self.feature_index = Some(feature_index);
        self
    }

    fn with_cutaway(mut self, blend: f32) -> Self {
        self.earth_cutaway = Some(blend);
        self
    }
}

fn canonical_showcase_shots() -> Vec<ShowcaseShot> {
    vec![
        ShowcaseShot::new(
            "01_cold_start_earth_aurora.png",
            SectorId::EARTH_ORBIT,
            DVec3::new(0.0, 0.16, 3.55),
            DVec3::ZERO,
            6.6,
            3,
            Locale::EnUs,
            1.5,
            true,
            0.64,
            None,
            8,
        )
        .with_zone(2)
        .with_cutaway(0.0),
        ShowcaseShot::new(
            "02_earth_deep_cutaway.png",
            SectorId::EARTH_ORBIT,
            DVec3::new(0.95, 0.45, 2.48),
            DVec3::ZERO,
            6.34,
            3,
            Locale::FiFi,
            1.25,
            true,
            0.70,
            None,
            8,
        )
        .with_zone(13)
        .with_cutaway(1.0),
        ShowcaseShot::new(
            "02b_scale_meridian_dive.png",
            SectorId::EARTH_ORBIT,
            DVec3::new(0.42, 0.30, 3.60),
            DVec3::ZERO,
            6.50,
            3,
            Locale::EnUs,
            1.5,
            false,
            0.54,
            Some(-1.45),
            8,
        ),
        ShowcaseShot::new(
            "03_solar_system_saturn.png",
            SectorId::SOLAR_SYSTEM,
            // SOLAR_SYSTEM canonical scale is 11.2; resolved dynamically relative to Saturn (`zone 7`).
            DVec3::new(-3.682, 0.3105, 4.781),
            DVec3::new(-2.522, -0.0795, 4.021),
            10.72,
            4,
            Locale::EnUs,
            1.75,
            true,
            0.68,
            None,
            8,
        )
        .with_zone(7),
        ShowcaseShot::new(
            "03b_solar_system_panorama.png",
            SectorId::SOLAR_SYSTEM,
            DVec3::new(-4.6, 8.8, 16.2),
            DVec3::new(0.0, -0.2, 0.0),
            11.45,
            4,
            Locale::EnUs,
            1.5,
            true,
            0.72,
            None,
            10,
        )
        .with_zone(0),
        ShowcaseShot::new(
            "04_milky_way_sgr_a_star.png",
            SectorId::MILKY_WAY,
            DVec3::new(0.0, 2.25, 7.80),
            DVec3::ZERO,
            20.5,
            5,
            Locale::EnUs,
            2.0,
            true,
            0.62,
            None,
            10,
        )
        .with_zone(10),
        ShowcaseShot::new(
            "04b_milky_way_landmarks.png",
            SectorId::MILKY_WAY,
            DVec3::new(1.85, 3.85, 7.60),
            DVec3::ZERO,
            20.5,
            5,
            Locale::FiFi,
            1.5,
            true,
            0.65,
            None,
            10,
        )
        .with_zone(7),
        ShowcaseShot::new(
            "05_cosmic_web.png",
            SectorId::COSMIC_WEB,
            DVec3::new(0.15, 0.45, 3.60),
            DVec3::ZERO,
            25.35,
            8,
            Locale::FiFi,
            1.5,
            true,
            0.66,
            None,
            10,
        ),
        ShowcaseShot::new(
            "06_cmb_and_inflation.png",
            SectorId::OBSERVABLE_CMB,
            DVec3::new(0.0, 0.25, 3.20),
            DVec3::ZERO,
            26.35,
            9,
            Locale::EnUs,
            1.25,
            true,
            0.65,
            None,
            8,
        ),
        ShowcaseShot::new(
            "07_quantum_atom_orbitals.png",
            SectorId::QUANTUM_FIELD,
            DVec3::new(1.22, 0.82, 3.35),
            DVec3::ZERO,
            -10.0,
            13,
            Locale::EnUs,
            1.5,
            true,
            0.60,
            None,
            10,
        )
        .with_zone(3),
        ShowcaseShot::new(
            "07b_matter_quarks_confinement.png",
            SectorId::QUANTUM_FIELD,
            DVec3::new(0.95, 0.65, 2.85),
            DVec3::ZERO,
            -11.25,
            13,
            Locale::EnUs,
            1.5,
            true,
            0.64,
            None,
            10,
        )
        .with_zone(6),
        ShowcaseShot::new(
            "07c_matter_molecules.png",
            SectorId::QUANTUM_FIELD,
            DVec3::new(-1.10, 0.72, 3.10),
            DVec3::ZERO,
            -10.2,
            13,
            Locale::FiFi,
            1.25,
            false,
            0.58,
            None,
            10,
        )
        .with_zone(2),
        ShowcaseShot::new(
            "08_planck_foam.png",
            SectorId::PLANCK_ORIGIN,
            DVec3::new(0.55, 0.48, 2.75),
            DVec3::ZERO,
            -35.0,
            14,
            Locale::FiFi,
            2.25,
            true,
            0.62,
            None,
            10,
        )
        .with_zone(11),
        ShowcaseShot::new(
            "08b_spacetime_gravitational_waves.png",
            SectorId::PLANCK_ORIGIN,
            DVec3::new(0.75, 0.95, 3.15),
            DVec3::ZERO,
            -29.6,
            14,
            Locale::EnUs,
            1.5,
            true,
            0.68,
            None,
            10,
        )
        .with_zone(8),
        ShowcaseShot::new(
            "09a_mind_hsl_prism_harp_nested.png",
            SectorId::MIND_CATHEDRAL,
            DVec3::new(0.0, 0.10, 0.90),
            DVec3::new(0.0, 0.02, -0.04),
            2.15,
            17,
            Locale::EnUs,
            1.5,
            true,
            0.64,
            None,
            8,
        ),
        ShowcaseShot::new(
            "09b_colors_hsl_prism.png",
            SectorId::COLORS_PRISM,
            DVec3::new(0.28, 0.18, 0.92),
            DVec3::new(0.0, 0.02, 0.0),
            1.95,
            1,
            Locale::EnUs,
            1.5,
            true,
            0.72,
            None,
            8,
        ),
        ShowcaseShot::new(
            "09c_music_cymatic_harp.png",
            SectorId::MUSIC_HARP,
            DVec3::new(-0.28, 0.18, 0.88),
            DVec3::new(0.0, -0.02, 0.0),
            1.92,
            2,
            Locale::EnUs,
            1.25,
            false,
            0.58,
            None,
            8,
        ),
        ShowcaseShot::new(
            "09d_colors_materials_gallery.png",
            SectorId::COLORS_PRISM,
            DVec3::new(-0.22, -0.02, 0.64),
            DVec3::new(0.0, -0.04, 0.0),
            1.84,
            1,
            Locale::FiFi,
            1.5,
            true,
            0.70,
            None,
            8,
        ),
        ShowcaseShot::new(
            "09e_music_48string_chladni.png",
            SectorId::MUSIC_HARP,
            DVec3::new(0.0, 0.36, 1.05),
            DVec3::new(0.0, -0.02, 0.0),
            1.98,
            2,
            Locale::EnUs,
            1.25,
            true,
            0.66,
            None,
            8,
        ),
        ShowcaseShot::new(
            "10_exoplanets_signal_cones.png",
            SectorId::ORION_SPUR,
            DVec3::new(0.95, 0.47, 2.55),
            DVec3::new(0.40, 0.05, -0.30),
            18.3,
            22,
            Locale::EnUs,
            1.75,
            true,
            0.66,
            None,
            8,
        ),
        ShowcaseShot::new(
            "10b_seti_prime_beacon.png",
            SectorId::ORION_SPUR,
            DVec3::new(-0.85, 0.62, 2.45),
            DVec3::new(0.40, 0.22, -0.30),
            18.32,
            22,
            Locale::EnUs,
            1.5,
            true,
            0.72,
            None,
            8,
        )
        .with_zone(12),
        ShowcaseShot::new(
            "11_final_horizon_lotus.png",
            SectorId::HYPER_LOTUS_25_HORIZON,
            DVec3::new(0.0, 5.0, 11.6),
            DVec3::ZERO,
            28.6,
            25,
            Locale::EnUs,
            1.5,
            true,
            0.68,
            None,
            10,
        ),
        ShowcaseShot::new(
            "12_local_group_galaxies.png",
            SectorId::LOCAL_GROUP,
            DVec3::new(0.45, 0.95, 3.40),
            DVec3::ZERO,
            22.0,
            6,
            Locale::FiFi,
            1.5,
            true,
            0.65,
            None,
            8,
        ),
        ShowcaseShot::new(
            "13_laniakea_flow_basin.png",
            SectorId::LANIAKEA,
            DVec3::new(-0.65, 1.15, 3.65),
            DVec3::ZERO,
            23.8,
            7,
            Locale::EnUs,
            1.75,
            false,
            0.56,
            None,
            8,
        ),
        ShowcaseShot::new(
            "14_early_inflation_horizon.png",
            SectorId::EARLY_INFLATION,
            DVec3::new(0.35, 0.45, 3.10),
            DVec3::ZERO,
            26.9,
            10,
            Locale::EnUs,
            1.25,
            true,
            0.62,
            None,
            8,
        ),
        ShowcaseShot::new(
            "15_hyper_lotus_evidence.png",
            SectorId::HYPER_LOTUS_11_EVIDENCE,
            DVec3::new(0.35, 2.65, 2.95),
            DVec3::ZERO,
            27.6,
            11,
            Locale::FiFi,
            1.5,
            true,
            0.66,
            None,
            8,
        ),
        ShowcaseShot::new(
            "16_hyper_lotus_future.png",
            SectorId::HYPER_LOTUS_12_FUTURE,
            DVec3::new(-2.95, 2.65, 0.35),
            DVec3::ZERO,
            27.6,
            12,
            Locale::EnUs,
            1.5,
            false,
            0.58,
            None,
            8,
        ),
        ShowcaseShot::new(
            "17_complexity_emergence_helix.png",
            SectorId::COMPLEXITY_HELIX,
            DVec3::new(1.10, 0.72, 3.55),
            DVec3::ZERO,
            -7.0,
            15,
            Locale::EnUs,
            1.5,
            true,
            0.64,
            None,
            8,
        ),
        ShowcaseShot::new(
            "18_life_biosphere_tree.png",
            SectorId::LIFE_BIOSPHERE,
            DVec3::new(-0.95, 0.82, 3.80),
            DVec3::ZERO,
            -1.0,
            16,
            Locale::FiFi,
            1.25,
            true,
            0.68,
            None,
            8,
        ),
        ShowcaseShot::new(
            "18b_life_dna_cell.png",
            SectorId::LIFE_BIOSPHERE,
            DVec3::new(0.72, 0.34, 2.85),
            DVec3::ZERO,
            -1.05,
            16,
            Locale::EnUs,
            1.5,
            true,
            0.66,
            None,
            8,
        )
        .with_zone(4),
        ShowcaseShot::new(
            "18c_mind_brain_lobes.png",
            SectorId::MIND_CATHEDRAL,
            DVec3::new(0.85, 0.55, 3.30),
            DVec3::ZERO,
            2.75,
            17,
            Locale::EnUs,
            1.5,
            true,
            0.68,
            None,
            8,
        )
        .with_zone(5),
        ShowcaseShot::new(
            "19_culture_civilization_web.png",
            SectorId::CULTURE_WEB,
            DVec3::new(0.75, 0.95, 3.95),
            DVec3::ZERO,
            6.2,
            18,
            Locale::EnUs,
            1.5,
            false,
            0.52,
            None,
            8,
        ),
        ShowcaseShot::new(
            "20_technology_orbital_ring.png",
            SectorId::TECH_RING,
            DVec3::new(-1.05, 1.05, 4.00),
            DVec3::ZERO,
            7.6,
            19,
            Locale::EnUs,
            1.75,
            true,
            0.65,
            None,
            8,
        ),
        ShowcaseShot::new(
            "20b_tech_ai_silicon_chip.png",
            SectorId::TECH_RING,
            DVec3::new(0.95, 0.88, 3.60),
            DVec3::ZERO,
            7.55,
            19,
            Locale::EnUs,
            1.5,
            true,
            0.70,
            None,
            8,
        )
        .with_zone(16),
        ShowcaseShot::new(
            "21_futures_possibility_arcs.png",
            SectorId::FUTURES_ARCS,
            DVec3::new(0.82, 1.30, 4.45),
            DVec3::ZERO,
            11.8,
            20,
            Locale::FiFi,
            1.5,
            true,
            0.63,
            None,
            8,
        ),
        ShowcaseShot::new(
            "21b_futures_dyson_oneill.png",
            SectorId::FUTURES_ARCS,
            DVec3::new(-0.92, 1.15, 4.10),
            DVec3::ZERO,
            11.85,
            20,
            Locale::EnUs,
            1.5,
            true,
            0.68,
            None,
            8,
        )
        .with_zone(16),
        ShowcaseShot::new(
            "22_cosmic_life_orion_spur.png",
            SectorId::ORION_SPUR,
            DVec3::new(-1.08, 0.56, 3.55),
            DVec3::new(-0.35, 0.0, 0.25),
            18.15,
            21,
            Locale::EnUs,
            1.5,
            false,
            0.60,
            None,
            8,
        ),
        ShowcaseShot::new(
            "22b_cosmic_life_europa_plumes.png",
            SectorId::ORION_SPUR,
            DVec3::new(0.42, 0.44, 2.65),
            DVec3::new(-0.35, 0.05, 0.25),
            18.12,
            21,
            Locale::FiFi,
            1.5,
            true,
            0.66,
            None,
            8,
        )
        .with_zone(12),
        ShowcaseShot::new(
            "23_hyper_lotus_perspectives.png",
            SectorId::HYPER_LOTUS_23_PERSPECTIVES,
            DVec3::new(-0.35, 2.65, -2.95),
            DVec3::ZERO,
            27.6,
            23,
            Locale::FiFi,
            1.5,
            true,
            0.66,
            None,
            8,
        )
        .with_feature(8, 1),
        ShowcaseShot::new(
            "23b_perspectives_six_lenses.png",
            SectorId::HYPER_LOTUS_23_PERSPECTIVES,
            DVec3::new(-0.55, 2.40, -2.70),
            DVec3::ZERO,
            27.6,
            23,
            Locale::EnUs,
            1.5,
            true,
            0.72,
            None,
            8,
        )
        .with_feature(22, 2),
        ShowcaseShot::new(
            "24_hyper_lotus_meaning.png",
            SectorId::HYPER_LOTUS_24_MEANING,
            DVec3::new(2.95, 2.65, -0.35),
            DVec3::ZERO,
            27.6,
            24,
            Locale::EnUs,
            1.25,
            false,
            0.55,
            None,
            8,
        )
        .with_feature(18, 0),
        ShowcaseShot {
            filename: "24b_meaning_self_valuing_scales.png",
            sector_id: SectorId::HYPER_LOTUS_24_MEANING,
            local_pos: DVec3::new(2.70, 2.45, -0.55),
            look_target: DVec3::ZERO,
            scale_exp: 27.6,
            experience_id: 24,
            zone_index: Some(24),
            feature_index: Some(3),
            earth_cutaway: None,
            locale: Locale::FiFi,
            playback_rate: 1.5,
            ghost_translation: true,
            reveal: 0.68,
            scale_vel: None,
            warmup_frames: 8,
            summon_journey_prompt: false,
        },
        ShowcaseShot::new(
            "25_journey_peripheral_exit_prompt.png",
            SectorId::EARTH_ORBIT,
            DVec3::new(0.0, 0.16, 3.55),
            DVec3::ZERO,
            6.6,
            3,
            Locale::EnUs,
            1.5,
            true,
            0.64,
            None,
            8,
        )
        .with_zone(2)
        .with_cutaway(0.0)
        .with_journey_prompt(),
    ]
}

fn start_showcase_shot_override(
    runtime: &mut UniverseRuntime,
    camera: &RenderCamera,
    shot: &ShowcaseShot,
    summon_wrist_ring: bool,
) -> VerificationNarrationOverride {
    runtime.settle_and_start_target_narration(
        camera,
        Vec3::NEG_Z,
        Some(shot.experience_id),
        shot.zone_index,
        shot.feature_index,
    );
    if shot.summon_journey_prompt {
        let _ = runtime.activate_journey("cosmic_highlights_tour", true);
        let cam_rot = glam::Quat::from_mat3(&glam::Mat3::from_cols(
            camera.cam_right,
            camera.cam_up,
            -camera.cam_forward,
        ))
        .normalize();
        let cam_rot64 = glam::DQuat::from_xyzw(
            cam_rot.x as f64,
            cam_rot.y as f64,
            cam_rot.z as f64,
            cam_rot.w as f64,
        );
        for (idx, offset) in [
            DVec3::new(0.55, 0.25, -1.75),
            DVec3::new(0.78, 0.42, -1.90),
        ]
        .into_iter()
        .enumerate()
        {
            let wpt = ScalePose64::new(
                runtime.flight.pose.sector_id,
                runtime.flight.pose.local_pos + cam_rot64 * offset,
                runtime.flight.pose.scale_exp,
                runtime.flight.pose.orient,
            );
            runtime.journey_recorder.record_manual_waypoint(
                &wpt,
                cosmos_journey::BeatTargetSelector::None,
                Some(format!("verify_{idx}")),
            );
        }
        let _ = runtime.force_verification_leash_prompt(cam_rot);
    }
    VerificationNarrationOverride {
        active_experience_id: shot.experience_id,
        phase_reveal: shot.reveal,
        show_spanish_ghost_translation: shot.ghost_translation,
        summon_wrist_ring,
        force_playback_rate: Some(shot.playback_rate),
        force_scale_vel: shot.scale_vel,
        force_zone_index: shot.zone_index,
        force_feature_index: shot.feature_index,
        force_earth_cutaway: shot.earth_cutaway,
    }
}

fn orient_looking_at(eye: DVec3, target: DVec3) -> DQuat {
    let dir = target - eye;
    if dir.length_squared() < 1e-12 {
        return DQuat::IDENTITY;
    }
    let forward = dir.normalize();
    let up_ref = if forward.y.abs() > 0.99 {
        DVec3::Z
    } else {
        DVec3::Y
    };
    let cam_z = -forward;
    let cam_x = forward.cross(up_ref).normalize();
    let cam_y = cam_z.cross(cam_x).normalize();
    DQuat::from_mat3(&DMat3::from_cols(cam_x, cam_y, cam_z)).normalize()
}

fn resolved_shot_pose(runtime: &UniverseRuntime, shot: &ShowcaseShot) -> (DVec3, DVec3) {
    if shot.filename == "03_solar_system_saturn.png"
        && let Some(saturn) = runtime.spatial.find_zone_entity(4, 7)
    {
        return (
            saturn.local_pos + DVec3::new(-0.88, 0.36, 0.58),
            saturn.local_pos + DVec3::new(0.28, -0.03, -0.18),
        );
    }
    (shot.local_pos, shot.look_target)
}

/// Renders all signature verification PNGs on the real RTX 5070 DX12 pipeline with active 3D MSDF
/// bilingual live-ink narration, tethers, Google-Earth labels, and the Wrist Resonance Ring.
pub fn run_headless_render(
    out_dir: &Path,
    width: u32,
    height: u32,
    run_benchmark: bool,
) -> Result<(Vec<HeadlessShotReport>, Option<BenchmarkSweepReport>)> {
    fs::create_dir_all(out_dir)?;

    let assets = UniverseAssets::load()?;
    let shaders_dir = assets.shaders_dir.clone();
    let mut runtime = UniverseRuntime::new(assets, AudioOutputMode::Headless)?;
    let mut renderer = CosmosRenderer::init_desktop(width, height)?;

    let mut text_renderer = MsdfTextRenderer::from_shaders_dir(
        renderer.device(),
        renderer.queue(),
        &runtime.atlas,
        &shaders_dir,
        renderer.shared_frame_bind_group_layout(),
        MsdfPipelineConfig {
            color_format: HDR_SCENE_FORMAT,
            depth_format: Some(DEPTH_FORMAT),
            enable_stereo_multiview: true,
            sample_count: 1,
        },
    )?;

    let aspect = width.max(1) as f32 / height.max(1) as f32;
    let fov_y_rad = 72.0_f32.to_radians();
    let mut reports = Vec::new();

    for shot in canonical_showcase_shots() {
        runtime.reset_verification_state();
        runtime.locale = shot.locale;
        runtime.wrist_ring.selected_locale = shot.locale;
        let (local_pos, look_target) = resolved_shot_pose(&runtime, &shot);
        let orient = orient_looking_at(local_pos, look_target);
        runtime.flight.pose =
            ScalePose64::new(shot.sector_id, local_pos, shot.scale_exp, orient);
        runtime.flight.linear_vel_cam = DVec3::ZERO;
        runtime.flight.angular_vel_rad = DVec3::ZERO;
        runtime.flight.scale_vel = 0.0;

        let camera = RenderCamera::mono(
            orient.as_quat(),
            fov_y_rad,
            aspect,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );
        let override_cfg = start_showcase_shot_override(&mut runtime, &camera, &shot, true);

        // Warm up GPU particle advection + Wrist Resonance Ring opening + live audio envelope.
        let mut final_pkt = None;
        for _ in 0..shot.warmup_frames {
            final_pkt = Some(runtime.step_verification_frame(
                &camera,
                Vec3::NEG_Z,
                Some(&override_cfg),
                1.0 / 60.0,
                width,
                height,
            ));
        }
        let pkt = final_pkt.expect("shot warmup executes >= 1 step");

        let text_uniforms = TextPassUniforms::default();
        text_renderer.upload(
            renderer.device(),
            renderer.queue(),
            &text_uniforms,
            &pkt.typo.glyphs,
            &pkt.typo.tethers,
        );

        let out_path = out_dir.join(shot.filename);
        let t0 = Instant::now();
        let pixels = renderer.render_to_png(&out_path, &pkt.scene, width, height, |rpass, stereo| {
            text_renderer.render(rpass, None, stereo);
        })?;
        let render_ms = t0.elapsed().as_secs_f64() * 1000.0;

        let mut pure_black = 0usize;
        let mut non_black = 0usize;
        for px in pixels.as_chunks::<4>().0 {
            if px[0] == 0 && px[1] == 0 && px[2] == 0 {
                pure_black += 1;
            } else {
                non_black += 1;
            }
        }

        reports.push(HeadlessShotReport {
            filename: shot.filename.to_owned(),
            path: out_path,
            experience_id: shot.experience_id,
            scale_exp: shot.scale_exp,
            render_ms,
            glyph_count: text_renderer.glyph_count(),
            tether_count: text_renderer.tether_count(),
            pure_black_pixels: pure_black,
            non_black_pixels: non_black,
        });
    }

    let bench = if run_benchmark {
        Some(run_benchmark_sweep(
            &mut runtime,
            &mut renderer,
            &mut text_renderer,
            out_dir,
            width,
            height,
        )?)
    } else {
        None
    };

    Ok((reports, bench))
}

fn run_benchmark_sweep(
    runtime: &mut UniverseRuntime,
    renderer: &mut CosmosRenderer,
    text_renderer: &mut MsdfTextRenderer,
    out_dir: &Path,
    width: u32,
    height: u32,
) -> Result<BenchmarkSweepReport> {
    let frame_count = 120usize;
    let aspect = width.max(1) as f32 / height.max(1) as f32;
    let fov_y_rad = 72.0_f32.to_radians();

    // Render into offscreen target texture (without PNG CPU encode overhead) to time GPU+CPU cadence.
    let target_tex = renderer.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("benchmark_target"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let target_view = target_tex.create_view(&wgpu::TextureViewDescriptor::default());

    let mut timings_ms = Vec::with_capacity(frame_count);
    for i in 0..frame_count {
        let t = i as f64 / (frame_count - 1) as f64;
        let u = -35.0 + t * (28.5 - (-35.0));
        let sector = runtime.spatial.best_anchor_sector_for_scale(u);
        runtime.flight.pose = ScalePose64::new(
            sector,
            DVec3::new(0.0, 0.15, 2.8),
            u,
            DQuat::IDENTITY,
        );
        runtime.flight.scale_vel = 4.2;

        let camera = RenderCamera::mono(
            DQuat::IDENTITY.as_quat(),
            fov_y_rad,
            aspect,
            DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
        );

        let t0 = Instant::now();
        let pkt = runtime.step_verification_frame(&camera, Vec3::NEG_Z, None, 1.0 / 90.0, width, height);
        text_renderer.upload(
            renderer.device(),
            renderer.queue(),
            &TextPassUniforms::default(),
            &pkt.typo.glyphs,
            &pkt.typo.tethers,
        );
        renderer.render_frame(
            &pkt.scene,
            &target_view,
            wgpu::TextureFormat::Rgba8UnormSrgb,
            width,
            height,
            false,
            |rpass, stereo| text_renderer.render(rpass, None, stereo),
        );
        let _ = renderer.device().poll(wgpu::PollType::wait_indefinitely());
        timings_ms.push(t0.elapsed().as_secs_f64() * 1000.0);
    }

    timings_ms.sort_by(|a, b| a.total_cmp(b));
    let min_ms = timings_ms[0];
    let max_ms = *timings_ms.last().unwrap_or(&0.0);
    let mean_ms = timings_ms.iter().sum::<f64>() / timings_ms.len() as f64;
    let p99_idx = ((timings_ms.len() as f64) * 0.99).floor() as usize;
    let p99_ms = timings_ms[p99_idx.min(timings_ms.len() - 1)];

    let adapter_name = renderer.adapter_info().name.clone();
    let summary_txt = format!(
        "HyperCosmosVR RTX 5070 Benchmark Sweep ({frame_count} frames across u=[-35..+28.5])\n\
         Adapter: {adapter_name}\n\
         Resolution: {width}x{height}\n\
         min_ms:  {min_ms:.3}\n\
         mean_ms: {mean_ms:.3} ({:.1} FPS)\n\
         p99_ms:  {p99_ms:.3}\n\
         max_ms:  {max_ms:.3}\n",
        1000.0 / mean_ms.max(1e-4)
    );
    fs::write(out_dir.join("benchmark_sweep.txt"), summary_txt)?;

    Ok(BenchmarkSweepReport {
        frame_count,
        min_ms,
        mean_ms,
        p99_ms,
        max_ms,
        adapter_name,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canonical_showcase_shots_verify_authentic_narration_and_meridian_isolation() {
        let assets = UniverseAssets::load().expect("load runtime assets");
        let mut rt = UniverseRuntime::new(assets, AudioOutputMode::Headless).expect("init runtime");
        let aspect = 16.0_f32 / 9.0;
        let fov_y_rad = 66.0_f32.to_radians();

        let shots = canonical_showcase_shots();
        assert!(
            shots.len() >= 38,
            "expected >= 38 Phase 7 showcase shots, got {}",
            shots.len()
        );
        let unique_names: std::collections::HashSet<&str> =
            shots.iter().map(|s| s.filename).collect();
        assert_eq!(unique_names.len(), shots.len(), "duplicate showcase shot filenames");
        for exp_id in 1_u8..=25 {
            assert!(
                shots.iter().any(|s| s.experience_id == exp_id),
                "missing showcase coverage for experience {exp_id:02}"
            );
        }

        for shot in shots {
            rt.reset_verification_state();
            rt.locale = shot.locale;
            rt.wrist_ring.selected_locale = shot.locale;

            let (local_pos, look_target) = resolved_shot_pose(&rt, &shot);
            let rot = orient_looking_at(local_pos, look_target);

            rt.flight.pose = ScalePose64::new(shot.sector_id, local_pos, shot.scale_exp, rot);
            rt.flight.scale_vel = shot.scale_vel.unwrap_or(0.0);

            let cam = RenderCamera::mono(
                rot.as_quat(),
                fov_y_rad,
                aspect,
                DEFAULT_DESKTOP_NEAR_RENDER_UNITS,
            );
            let ovr = start_showcase_shot_override(
                &mut rt,
                &cam,
                &shot,
                shot.filename == "02_wrist_ring_menu.png",
            );
            let mut pkt = rt.step_verification_frame(&cam, Vec3::NEG_Z, Some(&ovr), 1.0 / 60.0, 1600, 900);
            for _ in 0..shot.warmup_frames {
                pkt = rt.step_verification_frame(&cam, Vec3::NEG_Z, Some(&ovr), 1.0 / 60.0, 1600, 900);
            }

            assert!(
                !pkt.typo.glyphs.is_empty(),
                "expected active 3D typography glyphs on {}",
                shot.filename
            );
            if shot.filename == "03_solar_system_saturn.png" {
                assert!(
                    !rt.scale_meridian.visible,
                    "02b scale meridian bled into 03_solar_system_saturn"
                );
            }
            if shot.filename == "02b_scale_meridian_dive.png" {
                assert!(rt.scale_meridian.visible, "02b scale meridian must be visible");
            }
            if shot.filename == "04_milky_way_sgr_a_star.png" {
                assert_eq!(pkt.active_experience_id, 5);
            }
            if shot.summon_journey_prompt {
                assert!(rt.is_journey_paused_by_leash());
                let prompt = rt
                    .active_peripheral_prompt()
                    .expect("peripheral FOLLOW/EXIT prompt visible");
                let fwd = cam.cam_forward.normalize();
                let ang_follow = fwd.angle_between(prompt.follow_world.normalize()).to_degrees();
                let ang_exit = fwd.angle_between(prompt.exit_world.normalize()).to_degrees();
                assert!(
                    ang_follow > 20.0 && ang_exit > 20.0,
                    "peripheral pearls encroached on 12-deg fovea: follow={ang_follow:.1} exit={ang_exit:.1}"
                );
                assert!(pkt.typo.tethers.len() >= 2, "expected guide tethers for prompt and waypoints");
                assert!(pkt.scene.sprites.len() >= 4, "expected FOLLOW/EXIT + waypoint pearls");
            }
            println!(
                "shot={} exp={} glyphs={} tethers={}",
                shot.filename,
                pkt.active_experience_id,
                pkt.typo.glyphs.len(),
                pkt.typo.tethers.len()
            );
        }
    }
}
