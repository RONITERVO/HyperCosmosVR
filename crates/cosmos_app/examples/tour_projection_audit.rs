//! CPU-only authored-tour framing audit; bounding spheres are not visual quality proof.
//! Use --odyssey for the complete catalog, --saved for the shipped Highlights,
//! or --export to regenerate shipped Highlights using the actual runtime index.
//! Use --manifest <path> to validate and inspect a standalone review journey.
use cosmos_app::assets::UniverseAssets;
use cosmos_journey::{
    BeatTargetSelector, CosmicHighlightsPlugin, GrandCosmicOdysseyPlugin, StoryJourneyManifest,
    StoryJourneyPlugin,
};
use cosmos_render::{RenderCamera, SceneBuildInput, UniverseSceneBuilder};
use glam::Vec3;

fn main() -> anyhow::Result<()> {
    let assets = UniverseAssets::load()?;
    let args: Vec<_> = std::env::args().collect();
    let external_path = args.iter().position(|a| a == "--manifest")
        .map(|index| args.get(index + 1)
            .ok_or_else(|| anyhow::anyhow!("--manifest requires a JSON path")))
        .transpose()?;
    let journey = if let Some(path) = external_path {
        StoryJourneyManifest::from_json_str(&std::fs::read_to_string(path)?)?
    } else if args.iter().any(|a| a == "--saved") {
        StoryJourneyManifest::from_json_str(&std::fs::read_to_string(
            assets
                .root
                .join("assets/journeys/cosmic_highlights_tour.hcj.json"),
        )?)?
    } else if args.iter().any(|a| a == "--odyssey") {
        GrandCosmicOdysseyPlugin.build_manifest(&assets.catalog, &assets.spatial)?
    } else {
        CosmicHighlightsPlugin.build_manifest(&assets.catalog, &assets.spatial)?
    };
    journey.validate(Some(&assets.catalog))?;
    if args.iter().any(|a| a == "--export") {
        anyhow::ensure!(
            external_path.is_none() && !args
                .iter()
                .any(|a| matches!(a.as_str(), "--odyssey" | "--saved")),
            "export requires built-in Highlights"
        );
        std::fs::write(
            assets
                .root
                .join("assets/journeys/cosmic_highlights_tour.hcj.json"),
            format!("{}\n", journey.to_json_pretty()?),
        )?;
    }
    let mut builder = UniverseSceneBuilder::new();
    println!("beat,exp,zone,feature,primary_bound_deg,center_offaxis_deg,variant,distance,radius");
    for beat in journey.chapters.iter().flat_map(|c| &c.beats) {
        let Some(exp) = beat.target.experience_id() else {
            continue;
        };
        let pose = beat.pose.to_pose64(&beat.beat_id)?;
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
            });
        if let Some(hero) = primary {
            let p = Vec3::from_slice(&hero.pos_radius[..3]);
            let r = hero.pos_radius[3];
            let angle = 2.0 * (r / p.length()).min(1.0).asin().to_degrees();
            let off = p
                .normalize()
                .dot(camera.cam_forward)
                .clamp(-1.0, 1.0)
                .acos()
                .to_degrees();
            println!(
                "{},{},{},{},{:.3},{:.3},{},{:.4},{:.4}",
                beat.beat_id,
                exp,
                input.documentary.active_zone_index,
                input.documentary.active_feature_index,
                angle,
                off,
                hero.kind_tags[1],
                p.length(),
                r
            );
        } else {
            println!(
                "{},{},{},{},MISSING,,,,",
                beat.beat_id,
                exp,
                input.documentary.active_zone_index,
                input.documentary.active_feature_index
            );
        }
    }
    Ok(())
}
