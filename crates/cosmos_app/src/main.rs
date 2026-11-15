use std::path::PathBuf;

use anyhow::Result;
use cosmos_app::desktop::run_desktop_loop_with_journey;
use cosmos_app::engine::LaunchJourneyConfig;
use cosmos_app::headless::run_headless_render;
use cosmos_app::vr::try_run_openxr_session_with_journey;
use cosmos_xr::{probe_openxr, XrAvailability};

struct CliArgs {
    force_desktop: bool,
    prefer_hdr_surface: bool,
    headless_out: Option<PathBuf>,
    benchmark_sweep: bool,
    width: u32,
    height: u32,
    journey: LaunchJourneyConfig,
}

fn parse_args() -> CliArgs {
    let mut args = CliArgs {
        force_desktop: false,
        prefer_hdr_surface: false,
        headless_out: None,
        benchmark_sweep: false,
        width: 1600,
        height: 900,
        journey: LaunchJourneyConfig::default(),
    };

    let mut it = std::env::args().skip(1);
    while let Some(arg) = it.next() {
        match arg.as_str() {
            "--desktop" => args.force_desktop = true,
            "--hdr-surface" => args.prefer_hdr_surface = true,
            "--benchmark-sweep" => args.benchmark_sweep = true,
            "--autoplay-journey" => args.journey.autoplay = true,
            "--journey" => {
                if let Some(v) = it.next() {
                    args.journey.journey_id_or_path = Some(v);
                }
            }
            "--headless-render" => {
                let dir = it
                    .next()
                    .map(PathBuf::from)
                    .unwrap_or_else(|| PathBuf::from("artifacts/verification_renders"));
                args.headless_out = Some(dir);
            }
            "--width" => {
                if let Some(v) = it.next().and_then(|s| s.parse::<u32>().ok()) {
                    args.width = v.max(64);
                }
            }
            "--height" => {
                if let Some(v) = it.next().and_then(|s| s.parse::<u32>().ok()) {
                    args.height = v.max(64);
                }
            }
            "--help" | "-h" => {
                println!(
                    "HyperCosmosVR (RTX 5070 DX12 + OpenXR VR + Desktop HDR)\n\
                     Usage:\n\
                       cosmos_app [--desktop] [--hdr-surface] [--journey <id|path|uri>] [--autoplay-journey]\n\
                       cosmos_app --headless-render <out_dir> [--benchmark-sweep] [--width W] [--height H]\n\
                     Controls (Desktop): Hover scan focus, Scroll dive, Left-drag orbit, F+Mouse free-look,\n\
                       WASD/QE 6DoF, Right-Hold/Tab Wrist Resonance Ring, [ / ] speed, L locale EN/FI, Home reset,\n\
                       F1 help, Space/J tour pause/resume, N/P beat (Shift+N/P chapter), Esc explore/close help,\n\
                       R/+WPT, Shift+R live trace, Ctrl+S/F9 export, Enter/Backspace FOLLOW/EXIT, Alt+F4 quit."
                );
                std::process::exit(0);
            }
            _ => {}
        }
    }

    if args.benchmark_sweep && args.headless_out.is_none() {
        args.headless_out = Some(PathBuf::from("artifacts/verification_renders"));
    }

    args
}

fn main() -> Result<()> {
    let args = parse_args();

    if let Some(ref out_dir) = args.headless_out {
        let (shots, bench) =
            run_headless_render(out_dir, args.width, args.height, args.benchmark_sweep)?;
        println!(
            "[HyperCosmosVR] Headless RTX 5070 rendered {} PNG captures into '{}':",
            shots.len(),
            out_dir.display()
        );
        for s in &shots {
            println!(
                "  - {:36} exp={:02} u={:+5.1} ms={:6.2} glyphs={:4} tethers={:2} black={}px non_black={}px",
                s.filename,
                s.experience_id,
                s.scale_exp,
                s.render_ms,
                s.glyph_count,
                s.tether_count,
                s.pure_black_pixels,
                s.non_black_pixels
            );
        }
        if let Some(b) = bench {
            println!(
                "[HyperCosmosVR] Benchmark Sweep ({} frames, '{}'): min={:.2}ms mean={:.2}ms ({:.1} FPS) p99={:.2}ms max={:.2}ms",
                b.frame_count,
                b.adapter_name,
                b.min_ms,
                b.mean_ms,
                1000.0 / b.mean_ms.max(1e-4),
                b.p99_ms,
                b.max_ms
            );
        }
        return Ok(());
    }

    if !args.force_desktop {
        match probe_openxr("HyperCosmosVR", 1) {
            avail @ XrAvailability::Available { .. } => {
                if try_run_openxr_session_with_journey(avail, args.journey.clone())? {
                    return Ok(());
                }
            }
            XrAvailability::NoHeadset {
                reason,
                runtime_name,
                ..
            } => {
                eprintln!(
                    "[HyperCosmosVR] OpenXR ({}) inactive ({reason:?}); launching Desktop HDR window.",
                    runtime_name.as_deref().unwrap_or("none")
                );
            }
        }
    }

    run_desktop_loop_with_journey(args.prefer_hdr_surface, args.journey)
}

