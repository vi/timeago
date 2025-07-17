use super::super::{Language, TimeUnit};

#[derive(Default)]
pub struct Thai;
impl Language for Thai {
    fn clone_boxed(&self) -> super::super::BoxedLanguage {
        Box::new(Self {})
    }
    fn too_low(&self) -> &'static str {
        "ตอนนี้"
    }
    fn too_high(&self) -> &'static str {
        "นานมาแล้ว"
    }
    fn ago(&self) -> &'static str {
        "ที่แล้ว"
    }
    fn extra_space(&self) -> &str {
        ""
    }
    fn get_word(&self, tu: TimeUnit, _: u64) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds => "นาโนวินาที",
            Microseconds => "ไมโครวินาที",
            Milliseconds => "มิลลิวินาที",
            Seconds => "วินาที",
            Minutes => "นาที",
            Hours => "ชั่วโมง",
            Days => "วัน",
            Weeks => "สัปดาห์",
            Months => "เดือน",
            Years => "ปี",
        }
    }
}
