//! Terminal utilities for width detection and line truncation.

use terminal_size::{terminal_size, Width};
use unicode_width::UnicodeWidthStr;

/// Minimum terminal width below which truncation is disabled.
const MIN_TRUNCATION_WIDTH: usize = 40;

/// Get the current terminal width, or None if stdout is not a TTY (e.g., piped).
pub fn terminal_width() -> Option<usize> {
    terminal_size().map(|(Width(w), _)| w as usize)
}

/// Strip ANSI escape sequences from a string.
/// Handles SGR sequences (ESC[...m) used by the `colored` crate.
fn strip_ansi(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '\x1b' {
            // Start of escape sequence
            if chars.peek() == Some(&'[') {
                chars.next(); // consume '['
                // Consume until we hit a letter (the command)
                while let Some(&next) = chars.peek() {
                    chars.next();
                    if next.is_ascii_alphabetic() {
                        break;
                    }
                }
            }
        } else {
            result.push(c);
        }
    }

    result
}

/// Calculate display width of a string (Unicode-aware, ignores ANSI codes).
pub fn display_width(s: &str) -> usize {
    strip_ansi(s).width()
}

/// Truncate a string to fit within max_width display columns.
/// Appends "..." if truncated. Handles Unicode and ANSI codes properly.
///
/// Returns the original string if it fits, or a truncated version with "...".
/// Note: This strips ANSI codes in the truncated output.
pub fn truncate_to_width(s: &str, max_width: usize) -> String {
    let stripped = strip_ansi(s);
    let current_width = stripped.width();

    if current_width <= max_width {
        return s.to_string();
    }

    if max_width < 4 {
        return ".".repeat(max_width);
    }

    let target_width = max_width - 3; // Reserve space for "..."
    let mut truncated = String::new();
    let mut width = 0;

    for c in stripped.chars() {
        let char_width = unicode_width::UnicodeWidthChar::width(c).unwrap_or(0);
        if width + char_width > target_width {
            break;
        }
        truncated.push(c);
        width += char_width;
    }

    truncated.push_str("...");
    truncated
}

/// A line formatter that handles truncation based on terminal width.
pub struct LineFormatter {
    max_width: Option<usize>,
}

impl LineFormatter {
    /// Create a new LineFormatter.
    /// If max_width is None, no truncation is performed.
    pub fn new(max_width: Option<usize>) -> Self {
        Self { max_width }
    }

    /// Create a LineFormatter that auto-detects terminal width.
    /// Returns a formatter with no truncation if:
    /// - stdout is not a TTY (piped output)
    /// - terminal is narrower than MIN_TRUNCATION_WIDTH
    pub fn auto() -> Self {
        let max_width = terminal_width().filter(|&w| w >= MIN_TRUNCATION_WIDTH);
        Self { max_width }
    }

    /// Get the effective max width, if any.
    pub fn max_width(&self) -> Option<usize> {
        self.max_width
    }

    /// Calculate available width after accounting for a fixed-width prefix.
    /// Returns None if no truncation should be performed or if prefix consumes all space.
    pub fn available_width(&self, prefix_width: usize) -> Option<usize> {
        self.max_width
            .map(|w| w.saturating_sub(prefix_width))
            .filter(|&w| w >= 10) // Need at least 10 chars for meaningful content
    }

    /// Truncate content to fit within available width after prefix.
    /// Returns the original content if no truncation needed.
    pub fn truncate(&self, content: &str, prefix_width: usize) -> String {
        match self.available_width(prefix_width) {
            Some(available) => truncate_to_width(content, available),
            None => content.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_ansi_no_codes() {
        assert_eq!(strip_ansi("hello"), "hello");
    }

    #[test]
    fn test_strip_ansi_with_codes() {
        // Red text: ESC[31m ... ESC[0m
        assert_eq!(strip_ansi("\x1b[31mred\x1b[0m"), "red");
    }

    #[test]
    fn test_strip_ansi_bold() {
        // Bold: ESC[1m ... ESC[0m
        assert_eq!(strip_ansi("\x1b[1mbold\x1b[0m"), "bold");
    }

    #[test]
    fn test_display_width_ascii() {
        assert_eq!(display_width("hello"), 5);
    }

    #[test]
    fn test_display_width_with_ansi() {
        assert_eq!(display_width("\x1b[31mhello\x1b[0m"), 5);
    }

    #[test]
    fn test_display_width_unicode() {
        // CJK characters are typically 2 columns wide
        assert_eq!(display_width("日本"), 4);
    }

    #[test]
    fn test_truncate_short_string() {
        assert_eq!(truncate_to_width("hi", 10), "hi");
    }

    #[test]
    fn test_truncate_exact_fit() {
        assert_eq!(truncate_to_width("hello", 5), "hello");
    }

    #[test]
    fn test_truncate_needs_ellipsis() {
        assert_eq!(truncate_to_width("hello world", 8), "hello...");
    }

    #[test]
    fn test_truncate_very_narrow() {
        assert_eq!(truncate_to_width("hello", 3), "...");
    }

    #[test]
    fn test_line_formatter_no_truncation() {
        let fmt = LineFormatter::new(None);
        assert_eq!(fmt.truncate("hello world", 0), "hello world");
    }

    #[test]
    fn test_line_formatter_with_width() {
        let fmt = LineFormatter::new(Some(20));
        assert_eq!(fmt.truncate("hello world", 10), "hello w...");
    }

    #[test]
    fn test_line_formatter_prefix_too_large() {
        let fmt = LineFormatter::new(Some(20));
        // Prefix of 15 leaves only 5 chars, which is < 10 minimum
        assert_eq!(fmt.truncate("hello world", 15), "hello world");
    }
}
