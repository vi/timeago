use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Japanese;
impl Language for Japanese {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
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
}
