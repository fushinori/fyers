use serde::Deserialize;

/// Basic account profile information returned by the Fyers API.
#[derive(Debug, Deserialize, Clone)]
pub struct Profile {
    /// Name of the client.
    pub name: String,

    /// Optional display name chosen by the client.
    #[serde(default)]
    pub display_name: Option<String>,

    /// Client ID of the user.
    pub fy_id: String,

    /// URL to the user's profile image, if available.
    #[serde(default)]
    pub image: Option<String>,

    /// Registered email address of the client.
    pub email_id: String,

    /// Permanent Account Number (PAN) of the client.
    #[serde(rename = "PAN")]
    pub pan: String,

    /// Date when the account PIN was last changed.
    #[serde(default)]
    pub pin_change_date: Option<String>,

    /// Date when the account password was last changed.
    #[serde(default)]
    pub pwd_change_date: Option<String>,

    /// Registered mobile number.
    pub mobile_number: String,

    /// Whether Time-based One-Time Password (TOTP) is enabled.
    pub totp: bool,

    /// Number of days remaining until the current password expires.
    pub pwd_to_expire: i32,

    /// Whether DDPI (Demat Debit and Pledge Instruction) is enabled.
    pub ddpi_enabled: bool,

    /// Whether Margin Trading Facility (MTF) is enabled.
    pub mtf_enabled: bool,
}
