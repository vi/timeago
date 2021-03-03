#![cfg_attr(rustfmt, rustfmt_skip)]
use super::super::{Language, TimeUnit};

/// Default language for timeago
#[derive(Default)]
pub struct English;
impl Language for English {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
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
