pub const KEY_NAMES: [&str; 24] = [
    "C Major", "G Major", "D Major", "A Major", "E Major", "B Major", "F# Major", "C# Major",
    "F Major", "Bb Major", "Eb Major", "Ab Major", "A Minor", "E Minor", "B Minor", "F# Minor",
    "C# Minor", "G# Minor", "D# Minor", "A# Minor", "D Minor", "G Minor", "C Minor", "F Minor",
];

pub fn get_key_name(key_index: usize) -> &'static str {
    KEY_NAMES.get(key_index).unwrap_or(&"Unknown")
}

pub const NOTE_NAMES: [&str; 12] =
    ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];

pub fn get_note_name(note_index: i32) -> &'static str {
    if note_index >= 0 && note_index < 12 {
        NOTE_NAMES[note_index as usize]
    } else {
        "Unknown"
    }
}

pub fn format_duration(seconds: f32) -> String {
    let minutes = (seconds / 60.0) as u32;
    let remaining_seconds = seconds % 60.0;

    if minutes > 0 {
        format!("{}m {:.1}s", minutes, remaining_seconds)
    } else {
        format!("{:.1}s", remaining_seconds)
    }
}

pub fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

pub fn format_sample_rate(sample_rate: f32) -> String {
    if sample_rate >= 1000.0 {
        format!("{:.1}kHz", sample_rate / 1000.0)
    } else {
        format!("{}Hz", sample_rate as i32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_names() {
        assert_eq!(get_key_name(0), "C Major");
        assert_eq!(get_key_name(12), "A Minor");
        assert_eq!(get_key_name(100), "Unknown");
    }

    #[test]
    fn test_note_names() {
        assert_eq!(get_note_name(0), "C");
        assert_eq!(get_note_name(11), "B");
        assert_eq!(get_note_name(-1), "Unknown");
        assert_eq!(get_note_name(12), "Unknown");
    }

    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(30.5), "30.5s");
        assert_eq!(format_duration(90.0), "1m 30.0s");
        assert_eq!(format_duration(125.7), "2m 5.7s");
    }

    #[test]
    fn test_format_file_size() {
        assert_eq!(format_file_size(512), "512 B");
        assert_eq!(format_file_size(1536), "1.5 KB");
        assert_eq!(format_file_size(1048576), "1.0 MB");
    }

    #[test]
    fn test_format_sample_rate() {
        assert_eq!(format_sample_rate(44100.0), "44.1kHz");
        assert_eq!(format_sample_rate(48000.0), "48.0kHz");
        assert_eq!(format_sample_rate(800.0), "800Hz");
    }
}
