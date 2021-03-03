use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct French;
impl Language for French {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "maintenant" }
    fn too_high(&self) -> &'static str { "ancien" }
    fn ago(&self)      -> &'static str { "il y a" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "nanoseconde",
                Microseconds  =>  "microseconde",
                Milliseconds  =>  "milliseconde",
                Seconds       =>  "seconde",
                Minutes       =>  "minute",
                Hours         =>  "heure",
                Days          =>  "jour",
                Weeks         =>  "semaine",
                Months        =>  "mois",
                Years         =>  "année",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanosecondes",
                Microseconds  =>  "microsecondes",
                Milliseconds  =>  "milisecondes",
                Seconds       =>  "secondes",
                Minutes       =>  "minutes",
                Hours         =>  "heures",
                Days          =>  "jours",
                Weeks         =>  "semaines",
                Months        =>  "mois",
                Years         =>  "ans",
            }
        }
    }
    fn place_ago_before(&self) -> bool { true }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(French);
    assert_eq!(f.convert(Duration::from_secs(60)), "il y a 1 minute");
}
