#![allow(missing_docs)]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[cfg(feature = "isolang")]
extern crate isolang;

pub mod belarusian;
pub mod chinese;
pub mod english;
pub mod german;
pub mod japanese;
pub mod polish;
pub mod portuguese;
pub mod romanian;
pub mod russian;
pub mod swedish;
pub mod turkish;
pub mod french;
pub mod spanish;
pub mod danish;
pub mod italian;
pub mod ukrainian;

/// Helper function to make a language dynamically dispatched
pub fn boxup<L: super::Language + Send + Sync + 'static>(x: L) -> super::BoxedLanguage {
    Box::new(x) as super::BoxedLanguage
}

/// A public use for a public dependency.
#[cfg(feature = "isolang")]
pub use self::isolang::Language as IsolangLanguage;

/// Requires `isolang` Cargo feature
///
/// Try converting a isolang's language into our dynamically dispatched language
/// ```
/// extern crate isolang;
/// extern crate timeago;
/// let il = isolang::Language::from_639_1("ru").unwrap();
/// let l = timeago::from_isolang(il).unwrap();
/// let f = timeago::Formatter::with_language(l);
/// let d = std::time::Duration::from_secs(3600);
/// assert_eq!(f.convert(d), "1 час назад");
/// ```
#[cfg(feature = "isolang")]
pub fn from_isolang(x: isolang::Language) -> Option<super::BoxedLanguage> {
    Some(match x {
        x if x.to_name() == "English"    => boxup(english::English),
        x if x.to_name() == "Chinese"    => boxup(chinese::Chinese),
        x if x.to_name() == "Japanese"   => boxup(japanese::Japanese),
        x if x.to_name() == "Russian"    => boxup(russian::Russian),
        x if x.to_name() == "German"     => boxup(german::German),
        x if x.to_name() == "Belarusian" => boxup(belarusian::Belarusian),
        x if x.to_name() == "Polish"     => boxup(polish::Polish),
        x if x.to_name() == "Swedish"    => boxup(swedish::Swedish),
        x if x.to_name() == "Romanian"   => boxup(romanian::Romanian),
        x if x.to_name() == "Turkish"    => boxup(turkish::Turkish),
        x if x.to_name() == "French"     => boxup(french::French),
        x if x.to_name() == "Spanish"    => boxup(spanish::Spanish),
        x if x.to_name() == "Danish"     => boxup(danish::Danish),
        x if x.to_name() == "Portuguese" => boxup(portuguese::Portuguese),
        x if x.to_name() == "Italian"    => boxup(italian::Italian),
        x if x.to_name() == "Ukrainian"  => boxup(ukrainian::Ukrainian),
        _ => return None,
    })
}
