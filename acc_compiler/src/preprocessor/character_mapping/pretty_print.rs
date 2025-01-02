use super::mapper::MappedChar;

/// Prints characters in MappedChar slice as if there was no metadata
pub fn mapped_chars_to_str(chars: &[MappedChar]) -> String {
    chars
        .iter()
        .filter_map(|chr| chr.chr())
        .map(|chr| chr as char)
        .collect()
}

/// Prints characters in MappedChar slice with metadata, separated by newlines
pub fn verbose_mapped_chars_to_str(chars: &[MappedChar]) -> String {
    chars
        .iter()
        .map(|chr| {
            let char_part = if let Some(c) = chr.chr() {
                format!("{}", c as char)
            } else {
                "None".to_string()
            };

            format!("{} ({}, {})\n", char_part, chr.pos().0, chr.pos().1)
        })
        .collect::<String>()
}
