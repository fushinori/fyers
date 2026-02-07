use crate::utils;
use crate::{Fyers, FyersError, Profile};

impl Fyers {
    /// Fetch basic details about your fyers account.
    pub async fn profile(&self) -> Result<Profile, FyersError> {
        let response = self.get("profile").await?;
        utils::get_field_and_deserialize(&response, "data")
    }
}
