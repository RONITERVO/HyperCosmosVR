//! Explicit developer profiling. Normal rendering allocates no queries or
//! readback resources and never waits for profiling data.
use crate::RenderError;

const QUERY_COUNT: u32 = 9;
const LABELS: [&str; 8] = [
    "compute_and_clear",
    "sky",
    "surface",
    "absorption",
    "volume",
    "emission",
    "overlays",
    "post",
];

pub(crate) struct GpuProfiler {
    pub queries: wgpu::QuerySet,
    resolved: wgpu::Buffer,
    readback: wgpu::Buffer,
}

impl GpuProfiler {
    pub fn new(device: &wgpu::Device) -> Option<Self> {
        if !std::env::var("HYPERCOSMOS_GPU_PROFILE").is_ok_and(|value| value == "1") {
            return None;
        }
        let size = u64::from(QUERY_COUNT) * 8;
        Some(Self {
            queries: device.create_query_set(&wgpu::QuerySetDescriptor {
                label: Some("opt-in frame GPU timestamps"),
                ty: wgpu::QueryType::Timestamp,
                count: QUERY_COUNT,
            }),
            resolved: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("GPU timestamp resolve"),
                size,
                usage: wgpu::BufferUsages::QUERY_RESOLVE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: false,
            }),
            readback: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("GPU timestamp readback"),
                size,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            }),
        })
    }

    pub fn finish(&self, encoder: &mut wgpu::CommandEncoder) {
        encoder.write_timestamp(&self.queries, 8);
        encoder.resolve_query_set(&self.queries, 0..QUERY_COUNT, &self.resolved, 0);
        encoder.copy_buffer_to_buffer(
            &self.resolved,
            0,
            &self.readback,
            0,
            u64::from(QUERY_COUNT) * 8,
        );
    }

    pub fn read(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Vec<(&'static str, f64)>, RenderError> {
        let (tx, rx) = std::sync::mpsc::channel();
        self.readback
            .slice(..)
            .map_async(wgpu::MapMode::Read, move |result| {
                let _ = tx.send(result);
            });
        device
            .poll(wgpu::PollType::wait_indefinitely())
            .map_err(|e| RenderError::Readback(e.to_string()))?;
        rx.recv()
            .map_err(|e| RenderError::Readback(e.to_string()))?
            .map_err(|e| RenderError::Readback(e.to_string()))?;
        let bytes = self
            .readback
            .slice(..)
            .get_mapped_range()
            .map_err(|e| RenderError::Readback(e.to_string()))?;
        let stamps: &[u64] = bytemuck::cast_slice(&bytes);
        let period_ms = f64::from(queue.get_timestamp_period()) / 1_000_000.0;
        let samples = LABELS
            .iter()
            .enumerate()
            .map(|(i, label)| {
                (
                    *label,
                    stamps[i + 1].wrapping_sub(stamps[i]) as f64 * period_ms,
                )
            })
            .collect();
        drop(bytes);
        self.readback.unmap();
        Ok(samples)
    }
}
