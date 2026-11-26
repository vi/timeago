use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Romanian;
impl Language for Romanian {
    fn clone_boxed(&self) -> super::super::BoxedLanguage {
        Box::new(Self {})
    }
    fn too_low(&self) -> &'static str {
        "acum"
    }
    fn too_high(&self) -> &'static str {
        "demult"
    }
    fn ago(&self) -> &'static str {
        "acum"
    }
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds => "nanosecundă",
                Microseconds => "microsecundă",
                Milliseconds => "milisecundă",
                Seconds => "secundă",
                Minutes => "minut",
                Hours => "oră",
                Days => "zi",
                Weeks => "săptămână",
                Months => "lună",
                Years => "an",
            }
        } else if x != 0 && (x % 100 == 0 || x % 100 >= 20) {
            // From 20 on (and for 100, 200, ...) Romanian puts "de" between
            // the number and the noun: "20 de minute", but "19 minute", "101 ani".
            match tu {
                Nanoseconds => "de nanosecunde",
                Microseconds => "de microsecunde",
                Milliseconds => "de milisecunde",
                Seconds => "de secunde",
                Minutes => "de minute",
                Hours => "de ore",
                Days => "de zile",
                Weeks => "de săptămâni",
                Months => "de luni",
                Years => "de ani",
            }
        } else {
            match tu {
                Nanoseconds => "nanosecunde",
                Microseconds => "microsecunde",
                Milliseconds => "milisecunde",
                Seconds => "secunde",
                Minutes => "minute",
                Hours => "ore",
                Days => "zile",
                Weeks => "săptămâni",
                Months => "luni",
                Years => "ani",
            }
        }
    }
    fn place_ago_before(&self) -> bool {
        true
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Romanian);
    assert_eq!(f.convert(Duration::from_secs(60)), "acum 1 minut");
}

#[test]
fn test_de_for_20_and_up() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Romanian);
    assert_eq!(f.convert(Duration::from_secs(2)), "acum 2 secunde");
    assert_eq!(f.convert(Duration::from_secs(19)), "acum 19 secunde");
    assert_eq!(f.convert(Duration::from_secs(20)), "acum 20 de secunde");
    assert_eq!(f.convert(Duration::from_secs(45 * 60)), "acum 45 de minute");
    assert_eq!(f.convert(Duration::from_secs(21 * 3600)), "acum 21 de ore");
    let year = 31_536_036;
    assert_eq!(
        f.convert(Duration::from_secs(100 * year)),
        "acum 100 de ani"
    );
    assert_eq!(f.convert(Duration::from_secs(101 * year)), "acum 101 ani");
    assert_eq!(f.convert(Duration::from_secs(119 * year)), "acum 119 ani");
    assert_eq!(
        f.convert(Duration::from_secs(120 * year)),
        "acum 120 de ani"
    );
    let mut f = Formatter::with_language(Romanian);
    f.min_unit(TimeUnit::Minutes).too_low("0");
    assert_eq!(f.convert(Duration::from_secs(5)), "acum 0 minute");
}
