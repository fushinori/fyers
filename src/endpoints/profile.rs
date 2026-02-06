use crate::{Fyers, FyersError, Profile};

impl Fyers {
    /// Fetch basic details about your fyers account.
    pub async fn profile(&self) -> Result<Profile, FyersError> {
        self.get("profile").await
    }
}
