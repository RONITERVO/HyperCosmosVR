//! Spatial first-use guidance and tour transport. Rendering and pointer hit tests use
//! the same angular layout, at a comfortable stereo distance of 2.2 render units.
use cosmos_manifest::Locale;
use cosmos_render::RenderCamera;
use cosmos_typo::{
    AngularRect, FontRole, GlyphInkMode, GlyphInstance, MsdfAtlas, TetherInstance, TypoFrameOutput,
};
use glam::Vec3;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GuidePage {
    Welcome,
    Help,
    #[default]
    Hidden,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GuideAction {
    Highlights,
    Odyssey,
    Explore,
    Help,
    CloseHelp,
    ToggleTour,
    Previous,
    Next,
    Home,
}

#[derive(Clone, Debug)]
pub struct GuideLine {
    pub text: String,
    pub az_deg: f32,
    pub el_deg: f32,
    pub height_deg: f32,
    pub width_deg: f32,
    pub accent: bool,
}

#[derive(Clone, Debug)]
pub struct GuideButton {
    pub label: String,
    pub action: GuideAction,
    pub rect: AngularRect,
    pub enabled: bool,
}

#[derive(Clone, Debug, Default)]
pub struct GuideFrame {
    pub lines: Vec<GuideLine>,
    pub buttons: Vec<GuideButton>,
    pub hovered: Option<GuideAction>,
    pub hologram: Option<crate::hologram::HologramPose>,
    pub pointer_ray: Option<(Vec3, Vec3)>,
    pub hologram_alpha: f32,
}

pub struct GuideContext<'a> {
    pub page: GuidePage,
    pub locale: Locale,
    pub is_vr: bool,
    pub tour: Option<TourContext<'a>>,
    pub ring_hint: Option<&'a str>,
}

pub struct TourContext<'a> {
    pub title: &'a str,
    pub chapter: &'a str,
    pub stop: usize,
    pub total: usize,
    pub paused: bool,
    pub at_end: bool,
}

fn copy(locale: Locale, en: &'static str, fi: &'static str) -> &'static str {
    if locale == Locale::FiFi { fi } else { en }
}

impl GuideFrame {
    pub fn build(ctx: GuideContext<'_>, camera: &RenderCamera) -> Self {
        let mut out = Self::default();
        let loc = ctx.locale;
        // Fit narrow windows and headset FOVs as well as the desktop showcase aspect.
        let half_h = (camera.fov_y_rad * 0.5).to_degrees();
        let half_w = ((camera.fov_y_rad * 0.5).tan() * camera.aspect)
            .atan()
            .to_degrees();
        let width = (half_w - 3.0).clamp(12.0, 34.0);
        let top = (half_h - 6.0).clamp(16.0, 28.0);
        match ctx.page {
            GuidePage::Welcome => {
                let x = if !ctx.is_vr && half_w >= 45.0 {
                    -half_w + 22.0
                } else {
                    0.0
                };
                let w = (width * 2.0).min(34.0);
                let heading_top = if ctx.is_vr {
                    (half_h - 9.0).clamp(22.0, 33.0)
                } else {
                    23.0
                };
                out.line("HYPERCOSMOS", x, heading_top, 1.0, w, true);
                out.line(
                    copy(loc, "Your first flight", "Ensimmäinen matkasi"),
                    x,
                    heading_top - 5.0,
                    1.8,
                    w,
                    false,
                );
                if !ctx.is_vr {
                    out.line(
                        copy(loc, "Take a guided tour", "Lähde opastetulle matkalle"),
                        x,
                        12.5,
                        1.1,
                        w,
                        false,
                    );
                    out.line(
                        copy(
                            loc,
                            "or explore at your own pace.",
                            "tai tutki omaan tahtiin.",
                        ),
                        x,
                        9.0,
                        1.1,
                        w,
                        false,
                    );
                }
                out.button(
                    copy(loc, "Start highlights", "Aloita kohokohdat"),
                    GuideAction::Highlights,
                    x,
                    1.5,
                    w * 0.45,
                    true,
                );
                out.button(
                    copy(loc, "Explore freely", "Tutki vapaasti"),
                    GuideAction::Explore,
                    x,
                    -5.5,
                    w * 0.45,
                    true,
                );
                out.button(
                    copy(loc, "How to explore", "Näin tutkit"),
                    GuideAction::Help,
                    x,
                    -12.5,
                    w * 0.45,
                    true,
                );
                out.line(
                    if ctx.is_vr {
                        copy(
                            loc,
                            "Point and squeeze to choose",
                            "Osoita ja valitse liipaisimella",
                        )
                    } else {
                        copy(
                            loc,
                            "Click to choose · F1 for help",
                            "Napsauta valintaa · F1: ohjeet",
                        )
                    },
                    x,
                    if ctx.is_vr {
                        -(half_h - 3.0).min(38.0)
                    } else {
                        -20.0
                    },
                    0.95,
                    w,
                    false,
                );
            }
            GuidePage::Help => {
                out.line(
                    copy(loc, "Explore at your own pace", "Tutki omaan tahtiin"),
                    0.0,
                    top,
                    1.7,
                    width * 2.0,
                    true,
                );
                let rows = if ctx.is_vr {
                    [
                        copy(
                            loc,
                            "Point and hold still to hear a subject.",
                            "Osoita ja pysähdy kuullaksesi aiheesta.",
                        ),
                        copy(
                            loc,
                            "Left stick: fly · Right stick: change scale",
                            "Vasen sauva: lennä · Oikea: muuta mittakaavaa",
                        ),
                        copy(
                            loc,
                            "Two grips: pull hands apart to dive inward",
                            "Kaksi otetta: vie käsiä erilleen lähentyäksesi",
                        ),
                        copy(
                            loc,
                            "Palm up: controls · HELP: reopen this guide",
                            "Kämmen ylös: säätimet · HELP: ohjeet",
                        ),
                    ]
                } else {
                    [
                        copy(
                            loc,
                            "Point and hold still to hear a subject.",
                            "Osoita ja pysähdy kuullaksesi aiheesta.",
                        ),
                        copy(
                            loc,
                            "Scroll: change scale · Drag: orbit a subject",
                            "Rulla: mittakaava · Vedä: kierrä kohdetta",
                        ),
                        copy(
                            loc,
                            "WASD / Q E: fly · Hold F + mouse: look around",
                            "WASD / Q E: lennä · F + hiiri: katsele",
                        ),
                        copy(
                            loc,
                            "Tab: controls · Home: Earth · F1: help",
                            "Tab: säätimet · Home: Maa · F1: ohjeet",
                        ),
                    ]
                };
                for (i, line) in rows.iter().enumerate() {
                    out.line(
                        *line,
                        0.0,
                        top - 5.0 - i as f32 * 4.0,
                        1.1,
                        width * 2.0,
                        false,
                    );
                }
                out.line(
                    if ctx.is_vr {
                        copy(
                            loc,
                            "Speech: Spanish, then English or Finnish. Wrist: EN / FI.",
                            "Puhe: espanja, sitten englanti tai suomi. Ranne: EN / FI.",
                        )
                    } else {
                        copy(
                            loc,
                            "Speech: Spanish, then English or Finnish. Use L / EN / FI.",
                            "Puhe: espanja, sitten englanti tai suomi. Vaihda: L / EN / FI.",
                        )
                    },
                    0.0,
                    top - 21.0,
                    0.95,
                    width * 2.0,
                    false,
                );
                out.button(
                    copy(loc, "Highlights", "Kohokohdat"),
                    GuideAction::Highlights,
                    -width * 0.66,
                    top - 27.0,
                    width * 0.29,
                    true,
                );
                out.button(
                    copy(loc, "Full odyssey", "Koko odysseia"),
                    GuideAction::Odyssey,
                    0.0,
                    top - 27.0,
                    width * 0.29,
                    true,
                );
                out.button(
                    copy(loc, "Close help", "Sulje ohje"),
                    GuideAction::CloseHelp,
                    width * 0.66,
                    top - 27.0,
                    width * 0.29,
                    true,
                );
            }
            GuidePage::Hidden => {
                if let Some(tour) = ctx.tour {
                    let status = if tour.at_end {
                        copy(loc, "Tour complete", "Matka päättyi")
                    } else if tour.paused {
                        copy(loc, "Paused", "Tauolla")
                    } else {
                        copy(loc, "Guided tour", "Opastettu matka")
                    };
                    out.line(
                        format!("{status} · {} / {}", tour.stop, tour.total),
                        0.0,
                        top,
                        1.15,
                        width * 2.0,
                        true,
                    );
                    out.line(tour.chapter, 0.0, top - 3.6, 1.35, width * 2.0, false);
                    out.line(tour.title, 0.0, top - 6.6, 0.85, width * 2.0, false);
                    let el = -(half_h - 7.0).clamp(14.0, 28.0);
                    for (x, label, action, enabled) in [
                        (
                            -0.75,
                            copy(loc, "Previous", "Edellinen"),
                            GuideAction::Previous,
                            tour.stop > 1,
                        ),
                        (
                            -0.25,
                            if tour.at_end {
                                copy(loc, "Restart", "Alusta")
                            } else if tour.paused {
                                copy(loc, "Resume", "Jatka")
                            } else {
                                copy(loc, "Pause", "Tauko")
                            },
                            GuideAction::ToggleTour,
                            true,
                        ),
                        (
                            0.25,
                            copy(loc, "Next", "Seuraava"),
                            GuideAction::Next,
                            tour.stop < tour.total,
                        ),
                        (
                            0.75,
                            copy(loc, "Explore", "Tutki"),
                            GuideAction::Explore,
                            true,
                        ),
                    ] {
                        out.button(label, action, width * x, el, width * 0.22, enabled);
                    }
                    if !ctx.is_vr {
                        out.line(copy(loc, "Space: pause · P / N: previous / next · Esc: explore · F1: help", "Välilyönti: tauko · P / N: edellinen / seuraava · Esc: tutki · F1: ohje"), 0.0, el - 4.0, 0.8, width * 2.0, false);
                    }
                }
                if let Some(hint) = ctx.ring_hint {
                    out.line(hint, -width * 0.6, 0.0, 1.05, width, true);
                }
            }
        }
        if ctx.is_vr {
            out.hologram_alpha = 1.0;
            out.hologram = Some(crate::hologram::HologramPose {
                origin: Vec3::ZERO,
                orientation: glam::Quat::from_mat3(&glam::Mat3::from_cols(
                    camera.cam_right,
                    camera.cam_up,
                    -camera.cam_forward,
                )),
            });
            let count = out.buttons.len();
            for (i, button) in out.buttons.iter_mut().enumerate() {
                let span = 2.0 * (half_w - if count > 3 { 10.0 } else { 12.0 }).min(26.0);
                let az = if count <= 1 {
                    0.0
                } else {
                    -span * 0.5 + span * i as f32 / (count - 1) as f32
                };
                let el = match ctx.page {
                    GuidePage::Welcome => -(half_h - 15.0).clamp(18.0, 26.0),
                    GuidePage::Help => -10.0,
                    GuidePage::Hidden => -8.0,
                };
                button.rect = AngularRect::from_deg(
                    az,
                    el,
                    if count > 1 {
                        (span / (count - 1) as f32 * 0.44).min(11.0)
                    } else {
                        11.0
                    },
                    6.5,
                );
            }
        }
        out
    }

    fn line(
        &mut self,
        text: impl Into<String>,
        az_deg: f32,
        el_deg: f32,
        height_deg: f32,
        width_deg: f32,
        accent: bool,
    ) {
        self.lines.push(GuideLine {
            text: text.into(),
            az_deg,
            el_deg,
            height_deg,
            width_deg,
            accent,
        });
    }

    fn button(
        &mut self,
        label: &str,
        action: GuideAction,
        az: f32,
        el: f32,
        half_width: f32,
        enabled: bool,
    ) {
        self.buttons.push(GuideButton {
            label: label.to_owned(),
            action,
            rect: AngularRect::from_deg(az, el, half_width, 2.3),
            enabled,
        });
    }

    pub fn hit_test(
        &self,
        camera: &RenderCamera,
        origin: Vec3,
        direction: Vec3,
    ) -> Option<GuideAction> {
        self.button_at(camera, origin, direction)
            .filter(|b| b.enabled)
            .map(|b| b.action)
    }

    pub fn captures_pointer(&self, camera: &RenderCamera, origin: Vec3, direction: Vec3) -> bool {
        self.button_at(camera, origin, direction).is_some()
    }

    fn button_at(
        &self,
        camera: &RenderCamera,
        origin: Vec3,
        direction: Vec3,
    ) -> Option<&GuideButton> {
        if let Some(hologram) = self.hologram {
            return self
                .buttons
                .iter()
                .filter_map(|b| hologram.button_hit(b, origin, direction).map(|t| (b, t)))
                .min_by(|a, b| a.1.total_cmp(&b.1))
                .map(|(b, _)| b);
        }
        // Intersect the actual 3D guide plane; controller rays can originate away from the head.
        let forward = camera.cam_forward.normalize_or(Vec3::NEG_Z);
        let denom = direction.dot(forward);
        if denom <= 0.001 {
            return None;
        }
        let t = (2.2 - origin.dot(forward)) / denom;
        if t <= 0.0 {
            return None;
        }
        let p = origin + direction * t;
        let az = p.dot(camera.cam_right).atan2(2.2);
        let el = p.dot(camera.cam_up).atan2(2.2);
        self.buttons.iter().find(|b| {
            (az - b.rect.az_center).abs() <= b.rect.half_az
                && (el - b.rect.el_center).abs() <= b.rect.half_el
        })
    }

    pub fn blockers(&self, camera: &RenderCamera) -> Vec<AngularRect> {
        let rects = self
            .lines
            .iter()
            .map(|l| AngularRect::from_deg(l.az_deg, l.el_deg, l.width_deg * 0.5, l.height_deg))
            .chain(self.buttons.iter().map(|b| b.rect));
        let Some(hologram) = self.hologram else {
            return rects.collect();
        };
        rects
            .filter_map(|rect| {
                let mut min = glam::Vec2::splat(f32::INFINITY);
                let mut max = glam::Vec2::splat(f32::NEG_INFINITY);
                for x in [-1.0, 1.0] {
                    for y in [-1.0, 1.0] {
                        let p = hologram.position(
                            rect.az_center + x * rect.half_az,
                            rect.el_center + y * rect.half_el,
                            2.35,
                        );
                        let forward = p.dot(camera.cam_forward);
                        if forward <= 0.05 {
                            return None;
                        }
                        let projected = glam::Vec2::new(
                            p.dot(camera.cam_right).atan2(forward),
                            p.dot(camera.cam_up).atan2(forward),
                        );
                        min = min.min(projected);
                        max = max.max(projected);
                    }
                }
                Some(AngularRect {
                    az_center: (min.x + max.x) * 0.5,
                    el_center: (min.y + max.y) * 0.5,
                    half_az: (max.x - min.x) * 0.5,
                    half_el: (max.y - min.y) * 0.5,
                })
            })
            .collect()
    }

    pub fn append_typography(
        &self,
        atlas: &MsdfAtlas,
        camera: &RenderCamera,
        out: &mut TypoFrameOutput,
    ) {
        if self.lines.is_empty() && self.buttons.is_empty() {
            return;
        }
        if let Some(hologram) = self.hologram {
            hologram.append(self, atlas, out);
            return;
        }
        out.foreground_start = Some((out.glyphs.len(), out.tethers.len()));
        for line in &self.lines {
            draw_line(atlas, camera, line, 0.95, out);
        }
        for button in &self.buttons {
            let selected = self.hovered == Some(button.action);
            let line = GuideLine {
                text: button.label.clone(),
                az_deg: button.rect.az_center.to_degrees(),
                el_deg: button.rect.el_center.to_degrees(),
                height_deg: 1.15,
                width_deg: button.rect.half_az.to_degrees() * 1.8,
                accent: selected,
            };
            draw_line(
                atlas,
                camera,
                &line,
                if button.enabled { 1.0 } else { 0.30 },
                out,
            );
            let el = button.rect.el_center - 1.65_f32.to_radians();
            let left = guide_position(
                camera,
                button.rect.az_center - button.rect.half_az * 0.8,
                el,
            );
            let right = guide_position(
                camera,
                button.rect.az_center + button.rect.half_az * 0.8,
                el,
            );
            out.tethers.push(TetherInstance {
                p0_width: [
                    left.x,
                    left.y,
                    left.z,
                    if selected { 0.003 } else { 0.0015 },
                ],
                p1_alpha: [
                    right.x,
                    right.y,
                    right.z,
                    if button.enabled { 0.8 } else { 0.2 },
                ],
                color_hdr: if selected {
                    [0.50, 1.25, 1.45, 1.0]
                } else {
                    [0.25, 0.60, 0.80, 1.0]
                },
            });
        }
    }
}

fn guide_position(camera: &RenderCamera, az: f32, el: f32) -> Vec3 {
    camera.cam_forward * 2.2
        + camera.cam_right * (az.tan() * 2.2)
        + camera.cam_up * (el.tan() * 2.2)
}

fn draw_line(
    atlas: &MsdfAtlas,
    camera: &RenderCamera,
    line: &GuideLine,
    alpha: f32,
    out: &mut TypoFrameOutput,
) {
    let shaped = atlas.shape_text(FontRole::UiSans, &line.text, None);
    let width_em = shaped
        .lines
        .iter()
        .map(|l| l.width_em)
        .fold(0.0_f32, f32::max);
    // UiSans capital height is about 72% of an em; size the visible letters, not the em box.
    let em = (2.2 * line.height_deg.to_radians() / 0.72)
        .min((2.0 * 2.2 * (0.5 * line.width_deg.to_radians()).tan()) / width_em.max(1.0));
    let center = guide_position(camera, line.az_deg.to_radians(), line.el_deg.to_radians());
    let color = if line.accent {
        [0.55, 1.12, 1.24, alpha]
    } else {
        [1.10, 1.10, 1.08, alpha]
    };
    let right = camera.cam_right * (width_em * em * 0.5 + 0.035);
    let up = camera.cam_up * (em * 0.65);
    out.glyphs.push(GlyphInstance {
        pos_center: [center.x, center.y, center.z, 0.0],
        right_vec: [right.x, right.y, right.z, 0.0],
        up_vec: [up.x, up.y, up.z, 0.0],
        uv_rect: [0.0; 4],
        color_hdr: [0.001, 0.002, 0.004, 0.86],
        halo_color: [0.0; 4],
        ink_params: [0.0, 1.0, 2.0, GlyphInkMode::ControlBackdrop as u8 as f32],
    });
    for glyph in &shaped.glyphs {
        if !glyph.metrics.has_outline {
            continue;
        }
        let b = glyph.metrics.plane_bounds;
        let x =
            (glyph.x_em - shaped.lines[glyph.line_index].width_em * 0.5 + (b[0] + b[2]) * 0.5) * em;
        let y = (glyph.y_em + (b[1] + b[3]) * 0.5 - shaped.ascent_em * 0.35) * em;
        let pos = center + camera.cam_right * x + camera.cam_up * y;
        let right = camera.cam_right * ((b[2] - b[0]) * em * 0.5);
        let up = camera.cam_up * ((b[3] - b[1]) * em * 0.5);
        out.glyphs.push(GlyphInstance {
            pos_center: [pos.x, pos.y, pos.z, 0.005],
            right_vec: [right.x, right.y, right.z, glyph.metrics.px_range],
            up_vec: [up.x, up.y, up.z, 0.12],
            uv_rect: glyph.metrics.uv_rect,
            color_hdr: color,
            halo_color: [0.0, 0.0, 0.0, 0.95],
            ink_params: [0.0, 1.0, 2.0, GlyphInkMode::SpatialAnchor as u8 as f32],
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::Quat;

    #[test]
    fn guide_buttons_fit_and_pick_from_desktop_and_offset_controller_rays() {
        for aspect in [1.0, 16.0 / 9.0, 0.75] {
            let camera = RenderCamera::mono(
                Quat::from_rotation_y(0.6),
                72.0_f32.to_radians(),
                aspect,
                0.02,
            );
            for page in [GuidePage::Welcome, GuidePage::Help, GuidePage::Hidden] {
                let frame = GuideFrame::build(
                    GuideContext {
                        page,
                        locale: Locale::FiFi,
                        is_vr: aspect == 1.0,
                        tour: Some(TourContext {
                            title: "Cosmic highlights",
                            chapter: "Earth",
                            stop: 1,
                            total: 44,
                            paused: true,
                            at_end: false,
                        }),
                        ring_hint: None,
                    },
                    &camera,
                );
                let half_w = ((camera.fov_y_rad * 0.5).tan() * aspect).atan();
                for (i, b) in frame.buttons.iter().enumerate() {
                    assert!(b.rect.az_center.abs() + b.rect.half_az < half_w);
                    assert!(b.rect.el_center.abs() + b.rect.half_el < camera.fov_y_rad * 0.5);
                    for other in frame.buttons.iter().skip(i + 1) {
                        assert!(!b.rect.overlaps(&other.rect));
                    }
                    for origin in [Vec3::ZERO, Vec3::new(0.25, -0.15, 0.10)] {
                        let target = frame.hologram.map_or_else(
                            || guide_position(&camera, b.rect.az_center, b.rect.el_center),
                            |h| h.button_center(b),
                        );
                        let dir = (target - origin).normalize();
                        assert!(frame.captures_pointer(&camera, origin, dir));
                        assert_eq!(
                            frame.hit_test(&camera, origin, dir),
                            b.enabled.then_some(b.action)
                        );
                    }
                }
            }
        }
    }
}
