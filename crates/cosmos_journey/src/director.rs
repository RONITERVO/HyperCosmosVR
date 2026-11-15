use cosmos_manifest::UniverseCatalog;
use cosmos_math::{ScalePose64, ScaleSpline, ScaleSplineKeyframe, SectorId, SectorTree};
use glam::{DQuat, DVec3, Quat, Vec3};

use crate::elastic::{
    ElasticOverrideState, JourneyPlayState, ManualOverrideInput, PeripheralChoice,
    PeripheralPromptVisual,
};
use crate::schema::{
    BeatMotionProfile, BeatTargetSelector, JourneyBeat, JourneyChapter, LocalizedTitle,
    MotionProfileKind, SerializedPose64, StoryJourneyManifest, TransitionKind,
};

#[derive(Clone, Debug)]
struct FlatBeatRef {
    chapter_idx: usize,
    beat: JourneyBeat,
    pose: ScalePose64,
    look_at_pose: Option<ScalePose64>,
}

/// Saved checkpoint allowing the user to exit into free-flight and resume cleanly later.
#[derive(Clone, Debug)]
pub struct JourneyBookmark {
    pub journey_id: String,
    pub flat_beat_idx: usize,
    pub ghost_pose: ScalePose64,
}

#[derive(Clone, Debug)]
enum BeatPhase {
    TransitionIn {
        spline: ScaleSpline,
        duration_sec: f32,
        elapsed_sec: f32,
        narration_lead_sec: f32,
        narration_triggered: bool,
    },
    Dwelling {
        elapsed_sec: f32,
        min_dwell_sec: f32,
        max_dwell_sec: f32,
        base_pose: ScalePose64,
        base_look_at: Option<ScalePose64>,
    },
}

#[derive(Clone, Debug)]
struct ActiveSubHop {
    spline: ScaleSpline,
    duration_sec: f32,
    elapsed_sec: f32,
}

/// Per-frame output produced by `JourneyDirector::step` for `UniverseRuntime`.
#[derive(Clone, Debug, Default)]
pub struct JourneyStepOutput {
    pub desired_camera_pose: Option<ScalePose64>,
    pub ghost_pose: Option<ScalePose64>,
    pub trigger_target: Option<BeatTargetSelector>,
    pub active_caption_hint: Option<String>,
    pub active_chapter_title: Option<LocalizedTitle>,
    pub peripheral_prompt: Option<PeripheralPromptVisual>,
    pub exited_to_free_flight_pose: Option<ScalePose64>,
    pub beat_changed: bool,
    pub progress_fraction: f32,
}

/// Stateful orchestrator driving autonomous cosmic flights, sentence-paced micro-choreography,
/// and elastic thumbstick override / peripheral exit recovery.
pub struct JourneyDirector {
    manifest: StoryJourneyManifest,
    flat_beats: Vec<FlatBeatRef>,
    active_idx: usize,
    play_state: JourneyPlayState,
    ghost_pose: ScalePose64,
    phase: BeatPhase,
    sub_hop: Option<ActiveSubHop>,
    sentences_at_vantage: u8,
    vantage_sign: f64,
    narration_finished_for_beat: bool,
    override_state: ElasticOverrideState,
    bookmark: Option<JourneyBookmark>,
}

impl JourneyDirector {
    pub fn from_manifest(
        manifest: StoryJourneyManifest,
        initial_pose: ScalePose64,
    ) -> Result<Self, crate::schema::JourneyError> {
        let flat_refs = manifest.flatten_beats();
        if flat_refs.is_empty() {
            return Err(crate::schema::JourneyError::EmptyJourney(
                manifest.id.clone(),
            ));
        }

        let mut flat_beats = Vec::with_capacity(flat_refs.len());
        for (chapter_idx, _beat_idx, beat) in flat_refs {
            let pose = beat.pose.to_pose64(&beat.beat_id)?;
            let look_at_pose = beat
                .look_at_pose
                .as_ref()
                .map(|p| p.to_pose64(&beat.beat_id))
                .transpose()?;
            flat_beats.push(FlatBeatRef {
                chapter_idx,
                beat: beat.clone(),
                pose,
                look_at_pose,
            });
        }

        let first_pose = flat_beats[0].pose;
        Ok(Self {
            manifest,
            flat_beats,
            active_idx: 0,
            play_state: JourneyPlayState::Inactive,
            ghost_pose: initial_pose,
            phase: BeatPhase::Dwelling {
                elapsed_sec: 0.0,
                min_dwell_sec: 2.5,
                max_dwell_sec: 16.0,
                base_pose: first_pose,
                base_look_at: None,
            },
            sub_hop: None,
            sentences_at_vantage: 0,
            vantage_sign: 1.0,
            narration_finished_for_beat: false,
            override_state: ElasticOverrideState::default(),
            bookmark: None,
        })
    }

    pub fn manifest(&self) -> &StoryJourneyManifest {
        &self.manifest
    }

    #[inline]
    pub fn play_state(&self) -> JourneyPlayState {
        self.play_state
    }

    #[inline]
    pub fn is_active(&self) -> bool {
        matches!(
            self.play_state,
            JourneyPlayState::Playing | JourneyPlayState::Paused | JourneyPlayState::PausedByLeash
        )
    }

    #[inline]
    pub fn active_beat_index(&self) -> usize {
        self.active_idx
    }

    #[inline]
    pub fn total_beats(&self) -> usize {
        self.flat_beats.len()
    }

    #[inline]
    pub fn bookmark(&self) -> Option<&JourneyBookmark> {
        self.bookmark.as_ref()
    }

    #[inline]
    pub fn ghost_pose(&self) -> &ScalePose64 {
        &self.ghost_pose
    }

    #[inline]
    pub fn override_state(&self) -> &ElasticOverrideState {
        &self.override_state
    }

    pub fn active_chapter(&self) -> &JourneyChapter {
        let ch_idx = self.flat_beats[self.active_idx].chapter_idx;
        &self.manifest.chapters[ch_idx]
    }

    pub fn active_beat(&self) -> &JourneyBeat {
        &self.flat_beats[self.active_idx].beat
    }

    pub fn start(&mut self, tree: &SectorTree, from_pose: &ScalePose64, beat_idx: usize) {
        self.override_state.reset();
        self.play_state = JourneyPlayState::Playing;
        self.enter_beat(tree, from_pose, beat_idx.min(self.flat_beats.len() - 1));
    }

    pub fn resume_from_bookmark(&mut self, tree: &SectorTree, current_pose: &ScalePose64) -> bool {
        if let Some(bm) = self.bookmark.take()
            && bm.journey_id == self.manifest.id
        {
            self.override_state.reset();
            self.play_state = JourneyPlayState::Playing;
            self.enter_beat(
                tree,
                current_pose,
                bm.flat_beat_idx.min(self.flat_beats.len() - 1),
            );
            return true;
        }
        false
    }

    pub fn toggle_play_pause(&mut self, tree: &SectorTree, current_pose: &ScalePose64) {
        match self.play_state {
            JourneyPlayState::Inactive => {
                if !self.resume_from_bookmark(tree, current_pose) {
                    self.start(tree, current_pose, self.active_idx);
                }
            }
            JourneyPlayState::Completed => {
                self.start(tree, current_pose, 0);
            }
            JourneyPlayState::Paused => {
                // Preserve the active spline, dwell timer, and elastic offset exactly.
                self.play_state = JourneyPlayState::Playing;
            }
            JourneyPlayState::PausedByLeash => {
                self.play_state = JourneyPlayState::Playing;
                self.override_state.snap_to_follow();
            }
            JourneyPlayState::Playing => {
                self.play_state = JourneyPlayState::Paused;
            }
        }
    }

    pub fn exit_to_free_flight(&mut self, tree: &SectorTree) -> ScalePose64 {
        let composed = self.override_state.compose_camera_pose(tree, &self.ghost_pose);
        self.save_bookmark_at_ghost();
        self.override_state.reset();
        self.sub_hop = None;
        self.play_state = JourneyPlayState::Inactive;
        composed
    }

    pub fn follow_ghost_back(&mut self) {
        if self.play_state == JourneyPlayState::PausedByLeash {
            self.play_state = JourneyPlayState::Playing;
            self.override_state.snap_to_follow();
        }
    }

    fn seek_play_state(&self) -> JourneyPlayState {
        match self.play_state {
            JourneyPlayState::Paused | JourneyPlayState::PausedByLeash => JourneyPlayState::Paused,
            _ => JourneyPlayState::Playing,
        }
    }

    pub fn next_beat(&mut self, tree: &SectorTree, current_pose: &ScalePose64) -> bool {
        if self.active_idx + 1 < self.flat_beats.len() {
            self.play_state = self.seek_play_state();
            self.override_state.snap_to_follow();
            self.enter_beat(tree, current_pose, self.active_idx + 1);
            true
        } else if self.manifest.loop_journey {
            self.play_state = self.seek_play_state();
            self.override_state.snap_to_follow();
            self.enter_beat(tree, current_pose, 0);
            true
        } else {
            false
        }
    }

    pub fn prev_beat(&mut self, tree: &SectorTree, current_pose: &ScalePose64) -> bool {
        let target_idx = self.active_idx.saturating_sub(1);
        self.play_state = self.seek_play_state();
        self.override_state.snap_to_follow();
        self.enter_beat(tree, current_pose, target_idx);
        true
    }

    pub fn next_chapter(&mut self, tree: &SectorTree, current_pose: &ScalePose64) -> bool {
        let cur_ch = self.flat_beats[self.active_idx].chapter_idx;
        if let Some((idx, _)) = self
            .flat_beats
            .iter()
            .enumerate()
            .find(|(_, b)| b.chapter_idx > cur_ch)
        {
            self.play_state = self.seek_play_state();
            self.override_state.snap_to_follow();
            self.enter_beat(tree, current_pose, idx);
            true
        } else {
            false
        }
    }

    pub fn prev_chapter(&mut self, tree: &SectorTree, current_pose: &ScalePose64) -> bool {
        let cur_ch = self.flat_beats[self.active_idx].chapter_idx;
        let target_ch = cur_ch.saturating_sub(1);
        if let Some((idx, _)) = self
            .flat_beats
            .iter()
            .enumerate()
            .find(|(_, b)| b.chapter_idx == target_ch)
        {
            self.play_state = self.seek_play_state();
            self.override_state.snap_to_follow();
            self.enter_beat(tree, current_pose, idx);
            true
        } else {
            false
        }
    }

    /// Called when one bilingual sentence pair completes within the current narration plan.
    /// Triggers a smooth 1.25s micro-spline vantage shift once `max_sentences_per_vantage` (`1..=2`) is reached.
    pub fn notify_translation_sentence_finished(&mut self, tree: &SectorTree) {
        if self.play_state != JourneyPlayState::Playing {
            return;
        }
        let max_sent = self.active_beat().motion.max_sentences_per_vantage.max(1);
        self.sentences_at_vantage = self.sentences_at_vantage.saturating_add(1);
        if self.sentences_at_vantage < max_sent {
            return;
        }
        self.sentences_at_vantage = 0;

        let target_vantage = compute_vantage_hop_pose(
            tree,
            &self.ghost_pose,
            self.flat_beats[self.active_idx].look_at_pose.as_ref(),
            self.vantage_sign,
        );
        self.vantage_sign = -self.vantage_sign;
        if let Ok(spline) = build_transition_spline(
            tree,
            &self.ghost_pose,
            &target_vantage,
            TransitionKind::CatmullRomScaleSpline,
        ) {
            self.sub_hop = Some(ActiveSubHop {
                spline,
                duration_sec: 1.25,
                elapsed_sec: 0.0,
            });
        }
    }

    /// Called by `UniverseRuntime` when all audio segments of the active beat have completed.
    pub fn notify_beat_narration_finished(&mut self) {
        if self.is_active() {
            self.narration_finished_for_beat = true;
        }
    }

    /// Advances transition splines, continuous anti-boredom motion, elastic offsets, and exit prompts.
    #[allow(clippy::too_many_arguments)]
    pub fn step(
        &mut self,
        tree: &SectorTree,
        dt_sec: f32,
        override_input: &ManualOverrideInput,
        cam_basis_origin: Vec3,
        cam_basis_rot: Quat,
        pointer_origin: Option<Vec3>,
        pointer_dir: Option<Vec3>,
        pointer_triggered: bool,
    ) -> JourneyStepOutput {
        let mut out = JourneyStepOutput {
            progress_fraction: if self.flat_beats.len() <= 1 {
                1.0
            } else {
                self.active_idx as f32 / (self.flat_beats.len() - 1) as f32
            },
            ..Default::default()
        };

        if !self.is_active() {
            return out;
        }

        if self.play_state == JourneyPlayState::Paused {
            out.desired_camera_pose = Some(
                self.override_state
                    .compose_camera_pose(tree, &self.ghost_pose),
            );
            out.ghost_pose = Some(self.ghost_pose);
            out.active_chapter_title = Some(self.active_chapter().title.clone());
            out.active_caption_hint = self.active_beat().caption_hint.clone();
            return out;
        }

        let dt = dt_sec.max(0.0);
        self.override_state
            .step(override_input, dt, &mut self.play_state);

        // Compute ghost pose relative to camera origin for 3D tether rendering.
        let composed_now = self
            .override_state
            .compose_camera_pose(tree, &self.ghost_pose);
        let ghost_rel_vec = self
            .ghost_pose
            .project_entity(tree, &composed_now, 0.1)
            .map(|cr| cr.pos)
            .unwrap_or(Vec3::ZERO);

        let (prompt_vis, chosen) = self.override_state.evaluate_peripheral_prompt(
            self.play_state,
            cam_basis_origin,
            cam_basis_rot,
            ghost_rel_vec,
            pointer_origin,
            pointer_dir,
            pointer_triggered,
        );
        out.peripheral_prompt = prompt_vis;

        if let Some(choice) = chosen {
            match choice {
                PeripheralChoice::Follow => self.follow_ghost_back(),
                PeripheralChoice::Exit => {
                    let detached = self.exit_to_free_flight(tree);
                    out.exited_to_free_flight_pose = Some(detached);
                    out.peripheral_prompt = None;
                    return out;
                }
            }
        }

        if self.play_state == JourneyPlayState::PausedByLeash {
            let composed = self.override_state.compose_camera_pose(tree, &self.ghost_pose);
            out.desired_camera_pose = Some(composed);
            out.ghost_pose = Some(self.ghost_pose);
            out.active_chapter_title = Some(self.active_chapter().title.clone());
            out.active_caption_hint = self.active_beat().caption_hint.clone();
            return out;
        }

        // Advance active spline or dwelling choreography while Playing.
        let mut advance_next_beat = false;
        let beat_meta = self.flat_beats[self.active_idx].clone();
        match &mut self.phase {
            BeatPhase::TransitionIn {
                spline,
                duration_sec,
                elapsed_sec,
                narration_lead_sec,
                narration_triggered,
            } => {
                *elapsed_sec += dt;
                let t = if *duration_sec > 1e-4 {
                    (*elapsed_sec / *duration_sec).clamp(0.0, 1.0)
                } else {
                    1.0
                };
                if let Ok(mut p) = spline.evaluate(tree, smoothstep(t as f64)) {
                    let _ = p.reanchor(tree, beat_meta.pose.sector_id);
                    self.ghost_pose = p;
                }
                if !*narration_triggered && *elapsed_sec >= (*duration_sec - *narration_lead_sec).max(0.0) {
                    *narration_triggered = true;
                    out.trigger_target = Some(beat_meta.beat.target.clone());
                }
                if t >= 1.0 {
                    let trigger_now = !*narration_triggered;
                    if trigger_now {
                        out.trigger_target = Some(beat_meta.beat.target.clone());
                    }
                    self.phase = BeatPhase::Dwelling {
                        elapsed_sec: 0.0,
                        min_dwell_sec: beat_meta.beat.motion.min_dwell_sec.max(1.0),
                        max_dwell_sec: beat_meta.beat.motion.max_dwell_sec.max(4.0),
                        base_pose: beat_meta.pose,
                        base_look_at: beat_meta.look_at_pose,
                    };
                }
            }
            BeatPhase::Dwelling {
                elapsed_sec,
                min_dwell_sec,
                max_dwell_sec,
                base_pose,
                base_look_at,
            } => {
                *elapsed_sec += dt;
                if let Some(hop) = &mut self.sub_hop {
                    hop.elapsed_sec += dt;
                    let ht = (hop.elapsed_sec / hop.duration_sec.max(0.05)).clamp(0.0, 1.0);
                    if let Ok(mut p) = hop.spline.evaluate(tree, smoothstep(ht as f64)) {
                        let _ = p.reanchor(tree, base_pose.sector_id);
                        self.ghost_pose = p;
                        if ht >= 1.0 {
                            *base_pose = self.ghost_pose;
                        }
                    }
                    if ht >= 1.0 {
                        self.sub_hop = None;
                    }
                } else {
                    self.ghost_pose = step_motion_profile(
                        tree,
                        base_pose,
                        base_look_at.as_ref(),
                        &beat_meta.beat.motion,
                        *elapsed_sec,
                    );
                }

                let target_has_audio = !matches!(beat_meta.beat.target, BeatTargetSelector::None);
                let ready_to_advance = if target_has_audio {
                    (self.narration_finished_for_beat && *elapsed_sec >= *min_dwell_sec)
                        || *elapsed_sec >= *max_dwell_sec
                } else {
                    *elapsed_sec >= *min_dwell_sec
                };
                if ready_to_advance {
                    advance_next_beat = true;
                }
            }
        }

        if advance_next_beat {
            let cur_pose = self.ghost_pose;
            if self.active_idx + 1 < self.flat_beats.len() {
                self.enter_beat(tree, &cur_pose, self.active_idx + 1);
                out.beat_changed = true;
            } else if self.manifest.loop_journey {
                self.enter_beat(tree, &cur_pose, 0);
                out.beat_changed = true;
            } else {
                self.play_state = JourneyPlayState::Completed;
            }
        }

        let composed = self.override_state.compose_camera_pose(tree, &self.ghost_pose);
        out.desired_camera_pose = if self.play_state == JourneyPlayState::Completed {
            None
        } else {
            Some(composed)
        };
        out.ghost_pose = Some(self.ghost_pose);
        out.active_chapter_title = Some(self.active_chapter().title.clone());
        out.active_caption_hint = self.active_beat().caption_hint.clone();
        out
    }

    fn save_bookmark_at_ghost(&mut self) {
        self.bookmark = Some(JourneyBookmark {
            journey_id: self.manifest.id.clone(),
            flat_beat_idx: self.active_idx,
            ghost_pose: self.ghost_pose,
        });
    }

    fn enter_beat(&mut self, tree: &SectorTree, from_pose: &ScalePose64, next_idx: usize) {
        self.active_idx = next_idx;
        self.sub_hop = None;
        self.sentences_at_vantage = 0;
        self.narration_finished_for_beat = false;

        let target_ref = &self.flat_beats[next_idx];
        let mut duration_sec = target_ref.beat.transition_in.duration_sec.max(0.05);
        if target_ref.beat.transition_in.kind == TransitionKind::InstantBookmark {
            duration_sec = 0.05;
        }
        // Automatically adapt duration slightly when diving many orders of magnitude so speed feels luminous, not jarring.
        let du = (target_ref.pose.scale_exp - from_pose.scale_exp).abs() as f32;
        if du > 3.0 && target_ref.beat.transition_in.kind != TransitionKind::InstantBookmark {
            duration_sec = duration_sec.max((1.4 + du * 0.14).clamp(1.6, 5.2));
        }

        let lead_sec = target_ref
            .beat
            .transition_in
            .narration_lead_sec
            .clamp(0.0, duration_sec);

        match build_transition_spline(
            tree,
            from_pose,
            &target_ref.pose,
            target_ref.beat.transition_in.kind,
        ) {
            Ok(spline) => {
                self.phase = BeatPhase::TransitionIn {
                    spline,
                    duration_sec,
                    elapsed_sec: 0.0,
                    narration_lead_sec: lead_sec,
                    narration_triggered: false,
                };
                self.ghost_pose = *from_pose;
            }
            Err(_) => {
                self.ghost_pose = target_ref.pose;
                self.phase = BeatPhase::Dwelling {
                    elapsed_sec: 0.0,
                    min_dwell_sec: target_ref.beat.motion.min_dwell_sec.max(1.0),
                    max_dwell_sec: target_ref.beat.motion.max_dwell_sec.max(4.0),
                    base_pose: target_ref.pose,
                    base_look_at: target_ref.look_at_pose,
                };
            }
        }
    }
}

/// Builds a smooth `ScaleSpline` linking two poses, arcing logarithmically outward on big scale dives.
pub fn build_transition_spline(
    tree: &SectorTree,
    from: &ScalePose64,
    to: &ScalePose64,
    kind: TransitionKind,
) -> Result<ScaleSpline, cosmos_math::MathError> {
    let mut from_local = *from;
    from_local.reanchor(tree, to.sector_id)?;
    let mid_pos = from_local.local_pos.lerp(to.local_pos, 0.5);
    let mid_rot = from_local.orient.slerp(to.orient, 0.5).normalize();
    let mid_scale = 0.5 * (from_local.scale_exp + to.scale_exp);
    let du = (to.scale_exp - from_local.scale_exp).abs();

    let lateral_arc = match kind {
        TransitionKind::LogScaleHyperDive => {
            let right = mid_rot * DVec3::X;
            right * (0.45 * (du * 0.25).min(1.5)) + DVec3::Y * 0.22
        }
        TransitionKind::ContinuousOrbitBridge => {
            let right = mid_rot * DVec3::X;
            right * 0.38
        }
        TransitionKind::CatmullRomScaleSpline | TransitionKind::InstantBookmark => {
            let right = mid_rot * DVec3::X;
            right * (0.18 * (from_local.local_pos.distance(to.local_pos)).min(1.8))
        }
    };

    Ok(ScaleSpline::new(vec![
        ScaleSplineKeyframe {
            sector_id: from_local.sector_id,
            local_pos: from_local.local_pos,
            scale_exp: from_local.scale_exp,
            orient: from_local.orient,
            t: 0.0,
        },
        ScaleSplineKeyframe {
            sector_id: to.sector_id,
            local_pos: mid_pos + lateral_arc,
            scale_exp: mid_scale,
            orient: mid_rot,
            t: 0.5,
        },
        ScaleSplineKeyframe {
            sector_id: to.sector_id,
            local_pos: to.local_pos,
            scale_exp: to.scale_exp,
            orient: to.orient,
            t: 1.0,
        },
    ]))
}

/// Evaluates continuous stereoscopic parallax motion around `base_look_at` (or forward pivot).
fn step_motion_profile(
    tree: &SectorTree,
    base_pose: &ScalePose64,
    base_look_at: Option<&ScalePose64>,
    profile: &BeatMotionProfile,
    elapsed_sec: f32,
) -> ScalePose64 {
    let t = elapsed_sec as f64;
    let yaw_rad = (profile.yaw_deg_per_sec as f64).to_radians() * t;
    let pitch_rad = (profile.pitch_wave_deg as f64).to_radians() * (t * 0.85).sin();

    let pivot_local = base_look_at
        .and_then(|la| {
            let mut copy = *la;
            copy.reanchor(tree, base_pose.sector_id).ok()?;
            Some(copy.local_pos)
        })
        .unwrap_or_else(|| base_pose.local_pos + (base_pose.orient * DVec3::NEG_Z) * 2.2);

    let to_cam = base_pose.local_pos - pivot_local;
    let orbit_radius = to_cam.length().max(0.65);
    let forward_to_pivot = (-to_cam).normalize_or_zero();

    let (eff_yaw, eff_pitch, scale_offset, radius_scale) = match profile.kind {
        MotionProfileKind::OrbitalParallax => (
            yaw_rad * (profile.orbit_radius_factor as f64).clamp(0.15, 1.0),
            pitch_rad,
            profile.scale_breathing_du * (t * 0.72).sin(),
            1.0,
        ),
        MotionProfileKind::HelicalAscent => (
            yaw_rad,
            pitch_rad + 0.06 * (t * 0.5).cos(),
            profile.scale_breathing_du + 0.028 * t.min(12.0),
            1.0 + 0.06 * (t * 0.6).sin(),
        ),
        MotionProfileKind::ScaleDiveCruise => (
            yaw_rad * 0.6,
            pitch_rad * 0.7,
            -0.032 * t.min(10.0),
            (1.0 - 0.022 * t).max(0.68),
        ),
        MotionProfileKind::FigureEightShowcase => (
            (18.0_f64.to_radians()) * (t * 0.65).sin(),
            (7.5_f64.to_radians()) * (t * 1.30).sin(),
            profile.scale_breathing_du * (t * 0.9).cos(),
            1.0,
        ),
        MotionProfileKind::StaticHold => (
            0.35_f64.to_radians() * (t * 0.5).sin(),
            0.25_f64.to_radians() * (t * 0.4).cos(),
            0.015 * (t * 0.5).sin(),
            1.0,
        ),
    };

    if forward_to_pivot.length_squared() < 1e-6 {
        let mut p = *base_pose;
        p.scale_exp = (base_pose.scale_exp + scale_offset).clamp(-36.5, 29.0);
        return p;
    }

    let orb_rot = DQuat::from_rotation_y(eff_yaw) * DQuat::from_rotation_x(eff_pitch);
    let new_pos = pivot_local + (orb_rot * to_cam) * radius_scale;
    let new_orient = orient_toward(new_pos, pivot_local).max_align(&base_pose.orient, orb_rot);
    let _ = orbit_radius;
    ScalePose64::new(
        base_pose.sector_id,
        new_pos,
        (base_pose.scale_exp + scale_offset).clamp(-36.5, 29.0),
        new_orient,
    )
}

fn compute_vantage_hop_pose(
    tree: &SectorTree,
    current_pose: &ScalePose64,
    look_at: Option<&ScalePose64>,
    sign: f64,
) -> ScalePose64 {
    let pivot = look_at
        .and_then(|la| {
            let mut c = *la;
            c.reanchor(tree, current_pose.sector_id).ok()?;
            Some(c.local_pos)
        })
        .unwrap_or_else(|| current_pose.local_pos + (current_pose.orient * DVec3::NEG_Z) * 2.1);

    let offset = current_pose.local_pos - pivot;
    let hop_rot = DQuat::from_rotation_y(sign * 30.0_f64.to_radians())
        * DQuat::from_rotation_x(sign * 6.5_f64.to_radians());
    let next_pos = pivot + (hop_rot * offset) * 0.92;
    let next_orient = orient_toward(next_pos, pivot);
    ScalePose64::new(
        current_pose.sector_id,
        next_pos,
        (current_pose.scale_exp - 0.11 * sign).clamp(-36.5, 29.0),
        next_orient,
    )
}

/// Computes camera orientation facing `target_pos` (`-Z` forward toward `target_pos`, `+Y` world up).
pub fn orient_toward(from_pos: DVec3, target_pos: DVec3) -> DQuat {
    let dir = (target_pos - from_pos).normalize_or_zero();
    if dir.length_squared() < 1e-8 {
        return DQuat::IDENTITY;
    }
    let up = if dir.dot(DVec3::Y).abs() > 0.97 {
        DVec3::Z
    } else {
        DVec3::Y
    };
    let right = dir.cross(up).normalize();
    let ortho_up = right.cross(dir).normalize();
    let mat = glam::DMat3::from_cols(right, ortho_up, -dir);
    DQuat::from_mat3(&mat).normalize()
}

trait QuatFallback {
    fn max_align(self, fallback: &DQuat, delta: DQuat) -> DQuat;
}

impl QuatFallback for DQuat {
    fn max_align(self, fallback: &DQuat, delta: DQuat) -> DQuat {
        if self.is_finite() && self.length_squared() > 0.5 {
            self
        } else {
            (delta * *fallback).normalize()
        }
    }
}

#[inline]
fn smoothstep(t: f64) -> f64 {
    let x = t.clamp(0.0, 1.0);
    x * x * (3.0 - 2.0 * x)
}

/// Builds a concise localized chapter title helper from a catalog experience when synthesizing journeys.
pub fn localized_experience_title(catalog: &UniverseCatalog, exp_id: u8) -> LocalizedTitle {
    if let Some(exp) = catalog.experience(exp_id) {
        let base = catalog.str_slice(exp.title);
        return LocalizedTitle::unified(format!("{exp_id:02} · {base}"));
    }
    LocalizedTitle::unified(format!("Experience {exp_id:02}"))
}

#[allow(dead_code)]
fn _pose_helper_used() -> (SectorId, SerializedPose64) {
    (
        SectorId::EARTH_ORBIT,
        SerializedPose64 {
            sector: "EARTH_ORBIT".to_owned(),
            local_pos: [0.0, 0.0, 2.0],
            scale_exp: 6.6,
            orient_xyzw: [0.0, 0.0, 0.0, 1.0],
        },
    )
}
