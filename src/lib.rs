#![deny(missing_docs)]
//! Given a Duration, lossily format it like in 'N days ago'. Parsing it back to Duration is not supported yet. English only, at least for now.

use std::time::Duration;

/// Formatting style for `format`
pub enum Style {
    /// Long format, like "~2 years ago"
    LONG,
    /// Short format, like "02Yea"
    SHORT,
}

const S_IN_MNTH: u64 = 2628003; // 2628002,88 seconds according to Google

/// Do the formatting
///
/// ```
/// extern crate timeago;
/// assert_eq!(timeago::format(std::time::Duration::new(3600, 0), timeago::Style::LONG), "1 hour ago");
/// ```
pub fn format(d: Duration, style: Style) -> String {
    let s = d.as_secs();
    let n = d.subsec_nanos();
    match style {
        Style::LONG => {
            match s {
                0 => {
                    match n {
                        0 => "now".into(),
                        1 => "1 nanosecond ago".into(),
                        x if x > 1 && x < 1000 => format!("{} nanoseconds ago", n),
                        x if x >= 1000 && x < 2000 => "1 microsecond ago".into(),
                        x if x >= 2000 && x < 1000_000 => format!("{} milliseconds ago", n / 1000),
                        x if x >= 1000_000 && x < 2000_000 => "1 millisecond ago".into(),
                        x if x >= 2000_000 && x < 1000_000_000 => {
                            format!("{} milliseconds ago", n / 1000_000)
                        }
                        _ => panic!("Invalid duration passed to timeago::format"),
                    }
                }
                1 => "1 second ago".into(),
                x if x > 1 && x < 60 => format!("{} seconds ago", x),
                x if x >= 60 && x < 120 => "1 minute ago".into(),
                x if x >= 120 && x < 60 * 60 => format!("{} minutes ago", x / 60),
                x if x >= 60 * 60 && x < 60 * 60 * 2 => "1 hour ago".into(),
                x if x >= 60 * 60 * 2 && x < 60 * 60 * 24 => format!("{} hours ago", x / 60 / 60),
                x if x >= 60 * 60 * 24 && x < 60 * 60 * 24 * 2 => "1 day ago".into(),
                x if x >= 60 * 60 * 24 * 2 && x < S_IN_MNTH => {
                    format!("{} days ago", x / 60 / 60 / 24)
                }
                x if x >= S_IN_MNTH && x < 2 * S_IN_MNTH => "~1 month ago".into(),
                x if x >= 2 * S_IN_MNTH && x < 12 * S_IN_MNTH => {
                    format!("~{} months ago", x / S_IN_MNTH)
                }
                x if x >= 12 * S_IN_MNTH && x < 12 * 2 * S_IN_MNTH => "~1 year ago".into(),
                x => format!("~{} years ago", x / 12 / S_IN_MNTH),
            }
        }
        Style::SHORT => {
            match s {
                0 => " now ".into(),
                x if x > 0 && x < 60 => format!("{:02}sec", x),
                x if x >= 60 && x < 60 * 60 => format!("{:02}min", x / 60),
                x if x >= 60 * 60 && x < 60 * 60 * 24 => format!("{:02}hou", x / 60 / 60),
                x if x >= 60 * 60 * 24 && x < S_IN_MNTH => format!("{:02}day", x / 60 / 60 / 24),
                x if x >= S_IN_MNTH && x < 12 * S_IN_MNTH => format!("{:02}Mon", x / S_IN_MNTH),
                x if x >= 12 * S_IN_MNTH && x <= 99 * 12 * S_IN_MNTH => {
                    format!("{:02}Yea", x / 12 / S_IN_MNTH)
                }
                _ => " OLD ".into(),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::time::Duration;
    use super::{Style, format};

    fn dns(secs: u64) -> Duration {
        Duration::from_secs(secs)
    }
    fn dn(secs: u64, nanos: u32) -> Duration {
        Duration::new(secs, nanos)
    }
    fn fmtl(d: Duration) -> String {
        format(d, Style::LONG)
    }
    fn fmts(d: Duration) -> String {
        format(d, Style::SHORT)
    }

    #[test]
    fn test_long() {
        assert_eq!(fmtl(dns(0)), "now");
        assert_eq!(fmtl(dn(0, 500_000_000)), "500 milliseconds ago");
        assert_eq!(fmtl(dns(1)), "1 second ago");
        assert_eq!(fmtl(dn(1, 500_000_000)), "1 second ago");
        assert_eq!(fmtl(dns(59)), "59 seconds ago");
        assert_eq!(fmtl(dns(60)), "1 minute ago");
        assert_eq!(fmtl(dns(65)), "1 minute ago");
        assert_eq!(fmtl(dns(119)), "1 minute ago");
        assert_eq!(fmtl(dns(120)), "2 minutes ago");
        assert_eq!(fmtl(dns(3599)), "59 minutes ago");
        assert_eq!(fmtl(dns(3600)), "1 hour ago");
        assert_eq!(fmtl(dns(1000_000)), "11 days ago");
        assert_eq!(fmtl(dns(1000_000_000)), "~31 years ago");
    }

    #[test]
    fn test_short() {
        assert_eq!(fmts(dns(0)), " now ");
        assert_eq!(fmts(dn(0, 500_000_000)), " now ");
        assert_eq!(fmts(dns(1)), "01sec");
        assert_eq!(fmts(dn(1, 500_000_000)), "01sec");
        assert_eq!(fmts(dns(59)), "59sec");
        assert_eq!(fmts(dns(60)), "01min");
        assert_eq!(fmts(dns(65)), "01min");
        assert_eq!(fmts(dns(119)), "01min");
        assert_eq!(fmts(dns(120)), "02min");
        assert_eq!(fmts(dns(3599)), "59min");
        assert_eq!(fmts(dns(3600)), "01hou");
        assert_eq!(fmts(dns(1000_000)), "11day");
        assert_eq!(fmts(dns(1000_000_000)), "31Yea");
    }
}
