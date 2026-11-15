//! Embedded, provenance-tracked Earth maps. Mips are baked in linear light so city
//! lights and thin coastlines do not disappear or shimmer when the viewer moves.
use crate::RenderError;
use std::sync::Mutex;

const WIDTH: u32 = 8192;
const HEIGHT: u32 = 4096;
const MIP_COUNT: u32 = 14;
const MAPS: [&[u8]; 4] = [
    include_bytes!("../../../assets/textures/earth/day.png"),
    include_bytes!("../../../assets/textures/earth/night.png"),
    include_bytes!("../../../assets/textures/earth/cloud.png"),
    include_bytes!("../../../assets/textures/earth/relief.png"),
];

// Four 8K RGBA atlases occupy 1 GiB while decoded. A renderer-test process can
// create several devices concurrently; serialize these batches through upload
// so four contexts cannot multiply the transient decoded image allocation.
static DECODE_BATCH: Mutex<()> = Mutex::new(());

fn decode_maps() -> Result<Vec<image::RgbaImage>, RenderError> {
    std::thread::scope(|scope| {
        // Exactly one worker per independent embedded map, never one per mip.
        let workers: Vec<_> = MAPS
            .iter()
            .enumerate()
            .map(|(layer, bytes)| {
                scope.spawn(move || {
                    let atlas = image::load_from_memory_with_format(bytes, image::ImageFormat::Png)
                        .map_err(|e| RenderError::PipelineInit(format!("Earth map {layer}: {e}")))?
                        .into_rgba8();
                    if atlas.dimensions() != (WIDTH, WIDTH) {
                        return Err(RenderError::PipelineInit(format!(
                            "Earth map {layer}: invalid mip atlas"
                        )));
                    }
                    Ok(atlas)
                })
            })
            .collect();
        workers
            .into_iter()
            .enumerate()
            .map(|(layer, worker)| {
                worker.join().map_err(|_| {
                    RenderError::PipelineInit(format!("Earth map {layer}: decoder worker panicked"))
                })?
            })
            .collect()
    })
}

pub(crate) fn upload(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> Result<(wgpu::Texture, wgpu::TextureView, wgpu::Sampler), RenderError> {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("Earth NASA material array"),
        size: wgpu::Extent3d {
            width: WIDTH,
            height: HEIGHT,
            depth_or_array_layers: 4,
        },
        mip_level_count: MIP_COUNT,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let decode_guard = DECODE_BATCH
        .lock()
        .map_err(|_| RenderError::PipelineInit("Earth map decode batch lock poisoned".into()))?;
    for (layer, atlas) in decode_maps()?.into_iter().enumerate() {
        let mut y = 0;
        for mip in 0..MIP_COUNT {
            let width = (WIDTH >> mip).max(1);
            let height = (HEIGHT >> mip).max(1);
            queue.write_texture(
                wgpu::TexelCopyTextureInfo {
                    texture: &texture,
                    mip_level: mip,
                    origin: wgpu::Origin3d {
                        x: 0,
                        y: 0,
                        z: layer as u32,
                    },
                    aspect: wgpu::TextureAspect::All,
                },
                &atlas.as_raw()[(y * WIDTH * 4) as usize..],
                wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(WIDTH * 4),
                    rows_per_image: Some(height),
                },
                wgpu::Extent3d {
                    width,
                    height,
                    depth_or_array_layers: 1,
                },
            );
            y += height;
        }
    }
    // The consuming iteration drops each atlas immediately after its mip copies.
    drop(decode_guard);
    let view = texture.create_view(&wgpu::TextureViewDescriptor {
        dimension: Some(wgpu::TextureViewDimension::D2Array),
        ..Default::default()
    });
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("Earth longitude wrap latitude clamp"),
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::ClampToEdge,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        mipmap_filter: wgpu::MipmapFilterMode::Linear,
        anisotropy_clamp: 8,
        ..Default::default()
    });
    Ok((texture, view, sampler))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nasa_maps_share_geography_and_linear_mip_contract() {
        let _decode_guard = DECODE_BATCH.lock().unwrap();
        let maps = decode_maps().unwrap();
        assert!(maps.iter().all(|m| m.dimensions() == (WIDTH, WIDTH)));
        let sample = |layer: usize, lon: f32, lat: f32, channel: usize| {
            let x = ((lon + 180.0) / 360.0 * WIDTH as f32) as u32;
            let y = ((90.0 - lat) / 180.0 * HEIGHT as f32) as u32;
            let c = maps[layer].get_pixel(x, y)[channel] as f32 / 255.0;
            if c <= 0.04045 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        assert!(sample(3, -140.0, -10.0, 1) > 0.95, "Pacific is water");
        assert!(sample(3, 20.0, 22.0, 1) < 0.05, "Sahara is land");
        assert!(
            sample(0, 20.0, 22.0, 0) > sample(0, -140.0, -10.0, 0) * 10.0,
            "desert/ocean albedo differ"
        );
        assert!(
            sample(3, 85.0, 30.0, 0) > sample(3, 20.0, 22.0, 0),
            "Tibetan relief exceeds Sahara"
        );
        // Pinned CPC frame alpha, not its near-white cloud-color RGB. These
        // separate clear/cloudy regions also catch flipped latitude/longitude.
        assert!((0.25..0.40).contains(&sample(2, 0.0, 40.0, 0)));
        assert!(sample(2, -110.0, 30.0, 0) < 0.01, "clear northern Mexico");
        assert!(
            sample(2, 0.0, 80.0, 0) > 0.10,
            "missing CPC pole uses older global fill"
        );
        // Last mip is the whole-planet average, so a lost/misordered atlas level
        // would turn oceans off or light the complete night hemisphere at distance.
        let ocean_average = maps[3].get_pixel(0, WIDTH - 1)[1] as f32 / 255.0;
        assert!((0.78..0.86).contains(&ocean_average));
        assert!(
            maps[1].get_pixel(0, WIDTH - 1)[0] < 25,
            "night emission stays sparse at distance"
        );
    }
}
