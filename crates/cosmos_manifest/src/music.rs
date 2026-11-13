//! Pitch, frequency, octave, and timbre mapping matching App `02` (`02/scene.js`).

/// Lowest MIDI note (`C3`).
pub const MIDI_MIN: u8 = 48;
/// Total semitone slots across the 4-octave interactive field (`C3..B6`, MIDI `48..=95`).
pub const MIDI_COUNT: usize = 48;

/// Maps normalized horizontal coordinate `[0, 1]` to a MIDI note in `48..=95`.
pub fn midi_for_x(x_norm: f32) -> u8 {
    let clamped = (x_norm as f64).clamp(0.0, 0.999_999);
    MIDI_MIN + (clamped * MIDI_COUNT as f64).floor() as u8
}

/// 12-TET equal-temperament frequency in Hz (`A4 = 440 Hz` at MIDI `69`).
pub fn midi_to_hz(midi: u8) -> f64 {
    440.0 * 2_f64.powf((midi as f64 - 69.0) / 12.0)
}

/// Pitch class index `0..=11` (`0 = C / do` .. `11 = B / si`).
pub const fn pitch_class(midi: u8) -> u8 {
    midi % 12
}

/// Scientific pitch octave (`MIDI 48 -> octave 3`, `MIDI 95 -> octave 6`).
pub const fn octave_for_midi(midi: u8) -> i32 {
    (midi as i32 / 12) - 1
}

/// Spectral hue in `[0, 360)` matching `02/scene.js:colorForMidi`.
pub fn color_hue_for_midi(midi: u8) -> f32 {
    ((midi % 12) as f32 / 12.0) * 360.0
}

/// Maps normalized vertical coordinate `[0, 1]` to timbre band index `0..=3` matching `02/scene.js:timbreForY`.
pub fn timbre_index_for_y(y_norm: f32) -> usize {
    let n = y_norm.clamp(0.0, 1.0);
    if n < 0.25 {
        0
    } else if n < 0.5 {
        1
    } else if n < 0.75 {
        2
    } else {
        3
    }
}
