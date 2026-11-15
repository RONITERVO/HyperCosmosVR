//! Caption-free stereo evidence from the shipped Highlights camera poses.
//! cargo run --release -p cosmos_app --example earth_review -- target/earth_review 2560 1
//! Third argument is frames per stop (480 gives a full 16-second cycle at 30 fps).
//! Optional fourth argument filters beat IDs (comma-separated substrings, or all).
//! Optional fifth argument sets the first simulation time in seconds (default 13.2 for a still, 0 for a sequence).
//! Optional sixth argument sets eye height (default 16:9); use 2048 1 all 13.2 2048 for square stereo.
//! Optional seventh argument is capture stride. Zero captures only the last frame,
//! allowing consecutive timed frames without PNG/readback gaps (e.g. 2560 120 all 9.2 1440 0).
use cosmos_app::assets::UniverseAssets;
use cosmos_journey::{BeatTargetSelector, CosmicHighlightsPlugin, StoryJourneyPlugin};
use cosmos_math::{camera_view_matrix, reversed_z_infinite_perspective_rh};
use cosmos_render::{CosmosRenderer, EyeView, RenderCamera, SceneBuildInput, UniverseSceneBuilder};
use glam::{Mat4, Vec3};
use std::{path::PathBuf, time::Instant};

fn main() -> anyhow::Result<()> {
    let args: Vec<_> = std::env::args().collect();
    let output = PathBuf::from(
        args.get(1)
            .map(String::as_str)
            .unwrap_or("target/earth_review"),
    );
    let width: u32 = args.get(2).map(|s| s.parse()).transpose()?.unwrap_or(2560);
    let frames: u32 = args.get(3).map(|s| s.parse()).transpose()?.unwrap_or(1);
    let start_sec: f32 = args
        .get(5)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(if frames == 1 { 13.2 } else { 0.0 });
    anyhow::ensure!(
        width >= 128 && frames > 0,
        "width >=128, frames >0 required"
    );
    let height: u32 = args
        .get(6)
        .map(|s| s.parse())
        .transpose()?
        .unwrap_or(width * 9 / 16);
    anyhow::ensure!(height >= 128, "height >=128 required");
    let capture_stride: u32 = args.get(7).map(|s| s.parse()).transpose()?.unwrap_or(1);
    anyhow::ensure!(start_sec.is_finite(), "simulation time must be finite");
    anyhow::ensure!(
        !output.exists() || std::fs::read_dir(&output)?.next().is_none(),
        "use a fresh output directory to avoid mixing capture revisions"
    );
    std::fs::create_dir_all(&output)?;
    let assets = UniverseAssets::load()?;
    let journey = CosmicHighlightsPlugin.build_manifest(&assets.catalog, &assets.spatial)?;
    let init_start = Instant::now();
    let mut renderer = CosmosRenderer::init_for_dx12_luid(None, width, height, true)?;
    let init_seconds = init_start.elapsed().as_secs_f64();
    let format = wgpu::TextureFormat::Rgba8UnormSrgb;
    let texture = renderer.device().create_texture(&wgpu::TextureDescriptor {
        label: Some("Earth stereo review"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 2,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });
    let mut report = format!(
        "adapter={}\neye={}x{}\nrenderer_init_seconds={:.3}\ncapture_stride={} (0=consecutive render burst, last-frame capture only)\nsynthetic stereo; no headset/compositor; readback excluded from frame timing\n",
        renderer.adapter_info().name,
        width,
        height,
        init_seconds,
        capture_stride
    );
    for beat in &journey.chapters[0].beats {
        if args.get(4).is_some_and(|filter| {
            filter != "all" && !filter.split(',').any(|part| beat.beat_id.contains(part))
        }) {
            continue;
        }
        let BeatTargetSelector::CatalogZone {
            zone_index,
            feature_index,
            ..
        } = beat.target
        else {
            continue;
        };
        let pose = beat.pose.to_pose64(&beat.beat_id)?;
        let orient = pose.orient.as_quat();
        let mut camera = RenderCamera::mono(
            orient,
            72.0_f32.to_radians(),
            width as f32 / height as f32,
            0.002,
        );
        // Earth environment draw data is restored to physical meters by the
        // scene builder. Raw 64mm IPD matches the native headset camera policy.
        let projection =
            reversed_z_infinite_perspective_rh(camera.fov_y_rad, camera.aspect, camera.near_plane);
        for (eye, x) in [-0.032, 0.032].into_iter().enumerate() {
            let offset = orient * Vec3::new(x, 0.0, 0.0);
            let vp = projection * camera_view_matrix(orient) * Mat4::from_translation(-offset);
            camera.eyes[eye] = EyeView::from_view_proj(vp, offset);
        }
        camera.is_stereo = true;
        let mut builder = UniverseSceneBuilder::new();
        let folder = output.join(&beat.beat_id);
        std::fs::create_dir_all(&folder)?;
        let mut elapsed_ms = Vec::new();
        // Warm the pipelines before reporting reference timings. These include
        // submission/wait overhead and are not native-headset GPU timings.
        const WARMUP_FRAMES: u32 = 8;
        for frame in 0..frames + WARMUP_FRAMES {
            let mut input = SceneBuildInput::desktop_default(pose, camera, width, height);
            let capture_frame = frame.saturating_sub(WARMUP_FRAMES);
            input.time_sec = start_sec + capture_frame as f32 / 30.0;
            input.dt_sec = 1.0 / 30.0;
            input.documentary.active_zone_index = zone_index;
            input.documentary.active_feature_index = feature_index.unwrap_or(u16::MAX);
            let scene = builder.build(&assets.spatial, &input);
            let start = Instant::now();
            renderer.render_frame(&scene, &view, format, width, height, true, |_, _| {});
            renderer
                .device()
                .poll(wgpu::PollType::wait_indefinitely())?;
            if frame < WARMUP_FRAMES {
                continue;
            }
            elapsed_ms.push(start.elapsed().as_secs_f64() * 1000.0);
            if capture_frame + 1 < frames
                && (capture_stride == 0 || !capture_frame.is_multiple_of(capture_stride))
            {
                continue;
            }
            for eye in 0..2 {
                let pixels = readback(
                    renderer.device(),
                    renderer.queue(),
                    &texture,
                    width,
                    height,
                    eye,
                );
                image::save_buffer(
                    folder.join(format!("eye{eye}_{capture_frame:04}.png")),
                    &pixels,
                    width,
                    height,
                    image::ColorType::Rgba8,
                )?;
            }
        }
        let mean = elapsed_ms.iter().sum::<f64>() / elapsed_ms.len() as f64;
        elapsed_ms.sort_by(f64::total_cmp);
        let p95 = elapsed_ms[((elapsed_ms.len() - 1) as f64 * 0.95).ceil() as usize];
        let line = format!(
            "{} zone={} feature={:?} mean_ms={:.3} p95_ms={:.3} frames={} start_sec={:.3} end_sec={:.3}\n",
            beat.beat_id,
            zone_index,
            feature_index,
            mean,
            p95,
            frames,
            start_sec,
            start_sec + (frames - 1) as f32 / 30.0
        );
        print!("{line}");
        report.push_str(&line);
    }
    std::fs::write(output.join("capture.txt"), report)?;
    Ok(())
}

fn readback(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
    tex: &wgpu::Texture,
    width: u32,
    height: u32,
    layer: u32,
) -> Vec<u8> {
    let row = width * 4;
    let padded =
        row.div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT) * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("Earth review readback"),
        size: (padded * height) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: tex,
            mip_level: 0,
            origin: wgpu::Origin3d {
                x: 0,
                y: 0,
                z: layer,
            },
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(encoder.finish()));
    let slice = buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |result| {
        let _ = tx.send(result);
    });
    device
        .poll(wgpu::PollType::wait_indefinitely())
        .expect("readback poll");
    rx.recv()
        .expect("readback callback")
        .expect("readback mapping");
    let mapped = slice.get_mapped_range().expect("mapped pixels");
    let mut pixels = Vec::with_capacity((row * height) as usize);
    for y in 0..height as usize {
        pixels.extend_from_slice(&mapped[y * padded as usize..y * padded as usize + row as usize]);
    }
    drop(mapped);
    buffer.unmap();
    pixels
}
