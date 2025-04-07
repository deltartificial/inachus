use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ANSI_REGEXP: Regex = Regex::new(r"\x1b\[[0-9;]*m").unwrap();
}

pub fn strip_ansi(s: &str) -> String {
    ANSI_REGEXP.replace_all(s, "").into_owned()
}

pub fn pad_right_ansi_aware(colored: &str, width: usize) -> String {
    let raw = strip_ansi(colored);
    let char_count = raw.chars().count();
    let padding = if width > char_count {
        width - char_count
    } else {
        0
    };

    format!("{}{}", colored, " ".repeat(padding))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi() {
        let colored = "\x1b[32mHello\x1b[0m";
        assert_eq!(strip_ansi(colored), "Hello");
    }

    #[test]
    fn test_pad_right_ansi_aware() {
        let colored = "\x1b[32mHello\x1b[0m";
        let padded = pad_right_ansi_aware(colored, 10);
        assert_eq!(padded.len() - colored.len(), 5);
        assert_eq!(strip_ansi(&padded).len(), 10);
    }
}
