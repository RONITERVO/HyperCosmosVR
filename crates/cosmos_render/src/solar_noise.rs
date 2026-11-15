//! Immutable periodic random lattice shared by solar volumetric materials.
//! Texture hardware interpolates its corners; the shader warps fractional
//! coordinates with the original quintic curve to retain C2 continuity.

pub(crate) const DIM: u32 = 128;

pub(crate) fn shader_source() -> String {
    [
        include_str!("../../../shaders/common_oled.wgsl"),
        include_str!("../../../shaders/solar_noise_lattice.wgsl"),
    ]
    .join("\n")
}

pub(crate) fn create_lattice(
    device: &wgpu::Device,
    queue: &wgpu::Queue,
) -> (wgpu::Texture, wgpu::TextureView, wgpu::Sampler) {
    let texture = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("128-cubed authored solar noise lattice"),
        size: wgpu::Extent3d {
            width: DIM,
            height: DIM,
            depth_or_array_layers: DIM,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D3,
        format: wgpu::TextureFormat::Rgba16Float,
        usage: wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::COPY_DST,
        view_formats: &[],
    });
    let view = texture.create_view(&Default::default());
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("initialize immutable solar noise"),
        source: wgpu::ShaderSource::Wgsl(shader_source().into()),
    });
    let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
        label: Some("initialize solar lattice once"),
        layout: None,
        module: &shader,
        entry_point: Some("initialize_solar_noise"),
        compilation_options: Default::default(),
        cache: None,
    });
    let bindings = device.create_bind_group(&wgpu::BindGroupDescriptor {
        label: Some("solar lattice initialization target"),
        layout: &pipeline.get_bind_group_layout(0),
        entries: &[wgpu::BindGroupEntry {
            binding: 2,
            resource: wgpu::BindingResource::TextureView(&view),
        }],
    });
    let mut encoder = device.create_command_encoder(&Default::default());
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        pass.set_pipeline(&pipeline);
        pass.set_bind_group(0, &bindings, &[]);
        pass.dispatch_workgroups(DIM / 8, DIM / 8, DIM / 4);
    }
    queue.submit(Some(encoder.finish()));
    let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
        label: Some("periodic solar lattice trilinear sampler"),
        address_mode_u: wgpu::AddressMode::Repeat,
        address_mode_v: wgpu::AddressMode::Repeat,
        address_mode_w: wgpu::AddressMode::Repeat,
        mag_filter: wgpu::FilterMode::Linear,
        min_filter: wgpu::FilterMode::Linear,
        ..Default::default()
    });
    (texture, view, sampler)
}

#[cfg(test)]
#[path = "solar_noise_tests.rs"]
mod tests;
