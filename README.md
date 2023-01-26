# timeago
In Rust, format Duration into string like "1 hour ago" or "01hou".

Currently it does not [take the calendar into account](https://github.com/vi/timeago/issues/12) and assumes each month is about 30.4 days long.

Parsing such string back to a `Duration` is out of scope for this crate. Maybe see the [`chrono-english`](https://docs.rs/chrono-english) crate instead.

With `isolang` feature off, it supports Rust from version 1.24.

## API

[Documentation link](https://docs.rs/timeago/)

Simplified API excerpt (pseudocode):

```rust
pub struct Formatter<L : Language = English>{...}

impl Formatter {
    pub fn new() -> Formatter<English>;
    pub fn with_language(l: Language) -> Self;
    pub fn num_items(&mut self, x: usize) -> &mut Self;
    pub fn max_unit(&mut self, x: TimeUnit) -> &mut Self;
    pub fn min_unit(&mut self, x: TimeUnit) -> &mut Self;
    pub fn too_low(&mut self, x: &'static str) -> &mut Self;
    pub fn too_high(&mut self, x: &'static str) -> &mut Self;
    pub fn max_duration(&mut self, x: Duration) -> &mut Self;
    pub fn ago(&mut self, x: &'static str) -> &mut Self;
    
    pub fn convert(&self, d: Duration) -> String;
    pub fn convert_chrono(&self, from: chrono::DateTime, to: chrono::DateTime) -> String;
}

pub fn from_isolang(x : isolang::Language) -> Option<Box<Language>>;

pub fn format_5chars(d: Duration) -> String;
```

A `Language` can be constructed from [isolang::Language](https://docs.rs/isolang/1/isolang/enum.Language.html).

## Translations

* English
* Russian
* French
* Portuguese (contributed)
* German (unchecked)
* Belarusian (unchecked)
* Polish (unchecked)
* Spanish (contributed)
* Chinese (contributed)
* Romanian (contributed)
* Swedish (contributed)
* Turkish (contributed)
* Japanese (contributed)
* Danish (contributed)
* Italian (contributed)
* Ukrainian (contributed)

If you checked some language and certify that it's allright, submit a pull request that removes "(unchecked)" or "(contributed)" in the list above.

## Tool

There is a helper command line tool that allows easier experimenting when adding a new translation:

```
$ cargo run --features isolang en
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/timeago en`
60
1 minute ago
7200
2 hours ago
```

# See also

* [chrono-humanize](https://docs.rs/crate/chrono-humanize)
* compound_duration - split `Duration` into weeks/days/minues/etc. parts
