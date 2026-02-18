use serde::de::DeserializeOwned;

use crate::FyersError;

// Helper function to return deserialized object from a Value.
pub(crate) fn get_field_and_deserialize<T>(
    value: &serde_json::Value,
    field: &'static str,
) -> Result<T, FyersError>
where
    T: DeserializeOwned,
{
    let value = value.get(field).ok_or(FyersError::MissingField(field))?;

    Ok(T::deserialize(value)?)
}
