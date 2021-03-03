use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Polish;
impl Polish {
    fn accusative(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "nanosekundę",
            Microseconds  =>  "mikrosekundę",
            Milliseconds  =>  "milisekundę",
            Seconds       =>  "sekundę",
            Minutes       =>  "minutę",
            Hours         =>  "godzinę",
            Days          =>  "dzień",
            Weeks         =>  "tydzień",
            Months        =>  "miesiąc",
            Years         =>  "lat",
        }
    }
    fn genitive(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "nanosekundy",
            Microseconds  =>  "mikrosekundy",
            Milliseconds  =>  "milisekundy",
            Seconds       =>  "sekundy",
            Minutes       =>  "minuty",
            Hours         =>  "godziny",
            Days          =>  "dni",
            Weeks         =>  "tygodnie",
            Months        =>  "miesiące",
            Years         =>  "lata",
        }
    }
    fn genitive_plural(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "nanosekund",
            Microseconds  =>  "mikrosekund",
            Milliseconds  =>  "milisekund",
            Seconds       =>  "sekund",
            Minutes       =>  "minut",
            Hours         =>  "godzin",
            Days          =>  "dni",
            Weeks         =>  "tygodni",
            Months        =>  "miesięcy",
            Years         =>  "lat",
        }
    }
}
impl Language for Polish {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "teraz" }
    fn too_high(&self) -> &'static str { "dawno" }
    fn ago(&self)      -> &'static str { "temu" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        if tu == TimeUnit::Years && x == 1 {
            return "rok";
        }
        if (x % 100) >= 11 && (x % 100) <= 20 {
            self.genitive_plural(tu)
        } else if x % 10 == 1 {
            self.accusative(tu)
        } else if x % 10 >= 2 && x % 10 <= 4 {
            self.genitive(tu)
        } else if x % 10 >= 5 || x % 10 == 0 {
            self.genitive_plural(tu)
        } else {
            unreachable!()
        }
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Polish);
    assert_eq!(f.convert(Duration::from_secs(60)), "1 minutę temu");
    assert_eq!(f.convert(Duration::from_secs(2)), "2 sekundy temu");
    assert_eq!(f.convert(Duration::from_secs(5)), "5 sekund temu");
    assert_eq!(f.convert(Duration::from_secs(12)), "12 sekund temu");
    assert_eq!(f.convert(Duration::from_secs(1*3600*12*366)), "6 miesięcy temu");
    assert_eq!(f.convert(Duration::from_secs(1*3600*24*366)), "1 rok temu");
    assert_eq!(f.convert(Duration::from_secs(2*3600*24*366)), "2 lata temu");
    assert_eq!(f.convert(Duration::from_secs(4*3600*24*366)), "4 lata temu");
    assert_eq!(f.convert(Duration::from_secs(5*3600*24*366)), "5 lat temu");
    assert_eq!(f.convert(Duration::from_secs(10*3600*24*366)), "10 lat temu");
    assert_eq!(f.convert(Duration::from_secs(11*3600*24*366)), "11 lat temu");
    assert_eq!(f.convert(Duration::from_secs(14*3600*24*366)), "14 lat temu");
    assert_eq!(f.convert(Duration::from_secs(15*3600*24*366)), "15 lat temu");
    assert_eq!(f.convert(Duration::from_secs(19*3600*24*366)), "19 lat temu");
    assert_eq!(f.convert(Duration::from_secs(20*3600*24*366)), "20 lat temu");
    assert_eq!(f.convert(Duration::from_secs(21*3600*24*366)), "21 lat temu");
    assert_eq!(f.convert(Duration::from_secs(32*3600*24*366)), "32 lata temu");
    assert_eq!(f.convert(Duration::from_secs(99*3600*24*366)), "99 lat temu");
    assert_eq!(f.convert(Duration::from_secs(100*3600*24*366)), "100 lat temu");
    assert_eq!(f.convert(Duration::from_secs(101*3600*24*366)), "101 lat temu");
    assert_eq!(f.convert(Duration::from_secs(104*3600*24*366)), "104 lata temu");
    assert_eq!(f.convert(Duration::from_secs(111*3600*24*366)), "111 lat temu");
}
