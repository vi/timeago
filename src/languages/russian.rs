use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Russian;
impl Russian {
    fn accusative(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds => "наносекунду",
            Microseconds => "микросекунду",
            Milliseconds => "миллисекунду",
            Seconds => "секунду",
            Minutes => "минуту",
            Hours => "час",
            Days => "день",
            Weeks => "неделю",
            Months => "месяц",
            Years => "год",
        }
    }
    fn genitive(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds => "наносекунды",
            Microseconds => "микросекунды",
            Milliseconds => "миллисекунды",
            Seconds => "секунды",
            Minutes => "минуты",
            Hours => "часа",
            Days => "дня",
            Weeks => "недели",
            Months => "месяца",
            Years => "года",
        }
    }
    fn genitive_plural(&self, tu: TimeUnit) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds => "наносекунд",
            Microseconds => "микросекунд",
            Milliseconds => "миллисекунд",
            Seconds => "секунд",
            Minutes => "минут",
            Hours => "часов",
            Days => "дней",
            Weeks => "недель",
            Months => "месяцев",
            Years => "лет",
        }
    }
}
impl Language for Russian {
    fn clone_boxed(&self) -> super::super::BoxedLanguage {
        Box::new(Self {})
    }
    fn too_low(&self) -> &'static str {
        "сейчас"
    }
    fn too_high(&self) -> &'static str {
        "давно"
    }
    fn ago(&self) -> &'static str {
        "назад"
    }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        let last_two = x % 100;
        let last = x % 10;

        if (11..=20).contains(&last_two) {
            self.genitive_plural(tu)
        } else {
            match last {
                1 => self.accusative(tu),
                2..=4 => self.genitive(tu),
                0 | 5..=9 => self.genitive_plural(tu),
                _ => unreachable!(),
            }
        }
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Russian);
    assert_eq!(f.convert(Duration::from_secs(60)), "1 минуту назад");
    assert_eq!(f.convert(Duration::from_secs(2)), "2 секунды назад");
    assert_eq!(f.convert(Duration::from_secs(5)), "5 секунд назад");
    assert_eq!(f.convert(Duration::from_secs(12)), "12 секунд назад");
    assert_eq!(
        f.convert(Duration::from_secs(1 * 3600 * 12 * 366)),
        "6 месяцев назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(1 * 3600 * 24 * 366)),
        "1 год назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(2 * 3600 * 24 * 366)),
        "2 года назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(4 * 3600 * 24 * 366)),
        "4 года назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(5 * 3600 * 24 * 366)),
        "5 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(10 * 3600 * 24 * 366)),
        "10 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(11 * 3600 * 24 * 366)),
        "11 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(14 * 3600 * 24 * 366)),
        "14 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(15 * 3600 * 24 * 366)),
        "15 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(19 * 3600 * 24 * 366)),
        "19 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(20 * 3600 * 24 * 366)),
        "20 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(21 * 3600 * 24 * 366)),
        "21 год назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(32 * 3600 * 24 * 366)),
        "32 года назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(99 * 3600 * 24 * 366)),
        "99 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(100 * 3600 * 24 * 366)),
        "100 лет назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(101 * 3600 * 24 * 366)),
        "101 год назад"
    );
    assert_eq!(
        f.convert(Duration::from_secs(111 * 3600 * 24 * 366)),
        "111 лет назад"
    );
}
