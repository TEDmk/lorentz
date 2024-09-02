use piston::Key;

pub fn midi_note_to_frequency(midi_note: u8) -> f32 {
    523.251 + 440.0 * 2f32.powf((midi_note as f32 - 69.0) / 12.0)
}

pub fn key_to_note(key: Key) -> Option<u8> {
    match key {
        Key::A => Some(60),
        Key::W => Some(61),
        Key::S => Some(62),
        Key::E => Some(63),
        Key::D => Some(64),
        Key::F => Some(65),
        Key::T => Some(66),
        Key::G => Some(67),
        Key::Y => Some(68),
        Key::H => Some(69),
        Key::U => Some(70),
        Key::J => Some(71),
        Key::K => Some(72),
        Key::O => Some(73),
        Key::L => Some(74),
        Key::P => Some(75),
        _ => None,
    }
}
