use std::ffi::c_void;
use std::path::Path;

use crate::RenderError;
use crate::device::{GpuContext, dx12_raw_device_and_queue, init_for_dx12_luid};
use crate::pipeline::PipelineBank;
use crate::post_pass::PostPass;
use crate::scene_pass::ScenePass;
use crate::types::RenderSceneFrame;

/// High-level DX12 + `DynamicDxc` RTX 5070 OLED HDR renderer supporting Desktop Mono,
/// OpenXR Stereo `MULTIVIEW`, offscreen readback, and foreground `cosmos_typo` HDR hooks.
pub struct CosmosRenderer {
    gpu: GpuContext,
    pipelines: PipelineBank,
    scene_pass: ScenePass,
    post_pass: PostPass,
}

impl CosmosRenderer {
    /// Initializes desktop/headless renderer targeting discrete NVIDIA RTX 5070 DX12.
    pub fn init_desktop(width: u32, height: u32) -> Result<Self, RenderError> {
        Self::init_for_dx12_luid(None, width, height, false)
    }

    /// Initializes renderer pinned to `target_luid` (e.g. from `openxr::Instance::graphics_requirements::<D3D12>`).
    pub fn init_for_dx12_luid(
        target_luid: Option<(u32, i32)>,
        width: u32,
        height: u32,
        is_stereo: bool,
    ) -> Result<Self, RenderError> {
        let gpu = init_for_dx12_luid(target_luid)?;
        let pipelines = PipelineBank::new(&gpu.device, &gpu.queue)?;
        let scene_pass = ScenePass::new(&gpu.device, &pipelines, width, height, is_stereo);
        let post_pass = PostPass::new(&gpu.device, &pipelines, &scene_pass);
        Ok(Self {
            gpu,
            pipelines,
            scene_pass,
            post_pass,
        })
    }

    #[inline]
    pub fn device(&self) -> &wgpu::Device {
        &self.gpu.device
    }

    #[inline]
    pub fn queue(&self) -> &wgpu::Queue {
        &self.gpu.queue
    }

    #[inline]
    pub fn instance(&self) -> &wgpu::Instance {
        &self.gpu.instance
    }

    #[inline]
    pub fn adapter(&self) -> &wgpu::Adapter {
        &self.gpu.adapter
    }

    #[inline]
    pub fn adapter_info(&self) -> &wgpu::AdapterInfo {
        &self.gpu.adapter_info
    }

    #[inline]
    pub fn adapter_luid(&self) -> Option<(u32, i32)> {
        self.gpu.adapter_luid
    }

    /// Extracts raw `(*mut ID3D12Device, *mut ID3D12CommandQueue)` COM pointers for OpenXR.
    pub fn dx12_raw_device_and_queue(&self) -> Result<(*mut c_void, *mut c_void), RenderError> {
        dx12_raw_device_and_queue(&self.gpu.device)
    }

    /// Shared `@group(0)` layout (`u_frame` 432 B + `u_audio` 64 B) for external HDR overlay crates like `cosmos_typo`.
    #[inline]
    pub fn shared_frame_bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.pipelines.shared_frame_bgl
    }

    /// Shared `@group(0)` bind group holding current frame's `u_frame` + `u_audio`.
    #[inline]
    pub fn shared_frame_bind_group(&self) -> &wgpu::BindGroup {
        &self.scene_pass.shared_frame_bg
    }

    /// Renders a complete cosmic frame into `target_view` (`D2` mono or 2-layer `D2Array` stereo).
    ///
    /// `draw_overlay_in_hdr(&mut rpass, is_stereo)` executes directly inside the `Rgba16Float` HDR
    /// pass (with Reversed-Z depth buffer attached) prior to tight-threshold Dual-Kawase bloom and
    /// AgX OLED tonemapping.
    #[allow(clippy::too_many_arguments)]
    pub fn render_frame<F>(
        &mut self,
        scene: &RenderSceneFrame,
        target_view: &wgpu::TextureView,
        target_format: wgpu::TextureFormat,
        width: u32,
        height: u32,
        is_stereo: bool,
        draw_overlay_in_hdr: F,
    ) where
        F: FnMut(&mut wgpu::RenderPass<'_>, bool),
    {
        let resized = self.scene_pass.resize(&self.gpu.device, width, height, is_stereo);
        self.post_pass
            .sync_to_scene(&self.gpu.device, &self.pipelines, &self.scene_pass, resized);

        let mut encoder =
            self.gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("cosmos_frame_encoder"),
                });

        self.scene_pass.execute(
            &self.gpu.device,
            &self.gpu.queue,
            &mut encoder,
            &self.pipelines,
            scene,
            draw_overlay_in_hdr,
        );

        self.post_pass.execute(
            &self.gpu.device,
            &self.gpu.queue,
            &mut encoder,
            &self.pipelines,
            &self.scene_pass,
            target_view,
            target_format,
            scene.post,
        );

        self.gpu.queue.submit(Some(encoder.finish()));
    }

    /// Renders `scene` to an offscreen `Rgba8UnormSrgb` texture and reads back tightly packed
    /// `(width * height * 4)` RGBA8 pixels on CPU (used for visual headless verification).
    pub fn render_to_offscreen_rgba8<F>(
        &mut self,
        scene: &RenderSceneFrame,
        width: u32,
        height: u32,
        draw_overlay_in_hdr: F,
    ) -> Result<Vec<u8>, RenderError>
    where
        F: FnMut(&mut wgpu::RenderPass<'_>, bool),
    {
        let width = width.max(1);
        let height = height.max(1);
        let format = wgpu::TextureFormat::Rgba8UnormSrgb;

        let tex = self.gpu.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("offscreen_srgb_target"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let view = tex.create_view(&wgpu::TextureViewDescriptor::default());

        self.render_frame(
            scene,
            &view,
            format,
            width,
            height,
            false,
            draw_overlay_in_hdr,
        );

        let unpadded_bpr = width * 4;
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let padded_bpr = unpadded_bpr.div_ceil(align) * align;
        let buf_size = (padded_bpr * height) as u64;

        let readback = self.gpu.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("offscreen_readback_buf"),
            size: buf_size,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        let mut encoder =
            self.gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("offscreen_copy_encoder"),
                });
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &tex,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &readback,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(padded_bpr),
                    rows_per_image: Some(height),
                },
            },
            wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
        );
        self.gpu.queue.submit(Some(encoder.finish()));

        let slice = readback.slice(..);
        let (tx, rx) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |res| {
            let _ = tx.send(res);
        });
        self.gpu
            .device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| RenderError::Readback(format!("device poll failed: {e}")))?;
        rx.recv()
            .map_err(|e| RenderError::Readback(format!("map channel failed: {e}")))?
            .map_err(|e| RenderError::Readback(format!("buffer map failed: {e}")))?;

        let data = slice
            .get_mapped_range()
            .map_err(|e| RenderError::Readback(format!("mapped range failed: {e}")))?;
        let mut pixels = Vec::with_capacity((unpadded_bpr * height) as usize);
        for row in 0..height as usize {
            let start = row * padded_bpr as usize;
            let end = start + unpadded_bpr as usize;
            pixels.extend_from_slice(&data[start..end]);
        }
        drop(data);
        readback.unmap();
        Ok(pixels)
    }

    /// Renders `scene` offscreen and writes a PNG file at `path`.
    pub fn render_to_png<F>(
        &mut self,
        path: &Path,
        scene: &RenderSceneFrame,
        width: u32,
        height: u32,
        draw_overlay_in_hdr: F,
    ) -> Result<Vec<u8>, RenderError>
    where
        F: FnMut(&mut wgpu::RenderPass<'_>, bool),
    {
        let pixels = self.render_to_offscreen_rgba8(scene, width, height, draw_overlay_in_hdr)?;
        let img = image::RgbaImage::from_raw(width, height, pixels.clone())
            .ok_or_else(|| RenderError::Readback("failed to construct RgbaImage".into()))?;
        img.save(path)
            .map_err(|e| RenderError::Readback(format!("save png '{}': {e}", path.display())))?;
        Ok(pixels)
    }
}
