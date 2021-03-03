use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Belarusian;
impl Belarusian {
    fn accusative(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "нанасэкунду",
            Microseconds  =>  "мікрасэкунду",
            Milliseconds  =>  "мілісэкунду",
            Seconds       =>  "сэкунду",
            Minutes       =>  "хвіліну",
            Hours         =>  "гадзіну",
            Days          =>  "дзень",
            Weeks         =>  "тыдзень",
            Months        =>  "месяц",
            Years         =>  "год",
        }
    }
    fn genitive(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "нанасэкунды",
            Microseconds  =>  "мікрасэкунды",
            Milliseconds  =>  "мілісэкунды",
            Seconds       =>  "сэкунды",
            Minutes       =>  "хвіліны",
            Hours         =>  "гадзіны",
            Days          =>  "дні",
            Weeks         =>  "тыдні",
            Months        =>  "месяца",
            Years         =>  "гады",
        }
    }
    fn genitive_plural(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "нанасэкундаў",
            Microseconds  =>  "мікрасэкундаў",
            Milliseconds  =>  "мілісэкундаў",
            Seconds       =>  "сэкундаў",
            Minutes       =>  "хвілін",
            Hours         =>  "галзін",
            Days          =>  "дней",
            Weeks         =>  "тыдняў",
            Months        =>  "месяцаў",
            Years         =>  "гадоў",
        }
    }
}

impl Language for Belarusian {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "зараз" }
    fn too_high(&self) -> &'static str { "даўно" }
    fn ago(&self)      -> &'static str { "таму" }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        if (x % 100) >= 11 && (x % 100) <= 20 {
            self.genitive_plural(tu)
        } else  if x % 10 == 1 {
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
    let f = Formatter::with_language(Belarusian);
    assert_eq!(f.convert(Duration::from_secs(60)), "1 хвіліну таму");
    assert_eq!(f.convert(Duration::from_secs(2)), "2 сэкунды таму");
    assert_eq!(f.convert(Duration::from_secs(5)), "5 сэкундаў таму");
    assert_eq!(f.convert(Duration::from_secs(12)), "12 сэкундаў таму");
    assert_eq!(f.convert(Duration::from_secs(1*3600*12*366)), "6 месяцаў таму");
    assert_eq!(f.convert(Duration::from_secs(1*3600*24*366)), "1 год таму");
    assert_eq!(f.convert(Duration::from_secs(2*3600*24*366)), "2 гады таму");
    assert_eq!(f.convert(Duration::from_secs(4*3600*24*366)), "4 гады таму");
    assert_eq!(f.convert(Duration::from_secs(5*3600*24*366)), "5 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(10*3600*24*366)), "10 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(11*3600*24*366)), "11 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(14*3600*24*366)), "14 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(15*3600*24*366)), "15 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(19*3600*24*366)), "19 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(20*3600*24*366)), "20 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(21*3600*24*366)), "21 год таму");
    assert_eq!(f.convert(Duration::from_secs(32*3600*24*366)), "32 гады таму");
    assert_eq!(f.convert(Duration::from_secs(99*3600*24*366)), "99 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(100*3600*24*366)), "100 гадоў таму");
    assert_eq!(f.convert(Duration::from_secs(101*3600*24*366)), "101 год таму");
    assert_eq!(f.convert(Duration::from_secs(111*3600*24*366)), "111 гадоў таму");
}

