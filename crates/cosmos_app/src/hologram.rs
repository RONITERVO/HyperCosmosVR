//! Room-anchored holographic guidance with real depth, parallax and scene occlusion.
use crate::guide::{GuideButton, GuideFrame, GuideLine};
use cosmos_typo::{
    FontRole, GlyphInkMode, GlyphInstance, MsdfAtlas, TetherInstance, TypoFrameOutput,
};
use glam::{Quat, Vec3};

#[derive(Clone, Copy, Debug)]
pub struct HologramPose {
    /// Captured tracking-space origin expressed relative to the current head position.
    pub origin: Vec3,
    pub orientation: Quat,
}

impl Default for HologramPose {
    fn default() -> Self {
        Self {
            origin: Vec3::ZERO,
            orientation: Quat::IDENTITY,
        }
    }
}

impl HologramPose {
    pub fn position(&self, az: f32, el: f32, depth: f32) -> Vec3 {
        self.origin
            + self.orientation * Vec3::new(az.sin() * depth, el.tan() * depth, -az.cos() * depth)
    }

    pub fn button_depth(button: &GuideButton) -> f32 {
        1.95 + 0.28 * (button.rect.az_center * 2.0).cos()
    }

    pub fn button_center(&self, button: &GuideButton) -> Vec3 {
        self.position(
            button.rect.az_center,
            button.rect.el_center,
            Self::button_depth(button),
        )
    }

    pub fn button_hit(&self, button: &GuideButton, origin: Vec3, direction: Vec3) -> Option<f32> {
        let center = self.button_center(button);
        let az = button.rect.az_center;
        let normal = self.orientation * Vec3::new(-az.sin(), 0.0, az.cos());
        let denom = direction.dot(normal);
        if denom >= -0.001 {
            return None;
        }
        let distance = (center - origin).dot(normal) / denom;
        if distance <= 0.0 {
            return None;
        }
        let delta = origin + direction * distance - center;
        let right = self.orientation * Vec3::new(az.cos(), 0.0, az.sin());
        let up = self.orientation * Vec3::Y;
        let depth = Self::button_depth(button);
        (delta.dot(right).abs() <= depth * button.rect.half_az.tan()
            && delta.dot(up).abs() <= depth * button.rect.half_el.tan())
        .then_some(distance)
    }

    pub fn append(&self, frame: &GuideFrame, atlas: &MsdfAtlas, out: &mut TypoFrameOutput) {
        for line in &frame.lines {
            self.lettering(atlas, line, 2.35, 0.95 * frame.hologram_alpha, out);
        }
        for button in &frame.buttons {
            let selected = frame.hovered == Some(button.action);
            let depth = Self::button_depth(button);
            let center = self.position(
                button.rect.az_center,
                button.rect.el_center + 2.0_f32.to_radians(),
                depth,
            );
            let alpha = if button.enabled { 0.86 } else { 0.22 } * frame.hologram_alpha;
            let color = if selected {
                [0.45, 1.0, 1.2, alpha]
            } else {
                [0.10, 0.50, 0.72, alpha]
            };
            // Interlocking great circles form an actual volume, visible from the side.
            for plane in 0..3 {
                for segment in 0..48 {
                    let point = |i: usize| {
                        let (s, c) = (i as f32 * std::f32::consts::TAU / 48.0).sin_cos();
                        let v = match plane {
                            0 => Vec3::new(c, s, 0.0),
                            1 => Vec3::new(c, 0.0, s),
                            _ => Vec3::new(0.0, c, s),
                        };
                        center + self.orientation * v * if selected { 0.095 } else { 0.080 }
                    };
                    filament(point(segment), point(segment + 1), color, out);
                }
            }
            self.lettering(
                atlas,
                &GuideLine {
                    text: button.label.clone(),
                    az_deg: button.rect.az_center.to_degrees(),
                    el_deg: button.rect.el_center.to_degrees() - 3.4,
                    height_deg: 1.35,
                    width_deg: button.rect.half_az.to_degrees() * 1.9,
                    accent: selected,
                },
                depth,
                alpha,
                out,
            );
        }
        if let (Some(first), Some(last)) = (frame.buttons.first(), frame.buttons.last()) {
            for i in 0..48 {
                let point = |i: usize| {
                    self.position(
                        first.rect.az_center
                            + (last.rect.az_center - first.rect.az_center) * i as f32 / 48.0,
                        first.rect.el_center - 9.0_f32.to_radians(),
                        2.12,
                    )
                };
                filament(
                    point(i),
                    point(i + 1),
                    [0.06, 0.28, 0.42, 0.55 * frame.hologram_alpha],
                    out,
                );
            }
        }
        if let Some((origin, direction)) = frame.pointer_ray
            && let Some(distance) = frame
                .buttons
                .iter()
                .filter_map(|b| self.button_hit(b, origin, direction))
                .min_by(f32::total_cmp)
        {
            filament(
                origin,
                origin + direction * distance,
                [0.12, 0.75, 1.0, 0.7 * frame.hologram_alpha],
                out,
            );
        }
    }

    /// Curved, scene-occluded instrument disclosure without a panel or backplate.
    pub fn append_observation_label(&self, text: &str, atlas: &MsdfAtlas, out: &mut TypoFrameOutput) {
        self.lettering(atlas, &GuideLine {
            text: text.to_owned(),
            az_deg: -22.0,
            el_deg: -23.0,
            height_deg: 1.1,
            width_deg: 32.0,
            accent: false,
        }, 2.35, 0.78, out);
    }

    fn lettering(
        &self,
        atlas: &MsdfAtlas,
        line: &GuideLine,
        depth: f32,
        alpha: f32,
        out: &mut TypoFrameOutput,
    ) {
        let shaped = atlas.shape_text(FontRole::UiSans, &line.text, None);
        let width = shaped
            .lines
            .iter()
            .map(|l| l.width_em)
            .fold(0.0_f32, f32::max)
            .max(1.0);
        let em = (depth * line.height_deg.to_radians() / 0.72)
            .min(depth * line.width_deg.to_radians() / width);
        for glyph in &shaped.glyphs {
            if !glyph.metrics.has_outline {
                continue;
            }
            let b = glyph.metrics.plane_bounds;
            let x = (glyph.x_em - shaped.lines[glyph.line_index].width_em * 0.5
                + (b[0] + b[2]) * 0.5)
                * em;
            let y = (glyph.y_em + (b[1] + b[3]) * 0.5 - shaped.ascent_em * 0.35) * em;
            let az = line.az_deg.to_radians() + x / depth;
            let pos =
                self.position(az, line.el_deg.to_radians(), depth) + self.orientation * Vec3::Y * y;
            let right =
                self.orientation * Vec3::new(az.cos(), 0.0, az.sin()) * ((b[2] - b[0]) * em * 0.5);
            let up = self.orientation * Vec3::Y * ((b[3] - b[1]) * em * 0.5);
            out.glyphs.push(GlyphInstance {
                pos_center: [pos.x, pos.y, pos.z, 0.0],
                right_vec: [right.x, right.y, right.z, glyph.metrics.px_range],
                up_vec: [up.x, up.y, up.z, 0.24],
                uv_rect: glyph.metrics.uv_rect,
                color_hdr: if line.accent {
                    [0.16, 0.74, 0.90, alpha]
                } else {
                    [0.62, 0.82, 0.90, alpha]
                },
                halo_color: [0.0, 0.0, 0.0, 0.9],
                ink_params: [0.0, 1.0, 2.0, GlyphInkMode::SpatialAnchor as u8 as f32],
            });
        }
    }
}

fn filament(a: Vec3, b: Vec3, color: [f32; 4], out: &mut TypoFrameOutput) {
    out.tethers.push(TetherInstance {
        p0_width: [a.x, a.y, a.z, 0.0007],
        p1_alpha: [b.x, b.y, b.z, color[3]],
        color_hdr: [color[0], color[1], color[2], 1.0],
    });
}
