use std::ffi::c_void;
use std::path::{Path, PathBuf};

use windows_core::Interface;

use crate::RenderError;

const NVIDIA_PCI_VENDOR: u32 = 0x10DE;

/// Initialized RTX 5070 DX12 (`DynamicDxc`) GPU device context shared across Desktop,
/// Headless, and OpenXR D3D12 stereo sessions.
pub struct GpuContext {
    pub instance: wgpu::Instance,
    pub adapter: wgpu::Adapter,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter_info: wgpu::AdapterInfo,
    pub adapter_luid: Option<(u32, i32)>,
    pub dxc_path: PathBuf,
}

impl std::fmt::Debug for GpuContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GpuContext")
            .field("adapter_name", &self.adapter_info.name)
            .field("vendor", &format_args!("{:#06X}", self.adapter_info.vendor))
            .field("device_type", &self.adapter_info.device_type)
            .field("adapter_luid", &self.adapter_luid)
            .field("dxc_path", &self.dxc_path)
            .finish()
    }
}

/// Locates `dxcompiler.dll` (and ensures sibling `dxil.dll` exists when alongside).
///
/// Search order:
/// 1. Same directory as the running executable (production bundle).
/// 2. `C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\dxcompiler.dll`
/// 3. Highest versioned subdirectory under `C:\Program Files (x86)\Windows Kits\10\bin\*\x64`.
/// 4. Optional `WGPU_DXC_PATH` environment override.
pub fn locate_dxc_compiler() -> Result<PathBuf, RenderError> {
    if let Ok(env_path) = std::env::var("WGPU_DXC_PATH") {
        let p = PathBuf::from(env_path);
        if p.is_file() {
            return Ok(p);
        }
    }

    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let candidate = dir.join("dxcompiler.dll");
        if candidate.is_file() {
            return Ok(candidate);
        }
    }

    let sdk_preferred =
        Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin\10.0.26100.0\x64\dxcompiler.dll");
    if sdk_preferred.is_file() {
        return Ok(sdk_preferred.to_path_buf());
    }

    let sdk_bin = Path::new(r"C:\Program Files (x86)\Windows Kits\10\bin");
    if let Ok(entries) = std::fs::read_dir(sdk_bin) {
        let mut dirs: Vec<PathBuf> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| p.is_dir())
            .collect();
        dirs.sort();
        dirs.reverse();
        for dir in dirs {
            let candidate = dir.join("x64").join("dxcompiler.dll");
            if candidate.is_file() {
                return Ok(candidate);
            }
        }
    }

    Err(RenderError::DxcNotFound)
}

/// Extracts the Windows DXGI `LUID` `(LowPart, HighPart)` for a DX12 `wgpu::Adapter`.
pub fn adapter_dxgi_luid(adapter: &wgpu::Adapter) -> Option<(u32, i32)> {
    let hal_adapter = unsafe { adapter.as_hal::<wgpu_hal::api::Dx12>()? };
    let desc = unsafe { hal_adapter.raw_adapter().GetDesc2() }.ok()?;
    Some((desc.AdapterLuid.LowPart, desc.AdapterLuid.HighPart))
}

/// Extracts raw `(*mut ID3D12Device, *mut ID3D12CommandQueue)` COM pointers for OpenXR
/// `SessionCreateInfoD3D12` initialization without creating a dependency on `openxr`.
pub fn dx12_raw_device_and_queue(
    device: &wgpu::Device,
) -> Result<(*mut c_void, *mut c_void), RenderError> {
    let hal_dev = unsafe { device.as_hal::<wgpu_hal::api::Dx12>() }
        .ok_or_else(|| RenderError::DeviceInit("device is not backed by DX12 HAL".into()))?;
    let raw_dev = hal_dev.raw_device().as_raw();
    let raw_queue = hal_dev.raw_queue().as_raw();
    if raw_dev.is_null() || raw_queue.is_null() {
        return Err(RenderError::DeviceInit(
            "DX12 HAL returned null ID3D12Device or ID3D12CommandQueue".into(),
        ));
    }
    Ok((raw_dev, raw_queue))
}

/// Initializes a DX12 `DynamicDxc` `wgpu::Instance`, selects the NVIDIA RTX 5070 (or exact OpenXR
/// target LUID when `target_luid` is `Some((low, high))`), and opens `Device` + `Queue` with
/// `MULTIVIEW` enabled.
pub fn init_for_dx12_luid(target_luid: Option<(u32, i32)>) -> Result<GpuContext, RenderError> {
    let dxc_path = locate_dxc_compiler()?;
    let dxc_str = dxc_path
        .to_str()
        .ok_or_else(|| RenderError::DeviceInit("non-UTF8 path to dxcompiler.dll".into()))?
        .to_owned();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::DX12,
        backend_options: wgpu::BackendOptions {
            dx12: wgpu::Dx12BackendOptions {
                shader_compiler: wgpu::Dx12Compiler::DynamicDxc { dxc_path: dxc_str },
                ..Default::default()
            },
            ..Default::default()
        },
        ..wgpu::InstanceDescriptor::new_without_display_handle()
    });

    let adapters = pollster::block_on(instance.enumerate_adapters(wgpu::Backends::DX12));
    if adapters.is_empty() {
        return Err(RenderError::AdapterNotFound(
            "no DX12 adapters enumerated".into(),
        ));
    }

    let selected = select_adapter(adapters, target_luid)?;
    let adapter_info = selected.get_info();
    let adapter_luid = adapter_dxgi_luid(&selected);

    let supported = selected.features();
    if !supported.contains(wgpu::Features::MULTIVIEW) {
        return Err(RenderError::DeviceInit(format!(
            "selected adapter '{}' lacks Features::MULTIVIEW (DXC path: {})",
            adapter_info.name,
            dxc_path.display()
        )));
    }

    let mut required_features =
        wgpu::Features::MULTIVIEW | wgpu::Features::TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES;
    for opt in [
        wgpu::Features::SHADER_F16,
        wgpu::Features::SUBGROUP,
        wgpu::Features::TIMESTAMP_QUERY,
    ] {
        if supported.contains(opt) {
            required_features |= opt;
        }
    }
    if std::env::var("HYPERCOSMOS_GPU_PROFILE").is_ok_and(|value| value == "1") {
        let profiling = wgpu::Features::TIMESTAMP_QUERY
            | wgpu::Features::TIMESTAMP_QUERY_INSIDE_ENCODERS
            | wgpu::Features::TIMESTAMP_QUERY_INSIDE_PASSES;
        if !supported.contains(profiling) {
            return Err(RenderError::DeviceInit("GPU profiling requested but in-pass timestamps are unsupported".into()));
        }
        required_features |= profiling;
    }

    let mut required_limits = wgpu::Limits::default();
    let adapter_limits = selected.limits();
    if adapter_limits.max_multiview_view_count < 2 {
        return Err(RenderError::DeviceInit(format!(
            "selected adapter '{}' reports max_multiview_view_count={} (< 2)",
            adapter_info.name, adapter_limits.max_multiview_view_count
        )));
    }
    required_limits.max_multiview_view_count = adapter_limits.max_multiview_view_count.min(4);
    required_limits.max_storage_buffer_binding_size =
        adapter_limits.max_storage_buffer_binding_size.min(512 * 1024 * 1024);
    required_limits.max_buffer_size = adapter_limits.max_buffer_size.min(1024 * 1024 * 1024);
    required_limits.max_texture_dimension_2d = adapter_limits.max_texture_dimension_2d.min(16384);
    required_limits.max_compute_invocations_per_workgroup =
        adapter_limits.max_compute_invocations_per_workgroup.min(1024);
    required_limits.max_compute_workgroup_size_x =
        adapter_limits.max_compute_workgroup_size_x.min(1024);

    let (device, queue) = pollster::block_on(selected.request_device(&wgpu::DeviceDescriptor {
        label: Some("HyperCosmosVR RTX 5070 DX12"),
        required_features,
        required_limits,
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
    }))
    .map_err(|e| RenderError::DeviceInit(e.to_string()))?;

    Ok(GpuContext {
        instance,
        adapter: selected,
        device,
        queue,
        adapter_info,
        adapter_luid,
        dxc_path,
    })
}

fn select_adapter(
    adapters: Vec<wgpu::Adapter>,
    target_luid: Option<(u32, i32)>,
) -> Result<wgpu::Adapter, RenderError> {
    if let Some(target) = target_luid {
        for adapter in adapters {
            if adapter_dxgi_luid(&adapter) == Some(target) {
                return Ok(adapter);
            }
        }
        return Err(RenderError::AdapterNotFound(format!(
            "no DX12 adapter matched OpenXR target LUID ({}, {})",
            target.0, target.1
        )));
    }

    // Prefer discrete NVIDIA GPU (`0x10DE`), avoiding AMD iGPU / WARP.
    let mut fallback_discrete: Option<wgpu::Adapter> = None;
    for adapter in adapters {
        let info = adapter.get_info();
        if info.backend == wgpu::Backend::Dx12 && info.device_type == wgpu::DeviceType::DiscreteGpu {
            if info.vendor == NVIDIA_PCI_VENDOR {
                return Ok(adapter);
            }
            if fallback_discrete.is_none() {
                fallback_discrete = Some(adapter);
            }
        }
    }

    fallback_discrete.ok_or_else(|| {
        RenderError::AdapterNotFound("no discrete DX12 GPU adapter available".into())
    })
}
