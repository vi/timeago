#![cfg_attr(rustfmt, rustfmt_skip)]
use super::super::{Language, TimeUnit};

/// Default language for timeago
#[derive(Default)]
pub struct Basque;
impl Language for Basque {
    fn clone_boxed(&self)      -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self)         -> &'static str { "oraintxe bertan" }
    fn too_high(&self)         -> &'static str { "zaharregi" }
    fn ago(&self)              -> &'static str { "orain dela" }
    fn place_ago_before(&self) -> bool { true }
    fn place_unit_before(&self, x: u64) -> bool {
        x == 1
    }
    fn get_word(&self, tu: TimeUnit, _: u64) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "nanosegundo",
            Microseconds  =>  "mikrosegundo",
            Milliseconds  =>  "milisegundo",
            Seconds       =>  "segundo",
            Minutes       =>  "minutu",
            Hours         =>  "ordu",
            Days          =>  "egun",
            Weeks         =>  "aste",
            Months        =>  "hile",
            Years         =>  "urte",
        }
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Basque);
    assert_eq!(f.convert(Duration::from_secs(60)), "orain dela minutu 1");
    assert_eq!(f.convert(Duration::from_secs(120)), "orain dela 2 minutu");
}
