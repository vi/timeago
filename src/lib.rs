#![deny(missing_docs)]
//! Given a Duration, lossily format it like in 'N days ago'.
//!
//! Parsing it back to Duration is not supported yet (See [`chrono-english`] crate).
//!
//! Multiple languages are supported though `Language` trait.
//! Enable `isolang` feature to gain support of getting Language impl from
//! `lsolang::Language`.
//!
//! You can configure minimum and maximum time units, as well as "precision" of
//! how many items to emit.
//!
//! Fractional results like "1.5 days ago" are not supported.
//!
//! There is a special simplified version to get compact 5-character representation: `format_5chars`.
//!
//! The main item of timeago is [`Formatter`].
//!
//! [`chrono-english`]:https://docs.rs/chrono-english
//! [`Formatter`]:struct.Formatter.html

use std::time::Duration;

#[cfg(feature = "chrono")]
extern crate chrono;

/// Interface for connecting natural languages to use for the formatting
/// See "language" module documentation for details.
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

    /// For German and such
    fn place_ago_before(&self) -> bool {
        false
    }

    /// Make a dynamic copy of this language
    fn clone_boxed(&self) -> BoxedLanguage;
}

#[cfg_attr(rustfmt, rustfmt_skip)]
impl Language for BoxedLanguage {
    fn clone_boxed(&self) -> BoxedLanguage { (**self).clone_boxed() }
    fn too_low(&self) -> &'static str   { (**self).too_low() }
    fn too_high(&self) -> &'static str  { (**self).too_high() }
    fn ago(&self) -> &'static str       { (**self).ago() }
    fn place_ago_before(&self) -> bool  { (**self).place_ago_before() }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str  { 
        (**self).get_word(tu, x)
    }
}

/// Dynamic version of the `Language` trait
pub type BoxedLanguage = Box<Language + Send + Sync + 'static>;

/// A collection of natural languages supported out-of-the-box for the formatting.
///
/// You can implement a language yourself by deriving
/// the `Language` trait (pull requests are welcome).
///
/// The list of languages is also tracked in `README.md`.
/// If you spot an error, submit a fix or point it out on [Github issues](https://github.com/vi/timeago/issues/new). If on the other hand you have checked a language and assert that it is done properly, [submit a pull request against `README.md` of this project][er].
///
/// You can also choose the language at runtime using the `isolang` cargo feature and [`from_isolang`] function.
///
/// Requires `translations` Cargo feature.
///
/// [`from_isolang`]:fn.from_isolang.html
/// [er]:https://github.com/vi/timeago/edit/master/README.md
#[cfg(feature = "translations")]
pub mod languages;

#[cfg(all(feature = "isolang", feature = "translations"))]
pub use languages::from_isolang;

#[cfg(not(feature = "translations"))]
/// Non-english modes are currently disabled by omission of "translations" cargo feature.
pub mod languages {
    /// Non-english modes are currently disabled by omission of "translations" cargo feature.
    pub mod english;
}

pub use languages::english::English;

/// Various units of time to specify as maximum or minimum.
/// Note that calculations are approximate, not calendar-based.
#[allow(missing_docs)]
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
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
    pub fn min_duration(&self) -> Duration {
        use TimeUnit::*;
        match *self {
            Nanoseconds => Duration::new(0, 1),
            Microseconds => Duration::new(0, 1000),
            Milliseconds => Duration::new(0, 1_000_000),
            Seconds => Duration::new(1, 0),
            Minutes => Duration::new(60, 0),
            Hours => Duration::new(60 * 60, 0),
            Days => Duration::new(24 * 60 * 60, 0),
            Weeks => Duration::new(7 * 24 * 60 * 60, 0),
            Months => Duration::new(S_IN_MNTH, 0),
            Years => Duration::new(S_IN_MNTH * 12, 0),
        }
    }

    /// "Upgrade" minutes to hours, hours to days and so on.
    #[cfg_attr(rustfmt, rustfmt_skip)]
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

    /// "Downgrade" weeks to days, seconds to milliseconds and so on.
    #[cfg_attr(rustfmt, rustfmt_skip)]
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

/// Main formatter struct. Build it with new() and maybe modify some options, then use convert.
/// ```
/// let f = timeago::Formatter::new();
/// let d = std::time::Duration::from_secs(3600);
/// assert_eq!(f.convert(d), "1 hour ago");
/// ```
pub struct Formatter<L: Language = English> {
    lang: L,
    num_items: usize,
    min_unit: TimeUnit,
    max_unit: TimeUnit,
    too_low: Option<&'static str>,
    too_high: Option<&'static str>,
    ago: Option<&'static str>,
    max_duration: Duration,
}

impl Default for Formatter {
    fn default() -> Self {
        Self::new()
    }
}

impl Formatter {
    /// Constructor for some default formatting in English
    ///
    /// It emits one chunk, limits to seconds and has no maximum duration.
    pub fn new() -> Formatter {
        Formatter::with_language(English)
    }
}

impl Clone for Formatter<BoxedLanguage> {
    fn clone(&self) -> Formatter<BoxedLanguage> {
        Formatter {
            lang: self.lang.clone_boxed(),
            num_items: self.num_items,
            min_unit: self.min_unit,
            max_unit: self.max_unit,
            too_low: self.too_low,
            too_high: self.too_high,
            ago: self.ago,
            max_duration: self.max_duration,
        }
    }
}

impl<L: Language> Formatter<L> {
    /// Constructor for some default formatting with specified language instance
    ///
    /// It emits one item (chunk), limits to seconds and has no maximum duration.
    pub fn with_language(l: L) -> Self {
        Formatter {
            lang: l,
            num_items: 1,
            min_unit: TimeUnit::Seconds,
            max_unit: TimeUnit::Years,
            too_low: None,
            too_high: None,
            ago: None,
            max_duration: Duration::new(std::u64::MAX, 999_999_999),
        }
    }

    /// Set number of time unit items to emit (for example, 1 item is for "1 year"; 3 items is for "1 year 3 months 17 days"). Zero chunks like "0 minutes" are not emitted, expect of at the end if `too_low` is `"0"`.
    /// Default is 1.
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.num_items(1);
    /// let d = std::time::Duration::from_secs(3600+60+3);
    /// assert_eq!(f.convert(d), "1 hour ago");
    /// f.num_items(2);
    /// assert_eq!(f.convert(d), "1 hour 1 minute ago");
    /// f.num_items(3);
    /// assert_eq!(f.convert(d), "1 hour 1 minute 3 seconds ago");
    /// f.num_items(4);
    /// assert_eq!(f.convert(d), "1 hour 1 minute 3 seconds ago");
    /// ```
    pub fn num_items(&mut self, x: usize) -> &mut Self {
        assert!(x > 0);
        self.num_items = x;
        self
    }

    /// Set maximum used unit. Not to be confused with `max_duration`.
    /// Should not affect appearance of "old" or other `too_high` values.
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.max_unit(timeago::TimeUnit::Hours);
    /// let d = std::time::Duration::from_secs(60);
    /// assert_eq!(f.convert(d), "1 minute ago");
    /// let d = std::time::Duration::from_secs(3600);
    /// assert_eq!(f.convert(d), "1 hour ago");
    /// let d = std::time::Duration::from_secs(24*3600);
    /// assert_eq!(f.convert(d), "24 hours ago");
    /// let d = std::time::Duration::from_secs(30*24*3600);
    /// assert_eq!(f.convert(d), "720 hours ago");
    /// ```
    pub fn max_unit(&mut self, x: TimeUnit) -> &mut Self {
        self.max_unit = x;
        self
    }

    /// Set minimum used unit. Durations below minimally representable by that unit emit `too_low` value like "now", or like "0 days" instead of normal output.
    /// When `num_items` > 1, it also acts as precision limiter.
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.min_unit(timeago::TimeUnit::Minutes);
    /// let d = std::time::Duration::from_secs(30);
    /// assert_eq!(f.convert(d), "now");
    /// let d = std::time::Duration::from_secs(90);
    /// assert_eq!(f.convert(d), "1 minute ago");
    /// ```
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.num_items(99);
    /// let d = std::time::Duration::new(1*3600*24 + 2*3600 + 3*60 + 4, 500_000_000);
    /// assert_eq!(f.convert(d), "1 day 2 hours 3 minutes 4 seconds ago");
    /// f.min_unit(timeago::TimeUnit::Hours);
    /// assert_eq!(f.convert(d), "1 day 2 hours ago");
    /// f.min_unit(timeago::TimeUnit::Microseconds);
    /// assert_eq!(f.convert(d), "1 day 2 hours 3 minutes 4 seconds 500 milliseconds ago");
    /// f.min_unit(timeago::TimeUnit::Months);
    /// assert_eq!(f.convert(d), "now");
    /// ```
    pub fn min_unit(&mut self, x: TimeUnit) -> &mut Self {
        self.min_unit = x;
        self
    }

    /// Override what is used instead of "now" for too short durations (not representable with the time unit configures as `min_unit`).
    /// Setting this to special value `"0"` causes emitting output like "0 days", depending on `min_unit` property.
    /// Note that `Language`'s `too_low` is not used in this case, except of for `"0"`.
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.min_unit(timeago::TimeUnit::Months)
    ///  .too_low("this month");
    /// let d = std::time::Duration::from_secs(24*3600);
    /// assert_eq!(f.convert(d), "this month");
    /// ```
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.min_unit(timeago::TimeUnit::Minutes);
    /// let d = std::time::Duration::from_secs(30);
    /// assert_eq!(f.convert(d), "now");
    /// f.too_low("-");
    /// assert_eq!(f.convert(d), "-");
    /// f.too_low("");
    /// assert_eq!(f.convert(d), "");
    /// f.too_low("0");
    /// assert_eq!(f.convert(d), "0 minutes ago");
    /// ```
    pub fn too_low(&mut self, x: &'static str) -> &mut Self {
        self.too_low = Some(x);
        self
    }

    /// Override what is used instead of "old" for too high units.
    /// Note that `Language`'s `too_high` is not used in this case.
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.max_duration(std::time::Duration::from_secs(3600*24*30));
    /// f.too_high("ancient");
    /// let d = std::time::Duration::from_secs(1000_000_000_000);
    /// assert_eq!(f.convert(d), "ancient");
    /// ```
    pub fn too_high(&mut self, x: &'static str) -> &mut Self {
        self.too_high = Some(x);
        self
    }

    /// Maximum duration before it start giving "old" (or other `too_high` value)
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// f.max_duration(std::time::Duration::new(3600*24*30, 0));
    /// let d = std::time::Duration::from_secs(1000_000_000);
    /// assert_eq!(f.convert(d), "old");
    /// ```
    pub fn max_duration(&mut self, x: Duration) -> &mut Self {
        self.max_duration = x;
        self
    }

    /// Override what is used instead of "ago".
    /// Empty string literal `""` is a bit special in the space handling.
    /// ```
    /// let mut f = timeago::Formatter::new();
    /// let d = std::time::Duration::from_secs(60);
    /// assert_eq!(f.convert(d), "1 minute ago");
    /// f.ago("later");
    /// assert_eq!(f.convert(d), "1 minute later");
    /// f.ago("");
    /// assert_eq!(f.convert(d), "1 minute");
    /// ```
    pub fn ago(&mut self, x: &'static str) -> &mut Self {
        self.ago = Some(x);
        self
    }

    /// Format the timespan between `from` and `to` as a string like "15 days ago".
    ///
    /// Requires `chrono` Cargo feature.
    ///
    /// `from` should come before `to`, otherwise `"???"` will be returned.
    ///
    /// Currently it doesn't actually take the calendar into account and just converts datetimes
    /// into a plain old `std::time::Duration`, but in future here may be a proper implementation.
    ///
    /// ```
    /// extern crate chrono;
    /// extern crate timeago;
    /// let mut f = timeago::Formatter::new();
    /// f.num_items(2);
    /// let from = chrono::DateTime::parse_from_rfc3339("2013-12-19T15:00:00+03:00").unwrap();
    /// let to   = chrono::DateTime::parse_from_rfc3339("2013-12-23T17:00:00+03:00").unwrap();
    /// assert_eq!(f.convert_chrono(from, to), "4 days 2 hours ago");
    /// ```
    #[cfg(feature = "chrono")]
    pub fn convert_chrono<Tz1, Tz2>(
        &self,
        from: chrono::DateTime<Tz1>,
        to: chrono::DateTime<Tz2>,
    ) -> String
    where
        Tz1: chrono::TimeZone,
        Tz2: chrono::TimeZone,
    {
        let q = to.signed_duration_since(from);
        if let Ok(dur) = q.to_std() {
            self.convert(dur)
        } else {
            "???".to_owned()
        }
    }

    /// Convert specified [`Duration`] to a String representing
    /// approximation of specified timespan as a string like
    /// "5 days ago", with specified by other methods settings.
    /// See module-level doc for more info.
    /// ```
    /// let f = timeago::Formatter::new();
    /// let d = std::time::Duration::from_secs(3600*24);
    /// assert_eq!(f.convert(d), "1 day ago");
    /// ```
    ///
    /// [`Duration`]:https://doc.rust-lang.org/std/time/struct.Duration.html
    pub fn convert(&self, d: Duration) -> String {
        if d > self.max_duration {
            return self.too_high
                .unwrap_or_else(|| self.lang.too_high())
                .to_owned();
        }

        let mut ret = self.convert_impl(d, self.num_items);

        if ret == "" {
            let now = self.too_low.unwrap_or_else(|| self.lang.too_low());
            if now != "0" {
                return now.to_owned();
            } else {
                ret = format!("0 {}", self.lang.get_word(self.min_unit, 0));
            }
        }

        let ago = self.ago.unwrap_or_else(|| self.lang.ago());
        if ago == "" {
            ret
        } else if !self.lang.place_ago_before() {
            format!("{} {}", ret, ago)
        } else {
            format!("{} {}", ago, ret)
        }
    }

    fn convert_impl(&self, d: Duration, items_left: usize) -> String {
        if items_left == 0 {
            return "".to_owned();
        }

        let mut dtu = dominant_time_unit(d);

        while dtu > self.max_unit {
            dtu = dtu.smaller_unit().unwrap();
        }

        while dtu < self.min_unit {
            dtu = dtu.bigger_unit().unwrap();
        }

        let (x, rem) = split_up(d, dtu);

        if x == 0 {
            return "".to_owned();
        }

        let recurse_result = self.convert_impl(rem, items_left - 1);
        if recurse_result == "" {
            format!("{} {}", x, self.lang.get_word(dtu, x))
        } else {
            format!("{} {} {}", x, self.lang.get_word(dtu, x), recurse_result)
        }
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
fn dominant_time_unit(d: Duration) -> TimeUnit {
    use TimeUnit::*;

    match d {
        x if x < Microseconds.min_duration() => Nanoseconds ,
        x if x < Milliseconds.min_duration() => Microseconds,
        x if x < Seconds     .min_duration() => Milliseconds,
        x if x < Minutes     .min_duration() => Seconds     ,
        x if x < Hours       .min_duration() => Minutes     ,
        x if x < Days        .min_duration() => Hours       ,
        x if x < Weeks       .min_duration() => Days        ,
        x if x < Months      .min_duration() => Weeks       ,
        x if x < Years       .min_duration() => Months      ,
        _ => Years,
    }
}

fn divmod64(a: u64, b: u64) -> (u64, u64) {
    (a / b, a % b)
}
fn divmod32(a: u32, b: u32) -> (u32, u32) {
    (a / b, a % b)
}

fn split_up(d: Duration, tu: TimeUnit) -> (u64, Duration) {
    let s = d.as_secs();
    let n = d.subsec_nanos();

    let tud = tu.min_duration();
    let tus = tud.as_secs();
    let tun = tud.subsec_nanos();

    if tus != 0 {
        assert!(tun == 0);
        if s == 0 {
            (0, d)
        } else {
            let (c, s2) = divmod64(s, tus);
            (c, Duration::new(s2, n))
        }
    } else {
        // subsecond timeunit
        assert!(tus == 0);
        if s == 0 {
            let (c, n2) = divmod32(n, tun);
            (c.into(), Duration::new(0, n2))
        } else {
            assert!(1_000_000_000 % tun == 0);
            let tuninv = 1_000_000_000 / (u64::from(tun));
            let pieces = s.saturating_mul(tuninv).saturating_add(u64::from(n / tun));

            let subtract_s = pieces / tuninv;
            let subtract_ns = ((pieces % tuninv) as u32) * tun;

            let (mut s, mut n) = (s, n);

            if subtract_ns > n {
                s -= 1;
                n += 1_000_000_000;
            }

            let remain_s = s - subtract_s;
            let remain_ns = n - subtract_ns;
            (pieces, Duration::new(remain_s, remain_ns))
        }
    }
}

#[cfg(test)]
mod tests_split_up {
    use super::*;

    fn ds(secs: u64) -> Duration {
        Duration::from_secs(secs)
    }
    fn dn(secs: u64, nanos: u32) -> Duration {
        Duration::new(secs, nanos)
    }

    #[test]
    fn dominant_time_unit_test() {
        use TimeUnit::*;

        assert_eq!(dominant_time_unit(ds(3)), Seconds);
        assert_eq!(dominant_time_unit(ds(60)), Minutes);
        assert_eq!(dominant_time_unit(dn(0, 250_000_000)), Milliseconds);
    }

    #[test]
    fn split_up_test_sane() {
        use TimeUnit::*;

        assert_eq!(split_up(ds(120), Minutes), (2, ds(0)));
        assert_eq!(split_up(ds(119), Minutes), (1, ds(59)));
        assert_eq!(split_up(ds(60), Minutes), (1, ds(0)));
        assert_eq!(split_up(ds(1), Minutes), (0, ds(1)));
        assert_eq!(split_up(ds(0), Minutes), (0, ds(0)));
        assert_eq!(split_up(ds(3600), Minutes), (60, ds(0)));
        assert_eq!(split_up(ds(3600), Hours), (1, ds(0)));
        assert_eq!(split_up(ds(3600), Seconds), (3600, ds(0)));
        assert_eq!(split_up(ds(3600), Milliseconds), (3600_000, ds(0)));
        assert_eq!(split_up(ds(100000000), Years), (3, ds(5391892)));
        assert_eq!(split_up(ds(100000000), Months), (38, ds(135886)));
        assert_eq!(split_up(ds(100000000), Days), (1157, ds(35200)));
        assert_eq!(split_up(ds(3600), Microseconds), (3600_000_000, ds(0)));
    }
    #[test]
    fn split_up_test_tricky() {
        use TimeUnit::*;

        assert_eq!(split_up(ds(3600), Nanoseconds), (3600_000_000_000, ds(0)));
        assert_eq!(
            split_up(ds(3600_000), Nanoseconds),
            (3600_000_000_000_000, ds(0))
        );
        assert_eq!(
            split_up(ds(3600_000_000), Nanoseconds),
            (3600_000_000_000_000_000, ds(0))
        );
        assert_eq!(
            split_up(ds(3600_000_000_000), Nanoseconds),
            (std::u64::MAX, dn(3581_553_255_926, 290448385))
        );
        assert_eq!(
            split_up(ds(3600_000_000_000), Microseconds),
            (3600_000_000_000_000_000, ds(0))
        );
        assert_eq!(
            split_up(ds(3600_000_000_000_000), Microseconds),
            (std::u64::MAX, dn(3581_553_255_926_290, 448385000))
        );
        assert_eq!(
            split_up(ds(3600_000_000_000_000), Milliseconds),
            (3600_000_000_000_000_000, ds(0))
        );
        assert_eq!(
            split_up(ds(3600_000_000_000_000_000), Milliseconds),
            (std::u64::MAX, dn(3581_553_255_926_290_448, 385000000))
        );
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
#[deprecated(since = "0.1.0", note = "Use Formatter or format_5chars")]
#[derive(Copy, Clone)]
pub enum Style {
    /// Long format, like "2 years ago"
    LONG,
    /// Human format, like LONG but makes less than 1 second as `just now`
    HUMAN,
    /// Short format, like "02Yea". Should be exactly 5 characters.
    SHORT,
}

const S_IN_MNTH: u64 = 2_628_003; // 2628002,88 seconds according to Google

/// Do the formatting. See `Style`'s docstring for formatting options.
/// If you need just simple mode without bloated featureful implementation,
/// use version 0.0.2 of this crate
///
/// ```
/// extern crate timeago;
/// assert_eq!(timeago::format(std::time::Duration::new(3600, 0), timeago::Style::LONG), "1 hour ago");
/// ```
#[deprecated(since = "0.1.0", note = "Use Formatter or format_5chars")]
#[allow(deprecated)]
pub fn format(d: Duration, style: Style) -> String {
    match style {
        Style::LONG => Formatter::new().min_unit(TimeUnit::Nanoseconds).convert(d),
        Style::HUMAN => {
            let ret = Formatter::new().convert(d);
            if ret == "now" {
                "just now".to_owned()
            } else {
                ret
            }
        }
        Style::SHORT => format_5chars(d),
    }
}

#[cfg(test)]
mod tests {
    #[allow(deprecated)]
    use super::{format, Style};
    use std::time::Duration;

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
        assert_eq!(fmtl(dns(1000_000)), "1 week ago");
        assert_eq!(fmtl(dns(1000_000_000)), "31 years ago");
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
        assert_eq!(fmth(dns(1000_000)), "1 week ago");
        assert_eq!(fmth(dns(1000_000_000)), "31 years ago");
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
