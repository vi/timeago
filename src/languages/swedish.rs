use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Swedish;
impl Language for Swedish {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "nu" }
    fn too_high(&self) -> &'static str { "gammal" }
    fn ago(&self)      -> &'static str { "sedan" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "nanosekund",
                Microseconds  =>  "mikrosekund",
                Milliseconds  =>  "millisekund",
                Seconds       =>  "sekund",
                Minutes       =>  "minut",
                Hours         =>  "timme",
                Days          =>  "dag",
                Weeks         =>  "vecka",
                Months        =>  "månad",
                Years         =>  "år",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanosekunder",
                Microseconds  =>  "mikrosekunder",
                Milliseconds  =>  "millisekunder",
                Seconds       =>  "sekunder",
                Minutes       =>  "minuter",
                Hours         =>  "timmar",
                Days          =>  "dagar",
                Weeks         =>  "veckor",
                Months        =>  "månader",
                Years         =>  "år",
            }
        }
    }
}
