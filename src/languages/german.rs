use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct German;
impl Language for German {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "jetzt" }
    fn too_high(&self) -> &'static str { "zu alt" }
    fn ago(&self)      -> &'static str { "vor" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "Nanosekunde",
                Microseconds  =>  "Mikrosekunde",
                Milliseconds  =>  "Millisekunde",
                Seconds       =>  "Sekunde",
                Minutes       =>  "Minute",
                Hours         =>  "Stunde",
                Days          =>  "Tag",
                Weeks         =>  "Woche",
                Months        =>  "Monat",
                Years         =>  "Jahr",
            }
        } else {
            match tu {
                Nanoseconds   =>  "Nanosekunden",
                Microseconds  =>  "Mikrosekunden",
                Milliseconds  =>  "Millisekunden",
                Seconds       =>  "Sekunden",
                Minutes       =>  "Minuten",
                Hours         =>  "Stunden",
                Days          =>  "Tagen",
                Weeks         =>  "Wochen",
                Months        =>  "Monaten",
                Years         =>  "Jahren",
            }
        }
    }
    fn place_ago_before(&self) -> bool { true }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(German);
    assert_eq!(f.convert(Duration::from_secs(60)), "vor 1 Minute");
}
