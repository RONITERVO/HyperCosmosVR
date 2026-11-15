//! Runtime-catalog camera/renderer integration, without creating a GPU device.
//! Bounds catch absent, tiny and misdirected subjects; they do not establish
//! occupied-pixel detail, scientific clarity or headset visual acceptance.
use cosmos_app::assets::UniverseAssets;
use cosmos_journey::{
    BeatTargetSelector, CosmicHighlightsPlugin, GrandCosmicOdysseyPlugin, StoryJourneyManifest,
    StoryJourneyPlugin,
};
use cosmos_render::{RenderCamera, SceneBuildInput, UniverseSceneBuilder};
use glam::Vec3;

fn assert_authored_observations(assets: &UniverseAssets, manifest: &StoryJourneyManifest) {
    let mut builder = UniverseSceneBuilder::new();
    for beat in manifest.chapters.iter().flat_map(|c| &c.beats) {
        let Some(exp) = beat.target.experience_id() else {
            continue;
        };
        // These intentionally look across a horizon or through the wind, so a
        // centered globe/bounding sphere is not their composition contract.
        if exp == 3 && manifest.id == "cosmic_highlights_tour" {
            continue;
        }
        let pose = beat.pose.to_pose64(&beat.beat_id).unwrap();
        let camera = RenderCamera::mono(
            pose.orient.as_quat(),
            72_f32.to_radians(),
            16.0 / 9.0,
            0.002,
        );
        let mut input = SceneBuildInput::desktop_default(pose, camera, 2560, 1440);
        input.active_experience_id = exp;
        input.particle_budget = 0;
        if let BeatTargetSelector::CatalogZone {
            zone_index,
            feature_index,
            ..
        } = beat.target
        {
            input.documentary.active_zone_index = zone_index;
            input.documentary.active_feature_index = feature_index.unwrap_or(u16::MAX);
        }
        let scene = builder.build(&assets.spatial, &input);
        let vid = cosmos_render::doc_stages::resolve_zone_shader_vid(exp, &input.documentary);
        let primary = scene
            .heroes_surface
            .iter()
            .chain(&scene.heroes_volume)
            .filter(|h| h.kind_tags[1] == vid || exp <= 2)
            .max_by(|a, b| {
                let ratio = |h: &cosmos_render::HeroVolumeInstance| {
                    h.pos_radius[3] / Vec3::from_slice(&h.pos_radius[..3]).length()
                };
                ratio(a).total_cmp(&ratio(b))
            })
            .unwrap_or_else(|| panic!("{}: no primary subject", beat.beat_id));
        let position = Vec3::from_slice(&primary.pos_radius[..3]);
        let half_angle = (primary.pos_radius[3] / position.length())
            .min(1.0)
            .asin()
            .to_degrees();
        let offset = position
            .normalize()
            .dot(camera.cam_forward)
            .clamp(-1.0, 1.0)
            .acos()
            .to_degrees();
        assert!(
            half_angle >= 6.0,
            "{}: primary is too small ({:.2} degrees)",
            beat.beat_id,
            half_angle * 2.0
        );
        assert!(
            offset <= 12.0,
            "{}: camera misses subject by {offset:.2} degrees",
            beat.beat_id
        );
        // The global field's conservative 30-Earth-radius sphere contains large
        // empty corners; its actual selected path is narrower than the sphere.
        let global_field = exp == 3
            && input.documentary.active_zone_index == 0
            && input.documentary.active_feature_index <= 1;
        let max_extent = if global_field { 52.0 } else { 36.0 };
        assert!(
            offset + half_angle <= max_extent,
            "{}: subject exceeds authored field of view ({:.2} degree extent)",
            beat.beat_id,
            offset + half_angle
        );
        if exp == 3 && input.documentary.active_zone_index >= 7 {
            // The shader removes the positive octant. Looking from another
            // side leaves the intended interior lesson behind an opaque globe.
            assert!(
                pose.local_pos.x > 0.0 && pose.local_pos.y > 0.0 && pose.local_pos.z > 0.0,
                "{}: camera cannot see the authored cutaway opening",
                beat.beat_id
            );
        }
    }
}

#[test]
fn runtime_catalog_tours_frame_the_rendered_subjects() {
    let assets = UniverseAssets::load().expect("runtime catalog and spatial index");
    let odyssey = GrandCosmicOdysseyPlugin
        .build_manifest(&assets.catalog, &assets.spatial)
        .unwrap();
    assert!(
        odyssey.total_beats() > 3_000,
        "complete catalog coverage must remain present"
    );
    assert_authored_observations(&assets, &odyssey);
    let highlights = CosmicHighlightsPlugin
        .build_manifest(&assets.catalog, &assets.spatial)
        .unwrap();
    assert_eq!(highlights.total_beats(), 44);
    assert_authored_observations(&assets, &highlights);
    let shipped = StoryJourneyManifest::from_json_str(
        &std::fs::read_to_string(
            assets
                .root
                .join("assets/journeys/cosmic_highlights_tour.hcj.json"),
        )
        .unwrap(),
    )
    .unwrap();
    // The exporter must use the enriched runtime index, which includes real
    // Solar/Galactic landmarks absent from build_default_universe_index().
    let round_tripped =
        StoryJourneyManifest::from_json_str(&highlights.to_json_pretty().unwrap()).unwrap();
    assert!(
        shipped.chapters == round_tripped.chapters,
        "regenerate with cargo run --release -p cosmos_app --example tour_projection_audit -- --export"
    );
    assert_authored_observations(&assets, &shipped);
}
