use std::collections::BTreeMap;

use rune::runtime::{Object, Shared, Value};

/// Output type from the library
pub type RessaResult = BTreeMap<String, Value>;

/// Errors encounterable accessing Rune data
pub enum Error {
    MissingKey(String),
    InvalidKey(String),
    InvalidType(String),
    RuneAcquisition,
    RuneAccess(rune::runtime::AccessError),
}

/// Coerces a `runestick::Value` into a `T`
pub fn coerce_primitive<I, T, E>(value: &Value, into: I) -> Result<T, Error>
where
    I: FnOnce(Value) -> Result<T, E>,
{
    into(value.clone()).map_err(|_| Error::RuneAcquisition)
}

/// Extracts a value from the given object at key `name` and coerces it into `T`
pub fn extract_primitive<I, T, E>(
    obj: &BTreeMap<String, Value>,
    name: &str,
    into: I,
) -> Result<T, Error>
where
    I: FnOnce(Value) -> Result<T, E>,
{
    obj.get(name)
        .ok_or_else(|| Error::MissingKey(name.to_string()))
        .and_then(|value| coerce_primitive(value, into))
}

/// Coerces a `runestick::Value` into a `Shared<T>`
pub fn coerce<I, T, E>(value: &Value, into: I) -> Result<T, Error>
where
    I: FnOnce(Value) -> Result<Shared<T>, E>,
{
    match into(value.clone()) {
        Ok(obj) => obj.take().map_err(Error::RuneAccess),
        Err(_) => Err(Error::RuneAcquisition),
    }
}

/// Extracts a value from the given object at key `name` and coerces it into `T`
pub fn extract<I, T, E>(obj: &BTreeMap<String, Value>, name: &str, into: I) -> Result<T, Error>
where
    I: FnOnce(Value) -> Result<Shared<T>, E>,
{
    obj.get(name)
        .ok_or_else(|| Error::MissingKey(name.to_string()))
        .and_then(|value| coerce(value, into))
}

/// Coerces a `runestick::Object` into a `BTreeMap<String, Value>`
pub fn extract_object(obj: Object) -> BTreeMap<String, Value> {
    obj.into_inner()
}

/// Gets the `runestick::Vec` at key `name` and coerces it into a `Vec<T>`
pub fn extract_vec<I, T, E>(
    obj: &BTreeMap<String, Value>,
    name: &str,
    into: I,
) -> Result<Vec<T>, Error>
where
    I: Fn(Value) -> Result<Shared<T>, E> + Copy,
{
    let (result, err): (Vec<Option<T>>, Vec<Option<Error>>) = extract(obj, name, Value::into_vec)?
        .into_iter()
        // `into` has the Copy trait bound so it can be called multiple times when
        // captured by map's `FnMut` argument
        .map(|elem| coerce(&elem, into))
        .map(|obj| match obj {
            Ok(obj) => (Some(obj), None),
            Err(err) => (None, Some(err)),
        })
        .unzip();

    // Short-circuit any errors that occurred
    if let Some(Some(bad_result)) = err.into_iter().find(|err| err.is_some()) {
        return Err(bad_result);
    }

    // Coerce results out of the vec
    Ok(result.into_iter().flatten().collect())
}
