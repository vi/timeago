use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Chinese;
impl Language for Chinese {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "刚刚" }
    fn too_high(&self) -> &'static str { "大于" }
    fn ago(&self)      -> &'static str { "之前" }
    fn get_word(&self, tu: TimeUnit, _x: u64) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "纳秒",
            Microseconds  =>  "微秒",
            Milliseconds  =>  "毫秒",
            Seconds       =>  "秒",
            Minutes       =>  "分",
            Hours         =>  "小时",
            Days          =>  "天",
            Weeks         =>  "周",
            Months        =>  "月",
            Years         =>  "年",
        }
    }
}
