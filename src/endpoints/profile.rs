use crate::{Fyers, FyersError, Profile};
use crate::{urls, utils};

impl Fyers {
    /// Fetch basic details about your fyers account.
    pub async fn profile(&self) -> Result<Profile, FyersError> {
        let url = format!("{}/profile", urls::API_V3);
        let response = self.get(&url).await?;
        utils::get_field_and_deserialize(&response, "data")
    }
}
