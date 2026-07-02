use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Japanese;
impl Language for Japanese {
    fn clone_boxed(&self) -> super::super::BoxedLanguage {
        Box::new(Self {})
    }
    fn too_low(&self) -> &'static str {
        "今"
    }
    fn too_high(&self) -> &'static str {
        "後"
    }
    fn ago(&self) -> &'static str {
        "前"
    }
    fn get_word(&self, tu: TimeUnit, _x: u64) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds => "ナノ秒",
            Microseconds => "マイクロ秒",
            Milliseconds => "ミリ秒",
            Seconds => "秒",
            Minutes => "分",
            Hours => "時間",
            Days => "日",
            Weeks => "週間",
            Months => "月",
            Years => "年",
        }
    }
    // Japanese doesn't separate words with spaces, so drop the default " "
    // between the value, the unit, and the trailing "前": "5 分 前" -> "5分前".
    fn between_value_and_word(&self) -> &str {
        ""
    }
    fn between_chunks(&self) -> &str {
        ""
    }
    fn override_space_near_ago(&self) -> &str {
        ""
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;

    fn test_with_formatter<L: Language>(mut f: Formatter<L>) {
        f.min_unit(TimeUnit::Seconds);
        assert_eq!(f.convert(Duration::from_secs(0)), "今");
        assert_eq!(f.convert(Duration::from_nanos(42)), "今");
        assert_eq!(f.convert(Duration::from_micros(42)), "今");
        assert_eq!(f.convert(Duration::from_millis(42)), "今");
        assert_eq!(f.convert(Duration::from_secs(42)), "42秒前");
        assert_eq!(f.convert(Duration::from_mins(42)), "42分前");
        assert_eq!(f.convert(Duration::from_hours(2)), "2時間前");
        assert_eq!(f.convert(Duration::from_hours(24)), "1日前");
        assert_eq!(f.convert(Duration::from_hours(7 * 24)), "1週間前");

        f.num_items(2);
        assert_eq!(f.convert(Duration::from_secs(3600 + 42)), "1時間42秒前");

        f.min_unit(TimeUnit::Nanoseconds);
        assert_eq!(f.convert(Duration::from_nanos(42)), "42ナノ秒前");
    }

    test_with_formatter(Formatter::with_language(Japanese));
    test_with_formatter(Formatter::with_language(Japanese.clone_boxed()));
}
