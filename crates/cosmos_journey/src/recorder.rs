use std::fs;
use std::path::{Path, PathBuf};

use cosmos_math::{ScalePose64, SectorTree};

use crate::schema::{
    BeatMotionProfile, BeatTargetSelector, BeatTransition, CURRENT_SCHEMA_VERSION, JourneyBeat,
    JourneyChapter, JourneyError, LocalizedTitle, SerializedPose64, StoryJourneyManifest,
};

/// Single captured camera+scale+focus sample along a live flight trace.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordedSample {
    pub pose: ScalePose64,
    pub target: BeatTargetSelector,
    pub timestamp_sec: f32,
}

/// Explicit or decimated keyframe ready for export into a `.hcj.json` Story-Journey.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordedWaypoint {
    pub pose: ScalePose64,
    pub target: BeatTargetSelector,
    pub timestamp_sec: f32,
    pub label: String,
}

/// Output filesystem paths produced when exporting a recorded journey.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExportedJourneyPaths {
    pub json_path: PathBuf,
    pub share_uri_path: PathBuf,
    pub share_uri: String,
    pub beat_count: usize,
}

/// In-game 3D waypoint & continuous logarithmic-scale path recorder.
#[derive(Clone, Debug, Default)]
pub struct JourneyRecorder {
    waypoints: Vec<RecordedWaypoint>,
    live_trace_active: bool,
    trace_samples: Vec<RecordedSample>,
    recording_elapsed_sec: f32,
    last_trace_pose: Option<ScalePose64>,
    last_trace_time: f32,
    min_interval_sec: f32,
    min_pos_delta_ru: f64,
    min_scale_delta_du: f64,
}

impl JourneyRecorder {
    pub fn new() -> Self {
        Self {
            waypoints: Vec::new(),
            live_trace_active: false,
            trace_samples: Vec::new(),
            recording_elapsed_sec: 0.0,
            last_trace_pose: None,
            last_trace_time: -1.0,
            min_interval_sec: 0.12,
            min_pos_delta_ru: 0.06,
            min_scale_delta_du: 0.06,
        }
    }

    #[inline]
    pub fn live_trace_active(&self) -> bool {
        self.live_trace_active
    }

    #[inline]
    pub fn waypoint_count(&self) -> usize {
        self.waypoints.len()
    }

    #[inline]
    pub fn waypoints(&self) -> &[RecordedWaypoint] {
        &self.waypoints
    }

    #[inline]
    pub fn live_sample_count(&self) -> usize {
        self.trace_samples.len()
    }

    pub fn clear(&mut self) {
        self.waypoints.clear();
        self.trace_samples.clear();
        self.live_trace_active = false;
        self.recording_elapsed_sec = 0.0;
        self.last_trace_pose = None;
        self.last_trace_time = -1.0;
    }

    /// Snapshots an explicit camera waypoint (`R` hotkey or `+WPT` wrist bead).
    pub fn record_manual_waypoint(
        &mut self,
        pose: &ScalePose64,
        target: BeatTargetSelector,
        label: Option<String>,
    ) -> usize {
        let idx = self.waypoints.len() + 1;
        self.waypoints.push(RecordedWaypoint {
            pose: *pose,
            target,
            timestamp_sec: self.recording_elapsed_sec,
            label: label.unwrap_or_else(|| format!("wpt_{idx:03}")),
        });
        self.waypoints.len()
    }

    /// Toggles continuous live flight recording; finishing runs logarithmic Ramer–Douglas–Peucker
    /// simplification across 4D `[x, y, z, scale_weight * scale_exp]` and commits keyframes.
    pub fn toggle_live_trace(&mut self, tree: &SectorTree) -> bool {
        if !self.live_trace_active {
            self.live_trace_active = true;
            self.trace_samples.clear();
            self.last_trace_pose = None;
            self.last_trace_time = -1.0;
        } else {
            self.live_trace_active = false;
            self.commit_live_trace(tree);
        }
        self.live_trace_active
    }

    pub fn step_live_trace(
        &mut self,
        dt_sec: f32,
        pose: &ScalePose64,
        target: BeatTargetSelector,
    ) {
        let dt = dt_sec.max(0.0);
        self.recording_elapsed_sec += dt;
        if !self.live_trace_active {
            return;
        }
        let elapsed_since_last = self.recording_elapsed_sec - self.last_trace_time;
        let should_sample = match &self.last_trace_pose {
            None => true,
            Some(prev) => {
                let scale_du = (pose.scale_exp - prev.scale_exp).abs();
                let pos_ru = if pose.sector_id == prev.sector_id {
                    pose.local_pos.distance(prev.local_pos)
                } else {
                    1.0
                };
                elapsed_since_last >= self.min_interval_sec
                    && (pos_ru >= self.min_pos_delta_ru
                        || scale_du >= self.min_scale_delta_du
                        || self
                            .trace_samples
                            .last()
                            .is_some_and(|s| s.target != target))
            }
        };
        if should_sample {
            self.trace_samples.push(RecordedSample {
                pose: *pose,
                target,
                timestamp_sec: self.recording_elapsed_sec,
            });
            self.last_trace_pose = Some(*pose);
            self.last_trace_time = self.recording_elapsed_sec;
        }
    }

    /// Decimates raw trace samples into clean keyframe waypoints preserving scale turns & gaze targets.
    pub fn commit_live_trace(&mut self, tree: &SectorTree) -> usize {
        if self.trace_samples.is_empty() {
            return 0;
        }
        let simplified = rdp_simplify_samples(tree, &self.trace_samples, 0.22);
        let base_idx = self.waypoints.len();
        let added = simplified.len();
        for (i, sample) in simplified.into_iter().enumerate() {
            self.waypoints.push(RecordedWaypoint {
                pose: sample.pose,
                target: sample.target,
                timestamp_sec: sample.timestamp_sec,
                label: format!("trace_{:03}", base_idx + i + 1),
            });
        }
        self.trace_samples.clear();
        added
    }

    pub fn build_manifest(
        &self,
        id: &str,
        title: LocalizedTitle,
        author: &str,
    ) -> Result<StoryJourneyManifest, JourneyError> {
        if self.waypoints.is_empty() {
            return Err(JourneyError::EmptyJourney(id.to_owned()));
        }
        let mut beats = Vec::with_capacity(self.waypoints.len());
        let mut prev_ts = self.waypoints[0].timestamp_sec;
        for (i, wp) in self.waypoints.iter().enumerate() {
            let delta_t = (wp.timestamp_sec - prev_ts).clamp(1.2, 6.0);
            prev_ts = wp.timestamp_sec;
            beats.push(JourneyBeat {
                beat_id: format!("rec_{:03}_{}", i + 1, wp.label),
                pose: SerializedPose64::from_pose64(&wp.pose),
                look_at_pose: None,
                target: wp.target.clone(),
                motion: BeatMotionProfile::default(),
                transition_in: BeatTransition {
                    duration_sec: if i == 0 { 1.6 } else { delta_t },
                    ..Default::default()
                },
                caption_hint: None,
            });
        }

        let manifest = StoryJourneyManifest {
            schema_version: CURRENT_SCHEMA_VERSION,
            id: id.to_owned(),
            title,
            subtitle: LocalizedTitle::new(
                "Viaje grabado en escena",
                "In-game recorded journey",
                "Pelissä tallennettu matka",
            ),
            author: author.to_owned(),
            loop_journey: false,
            chapters: vec![JourneyChapter {
                id: "recorded_chapter_01".to_owned(),
                act_index: 1,
                title: LocalizedTitle::new(
                    "Ruta Personalizada",
                    "Custom Route",
                    "Oma Reitti",
                ),
                beats,
            }],
        };
        manifest.validate(None)?;
        Ok(manifest)
    }

    pub fn export_to_directory(
        &mut self,
        tree: &SectorTree,
        dir: impl AsRef<Path>,
        id: &str,
        title: LocalizedTitle,
        author: &str,
    ) -> Result<ExportedJourneyPaths, JourneyError> {
        if self.live_trace_active {
            self.live_trace_active = false;
            self.commit_live_trace(tree);
        }
        let manifest = self.build_manifest(id, title, author)?;
        let dir = dir.as_ref();
        fs::create_dir_all(dir)?;

        let json_path = dir.join(format!("{id}.hcj.json"));
        let share_uri_path = dir.join(format!("{id}.share.txt"));
        let json_str = manifest.to_json_pretty()?;
        let share_uri = manifest.encode_share_uri()?;
        fs::write(&json_path, json_str)?;
        fs::write(&share_uri_path, &share_uri)?;

        Ok(ExportedJourneyPaths {
            json_path,
            share_uri_path,
            share_uri,
            beat_count: manifest.total_beats(),
        })
    }
}

/// 4D Ramer–Douglas–Peucker decimation across `(x, y, z, 2.5 * scale_exp)` re-anchored into common ancestor.
pub fn rdp_simplify_samples(
    tree: &SectorTree,
    samples: &[RecordedSample],
    epsilon: f64,
) -> Vec<RecordedSample> {
    if samples.len() <= 2 {
        return samples.to_vec();
    }

    let first = &samples[0];
    let last = &samples[samples.len() - 1];
    let anchor = first.pose.sector_id;

    let to_vec4 = |s: &RecordedSample| -> glam::DVec4 {
        let mut p = s.pose;
        let _ = p.reanchor(tree, anchor);
        glam::DVec4::new(
            p.local_pos.x,
            p.local_pos.y,
            p.local_pos.z,
            p.scale_exp * 2.5,
        )
    };

    let a = to_vec4(first);
    let b = to_vec4(last);
    let ab = b - a;
    let ab_len2 = ab.length_squared();

    let mut max_dist = 0.0_f64;
    let mut split_idx = 0usize;
    for (idx, s) in samples.iter().enumerate().take(samples.len() - 1).skip(1) {
        let p = to_vec4(s);
        let d = if ab_len2 < 1e-12 {
            (p - a).length()
        } else {
            let t = ((p - a).dot(ab) / ab_len2).clamp(0.0, 1.0);
            (p - (a + ab * t)).length()
        };
        // Always boost priority of semantic target transitions so narration triggers are never pruned.
        let bonus = if s.target != first.target && s.target != BeatTargetSelector::None {
            epsilon * 1.5
        } else {
            0.0
        };
        if d + bonus > max_dist {
            max_dist = d + bonus;
            split_idx = idx;
        }
    }

    if max_dist > epsilon && split_idx > 0 {
        let mut left = rdp_simplify_samples(tree, &samples[..=split_idx], epsilon);
        let right = rdp_simplify_samples(tree, &samples[split_idx..], epsilon);
        left.pop();
        left.extend(right);
        left
    } else {
        vec![first.clone(), last.clone()]
    }
}
