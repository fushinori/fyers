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

pub(crate) fn map_api_error(code: i32, message: String) -> FyersError {
    match code {
        -8 => FyersError::TokenExpired,

        // -15 | -16 | -17
        -17..=-15 => FyersError::InvalidToken,

        -50 | 400 => FyersError::InvalidParams(message),

        -51 => FyersError::InvalidOrderId,

        -53 => FyersError::InvalidPositionId,

        -99 => FyersError::OrderRejected(message),

        -300 => FyersError::InvalidSymbol,

        -352 => FyersError::InvalidAppId,

        -429 => FyersError::RateLimited,

        _ => FyersError::Api { code, message },
    }
}
