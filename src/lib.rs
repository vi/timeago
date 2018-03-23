#![deny(missing_docs)]
#![allow(dead_code)]
//! Given a Duration, lossily format it like in 'N days ago'. Parsing it back to Duration is not supported yet. English only, at least for now.

use std::time::Duration;

/// Natural language to use for the formatting
/// TODO: connect it to `isolang` crate somehow
#[allow(missing_docs)]
pub trait Language {
    /// What to emit by default if value is too high
    fn too_low(&self) -> &'static str;
    
    /// What to emit by default if value is too low
    fn too_high(&self) -> &'static str;
    
    /// Chunk of text to put at the end by default
    fn ago(&self) -> &'static str;
    
    /// Get word representing the given time unit, for using with `x` number
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str;
}

/// Default language for timeago
#[derive(Default)]
pub struct English;
impl Language for English {
    fn too_low (&self) -> &'static str { "now" }
    fn too_high(&self) -> &'static str { "old" }
    fn ago(&self)      -> &'static str { "ago" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "nanosecond",
                Microseconds  =>  "microsecond",
                Milliseconds  =>  "millisecond",
                Seconds       =>  "second",
                Minutes       =>  "minute",
                Hours         =>  "hour",
                Days          =>  "day",
                Weeks         =>  "week",
                Months        =>  "month",
                Years         =>  "year",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanoseconds",
                Microseconds  =>  "microseconds",
                Milliseconds  =>  "milliseconds",
                Seconds       =>  "seconds",
                Minutes       =>  "minutes",
                Hours         =>  "hours",
                Days          =>  "days",
                Weeks         =>  "weeks",
                Months        =>  "months",
                Years         =>  "years",
            }
        }
    }
}

/// Various units of time to specify as maximum or minimum.
/// Note that calculations are approximate, not calendar-based.
#[allow(missing_docs)]
#[derive(Debug,Copy,Clone,Eq,PartialEq,Ord,PartialOrd,Hash)]
pub enum TimeUnit {
    Nanoseconds,
    Microseconds,
    Milliseconds,
    Seconds,
    Minutes,
    Hours,
    Days,
    Weeks,
    Months,
    Years,
}

impl TimeUnit {
    /// Get `std::time::Duration` corresponding to minimum duration that is representable by this time unit.
    /// TODO: example
    pub fn min_duration(&self) -> Duration {
        use TimeUnit::*;
        match *self {
            Nanoseconds => Duration::new(0, 1),
            Microseconds => Duration::new(0, 1000),
            Milliseconds => Duration::new(0, 1000_000),
            Seconds => Duration::new(1, 0),
            Minutes => Duration::new(60,0),
            Hours => Duration::new(60*60, 0),
            Days => Duration::new(24*60*60, 0),
            Weeks => Duration::new(7*24*60*60, 0),
            Months => Duration::new(S_IN_MNTH, 0),
            Years => Duration::new(S_IN_MNTH * 12, 0),
        }
    }
    
    /// TODO
    pub fn bigger_unit(&self) -> Option<TimeUnit> {
        use TimeUnit::*;
        match *self {
            Nanoseconds  => Some(Microseconds),
            Microseconds => Some(Milliseconds),
            Milliseconds => Some(Seconds     ),
            Seconds      => Some(Minutes     ),
            Minutes      => Some(Hours       ),
            Hours        => Some(Days        ),
            Days         => Some(Weeks       ),
            Weeks        => Some(Months      ),
            Months       => Some(Years       ),
            Years        => None,
        }
    }
    
    /// TODO
    pub fn smaller_unit(&self) -> Option<TimeUnit> {
        use TimeUnit::*;
        match *self {
            Nanoseconds  => None,
            Microseconds => Some(Nanoseconds ),
            Milliseconds => Some(Microseconds),
            Seconds      => Some(Milliseconds),
            Minutes      => Some(Seconds     ),
            Hours        => Some(Minutes     ),
            Days         => Some(Hours       ),
            Weeks        => Some(Days        ),
            Months       => Some(Weeks       ),
            Years        => Some(Months      ),
        }
    }
}

/// Main formatter struct. Build it with new() and maybe modify some options, then use convert
/// ```
/// let f = timeago::Formatter::new();
/// let d = std::time::Duration::from_secs(3600);
/// assert_eq!(f.convert(d), "1 hour ago");
/// ```
pub struct Formatter<L : Language = English>  {
    lang: L,
    num_items: usize,
    min_unit: TimeUnit,
    max_unit: TimeUnit,
    too_low: Option<&'static str>,
    too_high: Option<&'static str>,
    ago: Option<&'static str>,
}

impl Formatter {
    /// Constructor for some default formatting in English
    pub fn new() -> Formatter {
        Formatter::with_language(English)
    }
}
impl <L:Language> Formatter<L> {
    /// Constructor for some default formatting with specified language instance
    pub fn with_language(l: L) -> Self  {
        Formatter {
            lang: l,
            num_items: 1,
            min_unit: TimeUnit::Seconds,
            max_unit: TimeUnit::Years,
            too_low: None,
            too_high: None,
            ago: None,
        }
    }
    
    
    /// Set number of time unit items to emit (like 1 item for "1 year" or 3 items for "1 year and 3 months and 17 days")
    /// TODO: example
    pub fn num_items(&mut self, x: usize) -> &mut Self {
        assert!(x > 0);
        self.num_items = x;
        self
    }
    
    /// Set maximum used unit. Above that it just emits "old" (or some analogue).
    /// TODO: example
    pub fn max_unit(&mut self, x: TimeUnit) -> &mut Self {
        self.max_unit = x;
        self
    }
    
    /// Set minimum used unit. Durations below minimally representable by that unit emit `too_low` value like "now", or like "0 days" instead of "2 minutes".
    /// TODO: example
    pub fn min_unit(&mut self, x: TimeUnit) -> &mut Self {
        self.min_unit = x;
        self
    }
    
    /// Override what is used instead of "now" for too low units.
    /// Setting this to special value `"0"` causes to emit output like "0 days", depending on `min_unit` property.
    /// TODO: example
    pub fn too_low(&mut self, x: &'static str) -> &mut Self {
        self.too_low = Some(x);
        self
    }
    
    /// Override what is used instead of "old" for too high units.
    /// TODO: example
    pub fn too_high(&mut self, x: &'static str) -> &mut Self {
        self.too_high = Some(x);
        self
    }
    
    /// Override what is used instead of "ago".
    /// Empty string literal `""` is a bit special in the space handling.
    /// TODO: example
    pub fn ago(&mut self, x: &'static str) -> &mut Self {
        self.ago = Some(x);
        self
    }
    
    /// Do the actual conversion. Not implemented currently, use version 0.0.2
    /// TODO: example
    pub fn convert(&self, d: Duration) -> String {
        let dtu = dominant_time_unit(d);
        let (x, _rem) = split_up(d, dtu);
        format!("{} {} {}", x, self.lang.get_word(dtu, x), self.lang.ago())
    }
}

fn dominant_time_unit(d:Duration) -> TimeUnit {
    use TimeUnit::*;
    
    match d {
        x if x <= Nanoseconds .min_duration() => Nanoseconds ,
        x if x <= Microseconds.min_duration() => Microseconds,
        x if x <= Milliseconds.min_duration() => Milliseconds,
        x if x <= Seconds     .min_duration() => Seconds     ,
        x if x <= Minutes     .min_duration() => Minutes     ,
        x if x <= Hours       .min_duration() => Hours       ,
        x if x <= Days        .min_duration() => Days        ,
        x if x <= Weeks       .min_duration() => Weeks       ,
        x if x <= Months      .min_duration() => Months      ,
        _ => Years,
    }
}

fn divmod64(a: u64, b: u64) -> (u64, u64) {
    (a / b, a % b)
}
fn divmod32(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

fn split_up(d:Duration, tu: TimeUnit) -> (u64, Duration) {
    let s = d.as_secs();
    let n = d.subsec_nanos();
    
    let tud = tu.min_duration();
    let tus = tud.as_secs();
    let tun = tud.subsec_nanos();
    
    assert!((tus == 0) ^ (tun == 0));
    
    if tus != 0 {
        if s == 0 {
            (0, d)
        } else {
            let (c, s2) = divmod64(s, tus);
            (c, Duration::new(s2, n))
        }
    } else {
        if s == 0 {
            let (c, n2) = divmod32(n, tun);
            (c.into(), Duration::new(0, n2))
        } else {
            // tricky case: may be overflow
            unimplemented!()
        }
    }
}

/// A simplified formatter, resulting in short strings like "02Yea" or " now " or "07min".
/// Designed to always give 5-character strings.
pub fn format_5chars(d: Duration) -> String {
    let s = d.as_secs();
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

/// Simple formatting style for deprecated `format`.
#[deprecated(since="0.1.0",note="Use Formatter or format_5chars")]
pub enum Style {
    /// Long format, like "~2 years ago"
    LONG,
    /// Human format, like LONG but makes less than 1 second as `just now`
    HUMAN,
    /// Short format, like "02Yea". Should be exactly 5 characters.
    SHORT,
}

const S_IN_MNTH: u64 = 2628003; // 2628002,88 seconds according to Google

/// Do the formatting. See `Style`'s docstring for formatting options.
/// If you need just simple mode without bloated featureful implementation,
/// use version 0.0.2 of this crate
///
/// ```
/// extern crate timeago;
/// assert_eq!(timeago::format(std::time::Duration::new(3600, 0), timeago::Style::LONG), "1 hour ago");
/// ```
#[deprecated(since="0.1.0",note="Use Formatter or format_5chars")]
#[allow(deprecated)]
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
        Style::HUMAN => {
            match s {
                0 => "just now".into(),
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
            format_5chars(d)
        }
    }
}

/*
#[cfg(test)]
mod tests {
    use std::time::Duration;
    #[allow(deprecated)]
    use super::{Style, format};

    fn dns(secs: u64) -> Duration {
        Duration::from_secs(secs)
    }
    fn dn(secs: u64, nanos: u32) -> Duration {
        Duration::new(secs, nanos)
    }
    #[allow(deprecated)]
    fn fmtl(d: Duration) -> String {
        format(d, Style::LONG)
    }
    #[allow(deprecated)]
    fn fmth(d: Duration) -> String {
        format(d, Style::HUMAN)
    }
    #[allow(deprecated)]
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
    fn test_human() {
        assert_eq!(fmth(dns(0)), "just now");
        assert_eq!(fmth(dn(0, 500_000_000)), "just now");
        assert_eq!(fmth(dns(1)), "1 second ago");
        assert_eq!(fmth(dn(1, 500_000_000)), "1 second ago");
        assert_eq!(fmth(dns(59)), "59 seconds ago");
        assert_eq!(fmth(dns(60)), "1 minute ago");
        assert_eq!(fmth(dns(65)), "1 minute ago");
        assert_eq!(fmth(dns(119)), "1 minute ago");
        assert_eq!(fmth(dns(120)), "2 minutes ago");
        assert_eq!(fmth(dns(3599)), "59 minutes ago");
        assert_eq!(fmth(dns(3600)), "1 hour ago");
        assert_eq!(fmth(dns(1000_000)), "11 days ago");
        assert_eq!(fmth(dns(1000_000_000)), "~31 years ago");
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
*/
