use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Ukrainian;
impl Ukrainian {
    fn accusative(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "наносекунду",
            Microseconds  =>  "мікросекунду",
            Milliseconds  =>  "мілісекунду",
            Seconds       =>  "секунду",
            Minutes       =>  "хвилину",
            Hours         =>  "годину",
            Days          =>  "день",
            Weeks         =>  "тиждень",
            Months        =>  "місяць",
            Years         =>  "рік",
        }
    }
    fn genitive(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "наносекунди",
            Microseconds  =>  "мікросекунди",
            Milliseconds  =>  "мілісекунди",
            Seconds       =>  "секунди",
            Minutes       =>  "хвилини",
            Hours         =>  "години",
            Days          =>  "дня",
            Weeks         =>  "тижня",
            Months        =>  "місяця",
            Years         =>  "роки",
        }
    }
    fn genitive_plural(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "наносекунд",
            Microseconds  =>  "мікросекунд",
            Milliseconds  =>  "мілісекунд",
            Seconds       =>  "секунд",
            Minutes       =>  "хвилин",
            Hours         =>  "годин",
            Days          =>  "днів",
            Weeks         =>  "тижнів",
            Months        =>  "місяців",
            Years         =>  "років",
        }
    }
}
impl Language for Ukrainian {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "зараз" }
    fn too_high(&self) -> &'static str { "давно" }
    fn ago(&self)      -> &'static str { "тому" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
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
    let f = Formatter::with_language(Ukrainian);
    assert_eq!(f.convert(Duration::from_secs(60)), "1 хвилину тому");
    assert_eq!(f.convert(Duration::from_secs(2)), "2 секунди тому");
    assert_eq!(f.convert(Duration::from_secs(5)), "5 секунд тому");
    assert_eq!(f.convert(Duration::from_secs(12)), "12 секунд тому");
    assert_eq!(f.convert(Duration::from_secs(1*3600*12*366)), "6 місяців тому");
    assert_eq!(f.convert(Duration::from_secs(1*3600*24*366)), "1 рік тому");
    assert_eq!(f.convert(Duration::from_secs(2*3600*24*366)), "2 роки тому");
    assert_eq!(f.convert(Duration::from_secs(4*3600*24*366)), "4 роки тому");
    assert_eq!(f.convert(Duration::from_secs(5*3600*24*366)), "5 років тому");
    assert_eq!(f.convert(Duration::from_secs(10*3600*24*366)), "10 років тому");
    assert_eq!(f.convert(Duration::from_secs(11*3600*24*366)), "11 років тому");
    assert_eq!(f.convert(Duration::from_secs(14*3600*24*366)), "14 років тому");
    assert_eq!(f.convert(Duration::from_secs(15*3600*24*366)), "15 років тому");
    assert_eq!(f.convert(Duration::from_secs(19*3600*24*366)), "19 років тому");
    assert_eq!(f.convert(Duration::from_secs(20*3600*24*366)), "20 років тому");
    assert_eq!(f.convert(Duration::from_secs(21*3600*24*366)), "21 рік тому");
    assert_eq!(f.convert(Duration::from_secs(32*3600*24*366)), "32 роки тому");
    assert_eq!(f.convert(Duration::from_secs(99*3600*24*366)), "99 років тому");
    assert_eq!(f.convert(Duration::from_secs(100*3600*24*366)), "100 років тому");
    assert_eq!(f.convert(Duration::from_secs(101*3600*24*366)), "101 рік тому");
    assert_eq!(f.convert(Duration::from_secs(111*3600*24*366)), "111 років тому");
}
