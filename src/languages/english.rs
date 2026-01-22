use super::super::{Language, TimeUnit};

/// Default language for timeago
#[derive(Default)]
pub struct English;
impl Language for English {
    fn clone_boxed(&self) -> super::super::BoxedLanguage {
        Box::new(Self {})
    }
    fn too_low(&self) -> &'static str {
        "now"
    }
    fn too_high(&self) -> &'static str {
        "old"
    }
    fn ago(&self) -> &'static str {
        "ago"
    }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds => "nanosecond",
                Microseconds => "microsecond",
                Milliseconds => "millisecond",
                Seconds => "second",
                Minutes => "minute",
                Hours => "hour",
                Days => "day",
                Weeks => "week",
                Months => "month",
                Years => "year",
            }
        } else {
            match tu {
                Nanoseconds => "nanoseconds",
                Microseconds => "microseconds",
                Milliseconds => "milliseconds",
                Seconds => "seconds",
                Minutes => "minutes",
                Hours => "hours",
                Days => "days",
                Weeks => "weeks",
                Months => "months",
                Years => "years",
            }
        }
    }
}

#[derive(Default)]
pub struct EnglishAbbreviated;

impl Language for EnglishAbbreviated {
    fn too_low(&self) -> &'static str {
        "now"
    }

    fn too_high(&self) -> &'static str {
        "old"
    }

    fn ago(&self) -> &'static str {
        ""
    }

    fn get_word(&self, tu: TimeUnit, _: u64) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds => "ns",
            Microseconds => "μs",
            Milliseconds => "ms",
            Seconds => "s",
            Minutes => "m",
            Hours => "h",
            Days => "d",
            Weeks => "wk",
            Months => "mo",
            Years => "yr",
        }
    }

    fn clone_boxed(&self) -> crate::BoxedLanguage {
        todo!()
    }

    fn place_ago_before(&self) -> bool {
        false
    }

    fn override_space_near_ago(&self) -> &str {
        ""
    }

    fn place_unit_before(&self, _: u64) -> bool {
        false
    }

    fn between_chunks(&self) -> &str {
        " "
    }

    fn between_value_and_word(&self) -> &str {
        ""
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abbreviated() {
        fn fmt(seconds: u64) -> String {
            crate::Formatter::with_language(EnglishAbbreviated)
                .convert(std::time::Duration::from_secs(seconds))
        }

        assert_eq!(fmt(0), "now");
        assert_eq!(fmt(1), "1s");
        assert_eq!(fmt(59), "59s");
        assert_eq!(fmt(60), "1m");
        assert_eq!(fmt(65), "1m");
        assert_eq!(fmt(119), "1m");
        assert_eq!(fmt(120), "2m");
        assert_eq!(fmt(3599), "59m");
        assert_eq!(fmt(3600), "1h");
        assert_eq!(fmt(1000_000), "1wk");
        assert_eq!(fmt(1000_000_000), "31yr");
    }
}
