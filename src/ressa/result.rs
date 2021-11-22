use std::collections::{BTreeMap, HashMap};

use runestick::Value;

/// Output type from the library
pub type RessaResult = HashMap<String, BTreeMap<String, Value>>;

/// Errors encounterable accessing Rune data
pub enum Error {
    MissingKeyError(String),
    InvalidKeyError(String),
    InvalidTypeError(String),
    RuneAcquisitionError,
    RuneAccessError(runestick::AccessError),
}

#[macro_export]
macro_rules! coerce {
    ( $value:tt, $target:tt ) => {
        match $value.clone().$target() {
            Ok(obj) => match obj.take() {
                Ok(obj) => Ok(obj),
                Err(err) => Err(Error::RuneAccessError(err)),
            },
            Err(_) => Err(Error::RuneAcquisitionError),
        }
    };
}

#[macro_export]
macro_rules! extract {
    ( $value:ident, $name:tt, $target:tt ) => {
        match $value.get($name) {
            Some(obj) => coerce!(obj, $target),
            None => Err(Error::MissingKeyError($name.to_string())),
        }
    };
}

#[macro_export]
macro_rules! extract_vec {
    ( $value:ident, $name:tt, $target:tt ) => {
        {
            let mut converted = vec![];
            for elem in extract!($value, $name, into_vec)?.iter() {
                converted.push(coerce!(elem, $target)?);
            }
            Ok(converted)
        }
    };
}

/// Retrieve an object from the map, returning it or an error indicating a missing key
pub fn get_object<'a>(
    result: &'a RessaResult,
    key: &str,
) -> Result<&'a BTreeMap<String, Value>, Error> {
    return match result.get(key) {
        Some(obj) => Ok(obj),
        None => Err(Error::MissingKeyError(key.to_string())),
    };
}
