use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Spanish;
impl Language for Spanish {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "ahora" }
    fn too_high(&self) -> &'static str { "hace mucho" }
    fn ago(&self)      -> &'static str { "hace" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   =>  "nanosegundo",
                Microseconds  =>  "microsegundo",
                Milliseconds  =>  "milisegundo",
                Seconds       =>  "segundo",
                Minutes       =>  "minuto",
                Hours         =>  "hora",
                Days          =>  "día",
                Weeks         =>  "semana",
                Months        =>  "mes",
                Years         =>  "año",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanosegundos",
                Microseconds  =>  "microsegundos",
                Milliseconds  =>  "milisegundos",
                Seconds       =>  "segundos",
                Minutes       =>  "minutos",
                Hours         =>  "horas",
                Days          =>  "días",
                Weeks         =>  "semanas",
                Months        =>  "meses",
                Years         =>  "años",
            }
        }
    }
    fn place_ago_before(&self) -> bool { true }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Spanish);
    assert_eq!(f.convert(Duration::from_secs(60)), "hace 1 minuto");
}
