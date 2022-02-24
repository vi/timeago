#![cfg_attr(rustfmt, rustfmt_skip)]
use super::super::{Language, TimeUnit};

/// Default language for timeago
#[derive(Default)]
pub struct Italian;
impl Language for Italian {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "adesso" }
    fn too_high(&self) -> &'static str { "troppo vecchio" }
    fn ago(&self)      -> &'static str { "fa" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "nanosecondo",
                Microseconds  =>  "microsecondo",
                Milliseconds  =>  "millisecondo",
                Seconds       =>  "secondo",
                Minutes       =>  "minuto",
                Hours         =>  "ora",
                Days          =>  "giorno",
                Weeks         =>  "settimana",
                Months        =>  "mese",
                Years         =>  "anno",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanosecondi",
                Microseconds  =>  "microsecondi",
                Milliseconds  =>  "millisecondi",
                Seconds       =>  "secondi",
                Minutes       =>  "minuti",
                Hours         =>  "ore",
                Days          =>  "giorni",
                Weeks         =>  "settimane",
                Months        =>  "mesi",
                Years         =>  "anni",
            }
        }
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Italian);
    assert_eq!(f.convert(Duration::from_secs(60)), "1 minuto fa");
}
