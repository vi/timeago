use super::super::{Language, TimeUnit};

/// Default language for timeago
#[derive(Default)]
pub struct Turkish;
impl Language for Turkish {
    fn clone_boxed(&self) -> super::super::BoxedLanguage { Box::new(Self{}) }
    fn too_low (&self) -> &'static str { "şimdi" }
    fn too_high(&self) -> &'static str { "eski" }
    fn ago(&self)      -> &'static str { "önce" }
    fn get_word(&self, tu: TimeUnit, _x: u64) -> &'static str {
        use TimeUnit::*;
        match tu {
            Nanoseconds   =>  "nanosaniye",
            Microseconds  =>  "mikrosaniye",
            Milliseconds  =>  "milisaniye",
            Seconds       =>  "saniye",
            Minutes       =>  "dakika",
            Hours         =>  "saat",
            Days          =>  "gün",
            Weeks         =>  "hafta",
            Months        =>  "ay",
            Years         =>  "yıl",
        }
    }
}

#[test]
fn test() {
    use super::super::Formatter;
    use std::time::Duration;
    let f = Formatter::with_language(Turkish);
    assert_eq!(f.convert(Duration::from_secs(60)), "1 dakika önce");
    assert_eq!(f.convert(Duration::from_secs(2)), "2 saniye önce");
    assert_eq!(f.convert(Duration::from_secs(5)), "5 saniye önce");
    assert_eq!(f.convert(Duration::from_secs(12)), "12 saniye önce");
    assert_eq!(f.convert(Duration::from_secs(1*60*60)), "1 saat önce");
    assert_eq!(f.convert(Duration::from_secs(2*60*60)), "2 saat önce");
    assert_eq!(f.convert(Duration::from_secs(1*24*60*60)), "1 gün önce");
    assert_eq!(f.convert(Duration::from_secs(2*24*60*60)), "2 gün önce");
    assert_eq!(f.convert(Duration::from_secs(1*7*24*60*60)), "1 hafta önce");
    assert_eq!(f.convert(Duration::from_secs(2*7*24*60*60)), "2 hafta önce");
    assert_eq!(f.convert(Duration::from_secs(1*3600*12*366)), "6 ay önce");
    assert_eq!(f.convert(Duration::from_secs(1*3600*24*366)), "1 yıl önce");
    assert_eq!(f.convert(Duration::from_secs(2*3600*24*366)), "2 yıl önce");
    assert_eq!(f.convert(Duration::from_secs(100*3600*24*366)), "100 yıl önce");
    assert_eq!(f.convert(Duration::from_secs(101*3600*24*366)), "101 yıl önce");
    assert_eq!(f.convert(Duration::from_secs(111*3600*24*366)), "111 yıl önce");
}