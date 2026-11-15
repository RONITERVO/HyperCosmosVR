//! Caption-free stereo evidence from the shipped Highlights camera poses.
//! cargo run --release -p cosmos_app --example earth_review -- target/earth_review 2560 1
//! Third argument is frames per stop (481 includes both endpoints of 16 seconds at the default 30 Hz).
//! Optional fourth argument filters beat IDs (comma-separated substrings, or all).
//! Optional fifth argument sets the first simulation time in seconds (default 13.2 for a still, 0 for a sequence).
//! Optional sixth argument sets eye height (default 16:9); use 2048 1 all 13.2 2048 for square stereo.
//! Optional seventh argument is capture stride. Zero captures only the last frame,
//! allowing consecutive timed frames without PNG/readback gaps (e.g. 2560 120 all 9.2 1440 0).
//! Optional eighth argument selects a catalog experience (default 3/Earth; 4/Solar).
//! Optional ninth argument: static (default), head, orbit, head-orbit, head-look-up,
//! or solar-review (head-look-up for the eruption, head-orbit for other beats).
//! Optional tenth argument: video, to stream every frame to per-eye CPU-encoded
//! MP4 files independently of PNG stride, or no-video (default). Encoding runs are not timing benchmarks.
//! Optional eleventh argument sets positive finite synthetic capture Hz (default 30).
//! This sets simulation/motion steps and encoded FPS, not measured native headset cadence.
//! HYPERCOSMOS_GPU_PROFILE=1 adds measured-frame means for named GPU passes;
//! timestamp readback occurs after the CPU submission/wait sample is recorded.
//! Motion is deterministic capture time; all parameters/frusta and frame poses
//! are recorded. These are synthetic review movements, not headset measurements.
use cosmos_app::assets::UniverseAssets;
use cosmos_journey::{BeatTargetSelector, CosmicHighlightsPlugin, StoryJourneyPlugin};
use cosmos_math::{ScalePose64, camera_view_matrix, reversed_z_infinite_perspective_rh};
use cosmos_render::{CosmosRenderer, EyeView, RenderCamera, SceneBuildInput, UniverseSceneBuilder};
use glam::{DMat3, DQuat, DVec3, Mat4, Vec3};
use std::{collections::BTreeMap, io::Write, path::{Path, PathBuf}, process::{Child, Command, Stdio}, time::Instant};

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
    let experience: u8 = args.get(8).map(|s| s.parse()).transpose()?.unwrap_or(3);
    let motion = ReviewMotion::parse(args.get(9).map(String::as_str).unwrap_or("static"))?;
    let video = match args.get(10).map(String::as_str) {
        None | Some("no-video") => false,
        Some("video") => true,
        Some(_) => anyhow::bail!("optional tenth argument must be video or no-video"),
    };
    let capture_hz = parse_capture_hz(args.get(11).map(String::as_str))?;
    anyhow::ensure!(motion != ReviewMotion::SolarReview || experience == 4,
        "solar-review motion requires experience4");
    let ffmpeg_version = if video {
        anyhow::ensure!(width.is_multiple_of(2) && height.is_multiple_of(2),
            "yuv420p video requires even eye width and height");
        Some(ffmpeg_version()?)
    } else { None };
    anyhow::ensure!(
        motion == ReviewMotion::Static || matches!(experience, 3 | 4),
        "moving-camera review currently requires a physical Earth/Solar environment (experience3 or4)"
    );
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
    let aspect = width as f64 / height as f64;
    let tan_half_y = (36.0_f64.to_radians()).tan();
    report.push_str(&format!(
        "experience={experience}\nmotion={motion:?}\nsynthetic_capture_hz={capture_hz}\nvertical_fov_degrees=72\nhorizontal_fov_degrees={:.6}\nnear_plane_m=0.002\nfar_plane=infinite reversed-Z\nfrustum_tangents_left_right_bottom_top={:.9},{:.9},{:.9},{:.9}\nipd_m=0.064\nhead_yaw_deg=10*sin(2*pi*t/8); head_pitch_deg=4*sin(2*pi*t/11); roll=0\nhead_translation_local_m=(0.10*sin(2*pi*t/6),0.04*sin(2*pi*t/9),0.06*sin(2*pi*t/8))\norbit_yaw_deg=8*sin(2*pi*t/16); axis=body+Y; radius=authored distance; look_target=authored fixed world target\nmotion_t_seconds=frame/{capture_hz} (independent of simulation start); unused mode components are zero\nhead translation converted from meters using sector scale, not observer zoom\nsynthetic_capture_hz sets simulation steps and encoded playback; not measured native headset FPS\n",
        2.0 * (tan_half_y * aspect).atan().to_degrees(),
        -tan_half_y * aspect, tan_half_y * aspect, -tan_half_y, tan_half_y
    ));
    report.push_str("head_look_up_pitch_deg=15*(1-cos(2*pi*t/60)); replaces normal pitch only in head-look-up mode; capture-only voluntary-look illustration\n");
    report.push_str(&format!("video_encoding_active={video}\n"));
    report.push_str(&format!("gpu_profile_requested={}\ngpu_profile_readback=after CPU frame-time sample; warmups excluded; timestamps do not establish native headset cadence\n",
        std::env::var("HYPERCOSMOS_GPU_PROFILE").is_ok_and(|value| value == "1")));
    if let Some(version) = ffmpeg_version {
        report.push_str(&format!("ffmpeg={version}\nvideo=per-eye MP4; CPU libx264; preset=fast; crf=18; pix_fmt=yuv420p; encoder_threads_per_eye=4; filter_threads_per_eye=1; fps={capture_hz}\nvideo_color=BT.709 matrix/primaries; sRGB transfer; limited YUV range; lossy review copy, PNG retains original RGBA\nvideo_records_all_frames_independent_of_png_stride=true\nencoding-run timings exclude readback/write calls but include concurrent CPU encoder contention; NOT an isolated rendering benchmark\n"));
    }
    std::fs::write(output.join("capture.txt"), &report)?;
    for beat in journey.chapters.iter().flat_map(|chapter| &chapter.beats) {
        if beat.target.experience_id() != Some(experience) {
            continue;
        }
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
        let sector_scale = assets
            .spatial
            .tree
            .get(pose.sector_id)
            .ok_or_else(|| anyhow::anyhow!("missing observing sector"))?
            .scale_exp;
        let body_center = if experience == 4 {
            assets
                .spatial
                .find_zone_entity(4, zone_index)
                .map(|e| e.local_pos)
                .unwrap_or(DVec3::ZERO)
        } else {
            DVec3::ZERO
        };
        let look_target = beat
            .look_at_pose
            .as_ref()
            .map(|p| p.to_pose64(&beat.beat_id))
            .transpose()?
            .map(|p| p.local_pos)
            .unwrap_or(body_center);
        let mut builder = UniverseSceneBuilder::new();
        let folder = output.join(&beat.beat_id);
        std::fs::create_dir_all(&folder)?;
        let beat_motion = motion.for_beat(&beat.beat_id);
        let mut videos = if video {
            Some([VideoEncoder::start(&folder, 0, width, height, capture_hz)?,
                VideoEncoder::start(&folder, 1, width, height, capture_hz)?])
        } else { None };
        let mut elapsed_ms = Vec::new();
        let mut gpu_pass_ms: BTreeMap<&'static str, (f64, u32)> = BTreeMap::new();
        let mut poses = String::from(
            "frame,motion_sec,simulation_sec,local_x,local_y,local_z,scale_exp,qx,qy,qz,qw\n",
        );
        report.push_str(&format!("{} resolved_motion={:?} motion_body_center_sector={:?} motion_look_target_sector={:?} sector_scale_exp={}\n",
            beat.beat_id, beat_motion, body_center, look_target, sector_scale));
        // Warm the pipelines before reporting reference timings. These include
        // submission/wait overhead and are not native-headset GPU timings.
        const WARMUP_FRAMES: u32 = 8;
        for frame in 0..frames + WARMUP_FRAMES {
            let capture_frame = frame.saturating_sub(WARMUP_FRAMES);
            let motion_sec = f64::from(capture_frame) / capture_hz;
            let moved_pose = review_pose(
                pose,
                body_center,
                look_target,
                sector_scale,
                beat_motion,
                motion_sec,
            );
            // Rebuild both eye origins, view-projections and inverse matrices
            // every frame. Head rotation must not leave the IPD axis behind.
            let camera = stereo_camera(moved_pose, width, height);
            let mut input = SceneBuildInput::desktop_default(moved_pose, camera, width, height);
            input.active_experience_id = experience;
            input.time_sec = (f64::from(start_sec) + motion_sec) as f32;
            input.dt_sec = capture_hz.recip() as f32;
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
            if let Some(passes) = renderer.read_gpu_profile_ms()? {
                for (label, milliseconds) in passes {
                    let (sum, count) = gpu_pass_ms.entry(label).or_default();
                    *sum += milliseconds;
                    *count += 1;
                }
            }
            poses.push_str(&format!("{capture_frame},{motion_sec:.9},{:.9},{:.16},{:.16},{:.16},{:.9},{:.16},{:.16},{:.16},{:.16}\n",
                input.time_sec, moved_pose.local_pos.x, moved_pose.local_pos.y, moved_pose.local_pos.z,
                moved_pose.scale_exp, moved_pose.orient.x, moved_pose.orient.y, moved_pose.orient.z, moved_pose.orient.w));
            let save_png = capture_frame + 1 == frames
                || (capture_stride > 0 && capture_frame.is_multiple_of(capture_stride));
            if !save_png && videos.is_none() {
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
                if let Some(encoders) = &mut videos {
                    encoders[eye as usize].write_frame(&pixels)?;
                }
                if save_png {
                    image::save_buffer(
                        folder.join(format!("eye{eye}_{capture_frame:04}.png")),
                        &pixels, width, height, image::ColorType::Rgba8,
                    )?;
                }
            }
        }
        if let Some(encoders) = videos {
            for (eye, encoder) in encoders.into_iter().enumerate() {
                let encoded = encoder.finish()?;
                anyhow::ensure!(encoded == frames, "eye{eye} video frame count mismatch");
                report.push_str(&format!("{} eye{eye}_video_frames={encoded} encoder_exit=success\n", beat.beat_id));
            }
        }
        std::fs::write(folder.join("frame_poses.csv"), poses)?;
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
            f64::from(start_sec) + f64::from(frames - 1) / capture_hz
        );
        print!("{line}");
        report.push_str(&line);
        for (label, (sum, samples)) in gpu_pass_ms {
            let line = format!("{} gpu_pass={} mean_gpu_ms={:.6} samples={}\n",
                beat.beat_id, label, sum / f64::from(samples), samples);
            print!("{line}");
            report.push_str(&line);
        }
        std::fs::write(output.join("capture.txt"), &report)?;
    }
    std::fs::write(output.join("capture.txt"), report)?;
    Ok(())
}

fn ffmpeg_command() -> Command {
    let command = Command::new("ffmpeg");
    #[cfg(windows)]
    let command = {
        use std::os::windows::process::CommandExt;
        let mut command = command;
        command.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
        command
    };
    command
}

fn ffmpeg_version() -> anyhow::Result<String> {
    let output = ffmpeg_command().arg("-version").output()
        .map_err(|error| anyhow::anyhow!("video requires ffmpeg on PATH: {error}"))?;
    anyhow::ensure!(output.status.success(), "ffmpeg version preflight failed");
    Ok(String::from_utf8_lossy(&output.stdout).lines().next().unwrap_or("ffmpeg").to_owned())
}

fn parse_capture_hz(value: Option<&str>) -> anyhow::Result<f64> {
    let hz: f64 = value.map(str::parse).transpose()?.unwrap_or(30.0);
    anyhow::ensure!(hz.is_finite() && hz > 0.0,
        "capture_hz must be positive and finite");
    let step = hz.recip() as f32;
    anyhow::ensure!(step.is_finite() && step > 0.0,
        "capture_hz must have a representable positive simulation step");
    Ok(hz)
}

/// At most one RGBA frame is being written per eye. Pipe backpressure bounds
/// memory; there is no unbounded channel or accumulated video frame queue.
struct VideoEncoder {
    child: Child,
    path: PathBuf,
    log: PathBuf,
    frame_bytes: usize,
    frames: u32,
    finished: bool,
}

impl VideoEncoder {
    fn start(folder: &Path, eye: usize, width: u32, height: u32, capture_hz: f64) -> anyhow::Result<Self> {
        let path = folder.join(format!("eye{eye}.mp4"));
        let log = folder.join(format!("eye{eye}_ffmpeg.log"));
        let frame_bytes = usize::try_from(u64::from(width) * u64::from(height))?
            .checked_mul(4).ok_or_else(|| anyhow::anyhow!("RGBA frame byte count overflows"))?;
        let child = ffmpeg_command()
            .args(["-hide_banner", "-loglevel", "error", "-nostats", "-n",
                "-f", "rawvideo", "-pixel_format", "rgba",
                "-video_size", &format!("{width}x{height}"), "-framerate", &capture_hz.to_string(), "-i", "pipe:0",
                "-an", "-c:v", "libx264", "-preset", "fast", "-crf", "18",
                "-pix_fmt", "yuv420p", "-threads:v", "4", "-filter_threads", "1",
                // Tag the actual filtered AVFrames as well as encoder options.
                // Otherwise scale's unknown primaries/transfer can replace the
                // requested codec metadata. This does not alter pixel transfer.
                "-vf", "scale=out_color_matrix=bt709:out_range=tv,setparams=range=limited:color_primaries=bt709:color_trc=iec61966-2-1:colorspace=bt709",
                "-colorspace", "bt709", "-color_primaries", "bt709",
                "-color_trc", "iec61966-2-1", "-color_range", "tv",
                "-movflags", "+faststart"])
            .arg(&path)
            .stdin(Stdio::piped()).stdout(Stdio::null())
            .stderr(std::fs::File::create(&log)?).spawn()
            .map_err(|error| anyhow::anyhow!("cannot start encoder for {}: {error}", path.display()))?;
        Ok(Self { child, path, log, frame_bytes, frames: 0, finished: false })
    }

    fn write_frame(&mut self, rgba: &[u8]) -> anyhow::Result<()> {
        anyhow::ensure!(rgba.len() == self.frame_bytes, "RGBA video frame length mismatch");
        self.child.stdin.as_mut().ok_or_else(|| anyhow::anyhow!("encoder stdin is closed"))?
            .write_all(rgba).map_err(|error| anyhow::anyhow!(
                "video write failed for {}: {error}; encoder log: {}", self.path.display(), self.log.display()))?;
        self.frames += 1;
        Ok(())
    }

    fn finish(mut self) -> anyhow::Result<u32> {
        drop(self.child.stdin.take()); // EOF flushes delayed frames and finalizes MP4.
        let status = self.child.wait()?;
        self.finished = true;
        anyhow::ensure!(status.success(), "encoder failed for {} ({status}); inspect {}",
            self.path.display(), self.log.display());
        Ok(self.frames)
    }
}

impl Drop for VideoEncoder {
    fn drop(&mut self) {
        if !self.finished {
            drop(self.child.stdin.take());
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ReviewMotion {
    Static,
    Head,
    Orbit,
    HeadOrbit,
    HeadLookUp,
    SolarReview,
}

impl ReviewMotion {
    fn parse(value: &str) -> anyhow::Result<Self> {
        match value {
            "static" => Ok(Self::Static),
            "head" => Ok(Self::Head),
            "orbit" => Ok(Self::Orbit),
            "head-orbit" => Ok(Self::HeadOrbit),
            "head-look-up" => Ok(Self::HeadLookUp),
            "solar-review" => Ok(Self::SolarReview),
            _ => anyhow::bail!("motion must be static, head, orbit, head-orbit, head-look-up, or solar-review"),
        }
    }

    fn for_beat(self, beat_id: &str) -> Self {
        if self == Self::SolarReview {
            if beat_id == "solar_limb_eruption" { Self::HeadLookUp } else { Self::HeadOrbit }
        } else { self }
    }
}

fn review_pose(
    base: ScalePose64,
    center: DVec3,
    target: DVec3,
    sector_scale: f64,
    motion: ReviewMotion,
    elapsed: f64,
) -> ScalePose64 {
    let wave = |period: f64| (std::f64::consts::TAU * elapsed / period).sin();
    let mut pose = base;
    if matches!(motion, ReviewMotion::Orbit | ReviewMotion::HeadOrbit) {
        let rotation = DQuat::from_rotation_y(8.0_f64.to_radians() * wave(16.0));
        pose.local_pos = center + rotation * (base.local_pos - center);
        let forward = (target - pose.local_pos).normalize();
        let right = forward
            .cross(rotation * (base.orient * DVec3::Y))
            .normalize();
        let up = right.cross(forward).normalize();
        pose.orient = DQuat::from_mat3(&DMat3::from_cols(right, up, -forward)).normalize();
    }
    if matches!(motion, ReviewMotion::Head | ReviewMotion::HeadOrbit | ReviewMotion::HeadLookUp) {
        let head_translation = DVec3::new(0.10 * wave(6.0), 0.04 * wave(9.0), 0.06 * wave(8.0));
        pose.local_pos += pose.orient * head_translation * 10.0_f64.powf(-sector_scale);
        let pitch = if motion == ReviewMotion::HeadLookUp {
            15.0 * (1.0 - (std::f64::consts::TAU * elapsed / 60.0).cos())
        } else {
            4.0 * wave(11.0)
        };
        pose.orient = (pose.orient
            * DQuat::from_rotation_y(10.0_f64.to_radians() * wave(8.0))
            * DQuat::from_rotation_x(pitch.to_radians()))
        .normalize();
    }
    pose
}

fn stereo_camera(pose: ScalePose64, width: u32, height: u32) -> RenderCamera {
    let orient = pose.orient.as_quat();
    let mut camera = RenderCamera::mono(
        orient,
        72.0_f32.to_radians(),
        width as f32 / height as f32,
        0.002,
    );
    let projection =
        reversed_z_infinite_perspective_rh(camera.fov_y_rad, camera.aspect, camera.near_plane);
    for (eye, x) in [-0.032, 0.032].into_iter().enumerate() {
        let offset = orient * Vec3::new(x, 0.0, 0.0);
        let vp = projection * camera_view_matrix(orient) * Mat4::from_translation(-offset);
        camera.eyes[eye] = EyeView::from_view_proj(vp, offset);
    }
    camera.is_stereo = true;
    camera
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn capture_rate_is_validated_without_changing_motion_speed() {
        assert_eq!(parse_capture_hz(None).unwrap(), 30.0);
        assert_eq!(parse_capture_hz(Some("59.94")).unwrap(), 59.94);
        for invalid in ["0", "-1", "NaN", "inf", "-inf", "invalid", "1e-100", "1e100"] {
            assert!(parse_capture_hz(Some(invalid)).is_err(), "accepted invalid rate {invalid}");
        }
        let base = ScalePose64::new(cosmos_math::SectorId::SOLAR_SYSTEM,
            DVec3::new(0.0, 0.0, 0.005), 8.8, DQuat::IDENTITY);
        let at_30 = review_pose(base, DVec3::ZERO, DVec3::ZERO, 11.2,
            ReviewMotion::HeadOrbit, 120.0 / parse_capture_hz(None).unwrap());
        let at_120 = review_pose(base, DVec3::ZERO, DVec3::ZERO, 11.2,
            ReviewMotion::HeadOrbit, 480.0 / parse_capture_hz(Some("120")).unwrap());
        assert!((at_30.local_pos - at_120.local_pos).length() < 1e-15);
        assert!(at_30.orient.dot(at_120.orient).abs() > 1.0 - 1e-12,
            "capture rate must change sampling density, not simulated head/orbit speed");
    }

    #[test]
    fn review_motion_keeps_real_head_meters_and_rebuilds_stereo() {
        assert_eq!(ReviewMotion::parse("solar-review").unwrap().for_beat("solar_limb_eruption"), ReviewMotion::HeadLookUp);
        assert_eq!(ReviewMotion::SolarReview.for_beat("solar_active_region"), ReviewMotion::HeadOrbit);
        assert_eq!(ReviewMotion::Static.for_beat("solar_limb_eruption"), ReviewMotion::Static);
        let base = ScalePose64::new(
            cosmos_math::SectorId::SOLAR_SYSTEM,
            DVec3::new(0.0, 0.0, 0.005),
            8.8,
            DQuat::IDENTITY,
        );
        assert_eq!(
            review_pose(
                base,
                DVec3::ZERO,
                DVec3::ZERO,
                11.2,
                ReviewMotion::Static,
                2.0
            ),
            base
        );
        let moved = review_pose(
            base,
            DVec3::ZERO,
            DVec3::ZERO,
            11.2,
            ReviewMotion::Head,
            1.5,
        );
        let meters = (moved.local_pos - base.local_pos) * 10.0_f64.powf(11.2);
        assert!(
            (meters.x - 0.10).abs() < 1e-6,
            "tracked lean must stay in meters, independent of camera scale"
        );
        let mut zoomed = base;
        zoomed.scale_exp = 6.0;
        let zoomed_moved = review_pose(
            zoomed,
            DVec3::ZERO,
            DVec3::ZERO,
            11.2,
            ReviewMotion::Head,
            1.5,
        );
        assert_eq!(moved.local_pos, zoomed_moved.local_pos);
        let first = stereo_camera(base, 2560, 1440);
        let next = stereo_camera(moved, 2560, 1440);
        let eye_axis = next.eyes[1].eye_offset - next.eyes[0].eye_offset;
        assert!((eye_axis.length() - 0.064).abs() < 1e-6);
        assert!(
            eye_axis
                .normalize()
                .dot((moved.orient * DVec3::X).as_vec3())
                > 0.99999
        );
        for eye in 0..2 {
            assert_ne!(first.eyes[eye].view_proj, next.eyes[eye].view_proj);
            assert!(
                (next.eyes[eye].view_proj * next.eyes[eye].inv_view_proj)
                    .abs_diff_eq(Mat4::IDENTITY, 1e-4)
            );
        }
        let upward = review_pose(base, DVec3::ZERO, DVec3::ZERO, 11.2, ReviewMotion::HeadLookUp, 30.0);
        let gaze = upward.orient * DVec3::NEG_Z;
        assert!((gaze.y.asin().to_degrees() - 30.0).abs() < 1e-10,
            "the overhead review must actually look upward at the event apex");
        let returned = review_pose(base, DVec3::ZERO, DVec3::ZERO, 11.2, ReviewMotion::HeadLookUp, 60.0);
        assert!(returned.orient.dot(base.orient).abs() > 1.0 - 1e-12);
    }

    #[test]
    fn review_orbit_preserves_altitude_and_fixed_feature_target() {
        let center = DVec3::new(0.3, 0.2, -0.1);
        let target = center + DVec3::new(0.0, 0.0, 0.001);
        let base = ScalePose64::new(
            cosmos_math::SectorId::SOLAR_SYSTEM,
            center + DVec3::new(0.0, 0.0, 0.005),
            8.8,
            DQuat::IDENTITY,
        );
        for index in 0..=480 {
            let moved = review_pose(
                base,
                center,
                target,
                11.2,
                ReviewMotion::Orbit,
                f64::from(index) / 30.0,
            );
            assert!(((moved.local_pos - center).length() - 0.005).abs() < 1e-12);
            assert!(
                (moved.orient * DVec3::NEG_Z).dot((target - moved.local_pos).normalize())
                    > 1.0 - 1e-12
            );
        }
    }

    #[test]
    #[ignore = "requires ffmpeg/libx264 and ffprobe on PATH; CPU-only video roundtrip"]
    fn video_stream_preserves_both_eye_frame_order_and_color() {
        ffmpeg_version().unwrap();
        let unique = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
        let folder = std::env::temp_dir().join(format!("cosmos_video_{}_{unique}", std::process::id()));
        std::fs::create_dir(&folder).unwrap();
        let capture_hz = parse_capture_hz(None).unwrap();
        let mut encoders = [VideoEncoder::start(&folder, 0, 16, 16, capture_hz).unwrap(),
            VideoEncoder::start(&folder, 1, 16, 16, capture_hz).unwrap()];
        assert!(encoders[0].write_frame(&[0; 4]).is_err());
        let colors = [[255, 0, 0, 255], [0, 255, 0, 255], [0, 0, 255, 255]];
        for frame in 0..3 {
            for (eye, encoder) in encoders.iter_mut().enumerate() {
                let color = colors[if eye == 0 { frame } else { 2 - frame }];
                let pixels: Vec<u8> = (0..16 * 16).flat_map(|_| color).collect();
                encoder.write_frame(&pixels).unwrap();
            }
        }
        for encoder in encoders { assert_eq!(encoder.finish().unwrap(), 3); }
        for eye in 0..2 {
            let path = folder.join(format!("eye{eye}.mp4"));
            let mut probe = Command::new("ffprobe");
            #[cfg(windows)] {
                use std::os::windows::process::CommandExt;
                probe.creation_flags(0x0800_0000); // CREATE_NO_WINDOW
            }
            let metadata = probe.args(["-v", "error", "-select_streams", "v:0", "-show_entries",
                "stream=color_primaries,color_transfer,color_space,color_range,avg_frame_rate,nb_frames",
                "-of", "default=noprint_wrappers=1"]).arg(&path).output().unwrap();
            assert!(metadata.status.success(), "ffprobe must inspect the encoded MP4");
            let metadata = String::from_utf8(metadata.stdout).unwrap();
            for required in ["color_primaries=bt709", "color_transfer=iec61966-2-1",
                "color_space=bt709", "color_range=tv", "avg_frame_rate=30/1", "nb_frames=3"] {
                assert!(metadata.lines().any(|line| line == required),
                    "eye{eye} encoded stream lacks {required}: {metadata}");
            }
            let decoded = ffmpeg_command().args(["-hide_banner", "-loglevel", "error", "-i"])
                .arg(&path).args(["-f", "rawvideo", "-pix_fmt", "rgba", "pipe:1"]).output().unwrap();
            assert!(decoded.status.success());
            assert_eq!(decoded.stdout.len(), 16 * 16 * 4 * 3, "all input frames must reach the MP4");
            for frame in 0..3 {
                let expected = colors[if eye == 0 { frame } else { 2 - frame }];
                for (actual, expected) in decoded.stdout[frame * 16 * 16 * 4..][..4].iter().zip(expected) {
                    assert!((i16::from(*actual) - i16::from(expected)).abs() < 12,
                        "RGB order or video color conversion changed eye{eye} frame{frame}");
                }
            }
            std::fs::remove_file(path).unwrap();
            std::fs::remove_file(folder.join(format!("eye{eye}_ffmpeg.log"))).unwrap();
        }
        std::fs::remove_dir(folder).unwrap();
    }
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
