use piston::Key;

pub fn midi_note_to_frequency(midi_note: u8) -> f32 {
    523.251 + 440.0 * 2f32.powf((midi_note as f32 - 69.0) / 12.0)
}

pub fn key_to_note(key: Key) -> Option<f32> {
    match key {
        Key::A => Some(523.25),
        Key::W => Some(554.36),
        Key::S => Some(587.33),
        Key::E => Some(622.25),
        Key::D => Some(659.25),
        Key::F => Some(698.45),
        Key::T => Some(739.98),
        Key::G => Some(7.),
        Key::Y => Some(8.),
        Key::H => Some(9.),
        Key::U => Some(10.),
        Key::J => Some(11.),
        Key::K => Some(12.),
        Key::O => Some(13.),
        Key::L => Some(14.),
        Key::P => Some(15.),
        _ => None,
    }
}
