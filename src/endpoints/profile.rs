use crate::utils;
use crate::{Fyers, FyersError, Profile};

impl Fyers {
    /// Fetch basic details about your fyers account.
    pub async fn profile(&self) -> Result<Profile, FyersError> {
        let url = format!("{}/profile", self.base_urls.api_v3);
        let response = self.get(&url).await?;
        utils::get_field_and_deserialize(&response, "data")
    }
}
