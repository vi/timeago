#[cfg(all(feature = "isolang", feature = "translations"))]
extern crate isolang;
extern crate timeago;

use std::io::BufRead;

fn main() {
    let ls = std::env::args().nth(1).expect(
        "Usage: timeago  <ISO 639-1 two-letter language code>
Then feed unsigned numbers (seconds) into it.
",
    );
    let l;
    #[cfg(all(feature = "isolang", feature = "translations"))]
    {
        l = timeago::from_isolang(isolang::Language::from_639_1(&ls).unwrap()).unwrap();
    }
    #[cfg(any(not(feature = "isolang"), not(feature = "translations")))]
    {
        if ls != "en" {
            eprintln!("Enable both `isolang` and `translations` Cargo features for any languages apart from `en`");
            return;
        }
        l = timeago::English;
    }
    let mut f = timeago::Formatter::with_language(l);
    f.num_items(3);

    let si1 = std::io::stdin();
    let si = si1.lock();
    for line in si.lines() {
        let sec: u64 = line.unwrap().parse().unwrap();
        println!("{}", f.convert(std::time::Duration::from_secs(sec)));
    }
}
