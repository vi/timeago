use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Danish;
impl Language for Danish {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "nu" }
    fn too_high(&self) -> &'static str { "gammel" }
    fn ago(&self)      -> &'static str { "siden" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "nanosekund",
                Microseconds  =>  "mikrosekund",
                Milliseconds  =>  "millisekund",
                Seconds       =>  "sekund",
                Minutes       =>  "minut",
                Hours         =>  "time",
                Days          =>  "dag",
                Weeks         =>  "uge",
                Months        =>  "måned",
                Years         =>  "år",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanosekunder",
                Microseconds  =>  "mikrosekunder",
                Milliseconds  =>  "millisekunder",
                Seconds       =>  "sekunder",
                Minutes       =>  "minutter",
                Hours         =>  "timer",
                Days          =>  "dage",
                Weeks         =>  "uger",
                Months        =>  "måneder",
                Years         =>  "år",
            }
        }
    }
}
