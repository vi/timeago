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
pub fn from_isolang(x: isolang::Language) -> Option<Box<dyn super::Language>> {
    Some(match x {
        x if x.to_name() == "English"    => Box::new(english::English),
        x if x.to_name() == "Chinese"    => Box::new(chinese::Chinese),
        x if x.to_name() == "Japanese"   => Box::new(japanese::Japanese),
        x if x.to_name() == "Russian"    => Box::new(russian::Russian),
        x if x.to_name() == "German"     => Box::new(german::German),
        x if x.to_name() == "Belarusian" => Box::new(belarusian::Belarusian),
        x if x.to_name() == "Polish"     => Box::new(polish::Polish),
        x if x.to_name() == "Swedish"    => Box::new(swedish::Swedish),
        x if x.to_name() == "Romanian"   => Box::new(romanian::Romanian),
        x if x.to_name() == "Turkish"    => Box::new(turkish::Turkish),
        x if x.to_name() == "French"     => Box::new(french::French),
        x if x.to_name() == "Spanish"    => Box::new(spanish::Spanish),
        x if x.to_name() == "Danish"     => Box::new(danish::Danish),
        x if x.to_name() == "Portuguese" => Box::new(portuguese::Portuguese),
        _ => return None,
    })
}
