#![cfg_attr(rustfmt, rustfmt_skip)]
use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Portuguese;
impl Language for Portuguese {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "agora" }
    fn too_high(&self) -> &'static str { "antigo" }
    fn ago(&self)      -> &'static str { "atrás" }
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
                Days          =>  "dia",
                Weeks         =>  "semana",
                Months        =>  "mês",
                Years         =>  "ano",
            }
        } else {
            match tu {
                Nanoseconds   =>  "nanosegundos",
                Microseconds  =>  "microsegundos",
                Milliseconds  =>  "milisegundos",
                Seconds       =>  "segundos",
                Minutes       =>  "minutos",
                Hours         =>  "horas",
                Days          =>  "dias",
                Weeks         =>  "semanas",
                Months        =>  "meses",
                Years         =>  "anos",
            }
        }
    }
}
