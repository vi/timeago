use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Romanian;
impl Language for Romanian {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "acum" }
    fn too_high(&self) -> &'static str { "demult" }
    fn ago(&self)      -> &'static str { "acum" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "nanosecundă",
                Microseconds  =>  "microsecundă",
                Milliseconds  =>  "milisecundă",
                Seconds       =>  "secundă",
                Minutes       =>  "minut",
                Hours         =>  "oră",
                Days          =>  "zi",
                Weeks         =>  "săptămână",
                Months        =>  "lună",
                Years         =>  "an",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanosecunde",
                Microseconds  =>  "microsecunde",
                Milliseconds  =>  "milisecunde",
                Seconds       =>  "secunde",
                Minutes       =>  "minute",
                Hours         =>  "ore",
                Days          =>  "zile",
                Weeks         =>  "săptămâni",
                Months        =>  "luni",
                Years         =>  "ani",
            }
        }
    }
    fn place_ago_before(&self) -> bool { true }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Romanian);
    assert_eq!(f.convert(Duration::from_secs(60)), "acum 1 minut");
}
