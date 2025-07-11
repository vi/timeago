#![cfg_attr(rustfmt, rustfmt_skip)]
use super::super::{Language, TimeUnit};

/// Default language for timeago
#[derive(Default)]
pub struct Thai;

impl Language for Thai {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low(&self) -> &'static str { "ตอนนี้" }      // "now"
    fn too_high(&self) -> &'static str { "นานมาแล้ว" }  // "old"
    fn ago(&self) -> &'static str { "ที่แล้ว" }         // "ago"

    fn prefix_space_ago(&self) -> bool {
        false
    }
    
    fn get_word(&self, tu: TimeUnit, x: u64) -> &'static str {
        use TimeUnit::*;
        if x == 1 {
            match tu {
                Nanoseconds   => "นาโนวินาที",
                Microseconds  => "ไมโครวินาที",
                Milliseconds  => "มิลลิวินาที",
                Seconds       => "วินาที",
                Minutes       => "นาที",
                Hours         => "ชั่วโมง",
                Days          => "วัน",
                Weeks         => "สัปดาห์",
                Months        => "เดือน",
                Years         => "ปี",
            }
        } else {
            // In Thai, plural forms are often the same as singular, so we can reuse the same words
            match tu {
                Nanoseconds   => "นาโนวินาที",
                Microseconds  => "ไมโครวินาที",
                Milliseconds  => "มิลลิวินาที",
                Seconds       => "วินาที",
                Minutes       => "นาที",
                Hours         => "ชั่วโมง",
                Days          => "วัน",
                Weeks         => "สัปดาห์",
                Months        => "เดือน",
                Years         => "ปี",
            }
        }
    }
}
