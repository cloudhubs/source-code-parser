use std::collections::{BTreeMap, HashMap};

use runestick::{Object, Value};

/// Output type from the library
pub type RessaResult = HashMap<String, BTreeMap<String, Value>>;

/// Errors encounterable accessing Rune data
pub enum Error {
    MissingKey(String),
    InvalidKey(String),
    InvalidType(String),
    RuneAcquisition,
    RuneAccess(runestick::AccessError),
}

#[macro_export]
macro_rules! coerce {
    ( $value:tt, $target:tt ) => {
        match $value.clone().$target() {
            ::std::result::Ok(obj) => match obj.take() {
                ::std::result::Ok(obj) => ::std::result::Ok(obj),
                ::std::result::Err(err) => {
                    ::std::result::Err($crate::ressa::result::Error::RuneAccess(err))
                }
            },
            ::std::result::Err(_) => {
                ::std::result::Err($crate::ressa::result::Error::RuneAcquisition)
            }
        }
    };
}

#[macro_export]
macro_rules! extract {
    ( $value:ident, $name:tt, $target:tt ) => {
        match $value.get($name) {
            ::std::result::Some(obj) => coerce!(obj, $target),
            ::std::result::None => ::std::result::Err(
                $crate::ressa::result::Error::Error::MissingKey($name.to_string()),
            ),
        }
    };
}

pub fn extract_object(obj: Object) -> BTreeMap<String, Value> {
    obj.into_inner()
}

#[macro_export]
macro_rules! extract_vec {
    ( $value:ident, $name:tt, $target:tt ) => {{
        let mut converted = std::vec![];
        for elem in extract!($value, $name, into_vec)?.iter() {
            converted.push(coerce!(elem, $target)?);
        }
        ::std::result::Ok(converted)
    }};
}

/// Retrieve an object from the map, returning it or an error indicating a missing key
pub fn get_object<'a>(
    result: &'a RessaResult,
    key: &str,
) -> Result<&'a BTreeMap<String, Value>, Error> {
    return match result.get(key) {
        Some(obj) => Ok(obj),
        None => Err(Error::MissingKey(key.to_string())),
    };
}
