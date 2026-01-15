use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Korean;
impl Language for Korean {
    fn clone_boxed(&self) -> super::super::BoxedLanguage {
        Box::new(Self {})
    }
    fn too_low(&self) -> &'static str {
        "방금"
    }
    fn too_high(&self) -> &'static str {
        // https://stdict.korean.go.kr/search/searchView.do?word_no=241146&searchKeywordTo=3
        "오래전"
    }
    fn ago(&self) -> &'static str {
        "전"
    }
    fn get_word(&self, tu: TimeUnit, _x: u64) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds => "나노초",
            Microseconds => "마이크로초",
            Milliseconds => "밀리초",
            Seconds => "초",
            Minutes => "분",
            Hours => "시간",
            Days => "일",
            Weeks => "주",
            Months => "개월",
            Years => "년",
        }
    }
    fn between_value_and_word(&self) -> &str {
        // https://www.korean.go.kr/kornorms/regltn/regltnView.do?regltn_code=0001&regltn_no=263
        ""
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let mut f = Formatter::with_language(Korean);
    f.min_unit(TimeUnit::Seconds);
    assert_eq!(f.convert(Duration::from_secs(0)), "방금");
    assert_eq!(f.convert(Duration::from_nanos(42)), "방금");
    assert_eq!(f.convert(Duration::from_micros(42)), "방금");
    assert_eq!(f.convert(Duration::from_millis(42)), "방금");
    assert_eq!(f.convert(Duration::from_secs(42)), "42초 전");
    assert_eq!(f.convert(Duration::from_mins(42)), "42분 전");
    assert_eq!(f.convert(Duration::from_hours(2)), "2시간 전");
    assert_eq!(f.convert(Duration::from_hours(23)), "23시간 전");
    assert_eq!(f.convert(Duration::from_hours(24)), "1일 전");
    assert_eq!(f.convert(Duration::from_hours(24 + 1)), "1일 전");
    assert_eq!(f.convert(Duration::from_hours(2 * 24 - 1)), "1일 전");
    assert_eq!(f.convert(Duration::from_hours(2 * 24)), "2일 전");
    assert_eq!(f.convert(Duration::from_hours(2 * 24 + 1)), "2일 전");
    assert_eq!(f.convert(Duration::from_hours(3 * 24 - 1)), "2일 전");
    assert_eq!(f.convert(Duration::from_hours(3 * 24)), "3일 전");
    assert_eq!(f.convert(Duration::from_hours(3 * 24 + 1)), "3일 전");
    assert_eq!(f.convert(Duration::from_hours(42 * 24)), "1개월 전");
    assert_eq!(f.convert(Duration::from_hours(364 * 24)), "11개월 전");
    assert_eq!(f.convert(Duration::from_hours(365 * 24)), "11개월 전");
    assert_eq!(f.convert(Duration::from_hours(366 * 24)), "1년 전");
    assert_eq!(f.convert(Duration::from_hours(42 * 366 * 24)), "42년 전");

    f.min_unit(TimeUnit::Nanoseconds);
    assert_eq!(f.convert(Duration::from_nanos(42)), "42나노초 전");
    assert_eq!(f.convert(Duration::from_micros(42)), "42마이크로초 전");
    assert_eq!(f.convert(Duration::from_millis(42)), "42밀리초 전");

    f.max_unit(TimeUnit::Months);
    assert_eq!(f.convert(Duration::from_hours(365 * 24)), "11개월 전");
    assert_eq!(f.convert(Duration::from_hours(366 * 24)), "12개월 전");

    f.max_duration(Duration::from_hours(365 * 24));
    assert_eq!(f.convert(Duration::from_hours(365 * 24)), "11개월 전");
    assert_eq!(f.convert(Duration::from_hours(366 * 24)), "오래전");
}
