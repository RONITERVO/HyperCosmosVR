//! Perceptual RGB -> Spanish color solver and WCAG contrast helper matching App `01` (`01/scene.js`).

/// Standard HSL color in degrees `[0, 360)`, saturation `[0, 1]`, lightness `[0, 1]`.
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HslColor {
    pub h: f64,
    pub s: f64,
    pub l: f64,
}

const PALETTE: &[(f64, &str)] = &[
    (8.0, "rojo"),
    (20.0, "coral"),
    (38.0, "naranja"),
    (52.0, "ámbar"),
    (68.0, "amarillo"),
    (92.0, "verde lima"),
    (138.0, "verde"),
    (158.0, "esmeralda"),
    (178.0, "turquesa"),
    (198.0, "cian"),
    (218.0, "azul cielo"),
    (248.0, "azul"),
    (268.0, "añil"),
    (288.0, "violeta"),
    (310.0, "púrpura"),
    (330.0, "magenta"),
    (345.0, "rosa"),
    (360.0, "rojo"),
];

/// Converts sRGB `u8` components into HSL matching `01/scene.js:rgbToHsl` verbatim
/// (including IEEE remainder `% 6.0` behavior before `+ 360.0` wrap).
pub fn rgb_to_hsl(r: u8, g: u8, b: u8) -> HslColor {
    let rf = r as f64 / 255.0;
    let gf = g as f64 / 255.0;
    let bf = b as f64 / 255.0;
    let max = rf.max(gf).max(bf);
    let min = rf.min(gf).min(bf);
    let l = (max + min) / 2.0;
    let delta = max - min;
    let mut h = 0.0;
    let mut s = 0.0;

    if delta != 0.0 {
        s = delta / (1.0 - (2.0 * l - 1.0).abs());
        if max == rf {
            h = 60.0 * (((gf - bf) / delta) % 6.0);
        } else if max == gf {
            h = 60.0 * ((bf - rf) / delta + 2.0);
        } else {
            h = 60.0 * ((rf - gf) / delta + 4.0);
        }
    }
    if h < 0.0 {
        h += 360.0;
    }

    HslColor { h, s, l }
}

/// Classifies an `(r, g, b)` color into `(base_name, optional_modifier)` matching `01/scene.js:spanishColorName`.
pub fn spanish_color_parts(r: u8, g: u8, b: u8) -> (&'static str, Option<&'static str>) {
    let HslColor { h, s, l } = rgb_to_hsl(r, g, b);

    if l <= 0.055 {
        return ("negro", None);
    }
    if l >= 0.965 {
        return ("blanco", None);
    }
    if s <= 0.09 {
        if l < 0.28 {
            return ("gris oscuro", None);
        }
        if l > 0.76 {
            return ("gris claro", None);
        }
        return ("gris", None);
    }

    let red_hue = !(15.0..345.0).contains(&h);
    if red_hue && l < 0.24 && s > 0.35 {
        return ("granate", None);
    }
    if (18.0..48.0).contains(&h) && l < 0.36 && s > 0.32 {
        return ("marrón", None);
    }
    if (32.0..58.0).contains(&h) && l > 0.76 && s < 0.72 {
        return ("beige", None);
    }
    if (40.0..58.0).contains(&h) && (0.38..=0.67).contains(&l) && s > 0.72 {
        return ("dorado", None);
    }
    if (52.0..82.0).contains(&h) && l < 0.32 {
        return ("verde oliva", None);
    }
    if (205.0..248.0).contains(&h) && l < 0.23 {
        return ("azul marino", None);
    }
    if !(12.0..330.0).contains(&h) && l > 0.72 {
        return ("rosa", None);
    }

    let mut base = "rojo";
    for &(limit, name) in PALETTE {
        if h < limit {
            base = name;
            break;
        }
    }

    let modifier = if l < 0.26 {
        Some("oscuro")
    } else if l > 0.79 && s < 0.76 {
        Some("pálido")
    } else if l > 0.72 {
        Some("claro")
    } else if s < 0.38 {
        Some("apagado")
    } else {
        None
    };

    (base, modifier)
}

/// Returns the canonical Spanish color name used as App `01`'s catalog sourceKey.
pub fn spanish_color_name(r: u8, g: u8, b: u8) -> String {
    match spanish_color_parts(r, g, b) {
        (base, None) => base.to_owned(),
        (base, Some(mod_)) => format!("{base} {mod_}"),
    }
}

/// Computes IEC 61966-2-1 sRGB relative luminance in `[0.0, 1.0]`.
pub fn srgb_relative_luminance(r: u8, g: u8, b: u8) -> f64 {
    fn channel(v: u8) -> f64 {
        let c = v as f64 / 255.0;
        if c <= 0.04045 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b)
}

/// Returns high-contrast ink RGB matching `01/scene.js:contrastFor` (`L > 0.43` -> dark ink `[10, 10, 14]`, else `[255, 255, 255]`).
pub fn contrast_ink(r: u8, g: u8, b: u8) -> [u8; 3] {
    if srgb_relative_luminance(r, g, b) > 0.43 {
        [10, 10, 14]
    } else {
        [255, 255, 255]
    }
}
