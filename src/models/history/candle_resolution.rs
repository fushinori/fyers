use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub enum CandleResolution {
    #[serde(rename = "D")]
    Day,

    #[serde(rename = "5S")]
    Seconds5,

    #[serde(rename = "10S")]
    Seconds10,

    #[serde(rename = "15S")]
    Seconds15,

    #[serde(rename = "30S")]
    Seconds30,

    #[serde(rename = "45S")]
    Seconds45,

    #[serde(rename = "1")]
    Minute1,

    #[serde(rename = "2")]
    Minute2,

    #[serde(rename = "3")]
    Minute3,

    #[serde(rename = "5")]
    Minute5,

    #[serde(rename = "10")]
    Minute10,

    #[serde(rename = "15")]
    Minute15,

    #[serde(rename = "20")]
    Minute20,

    #[serde(rename = "30")]
    Minute30,

    #[serde(rename = "60")]
    Minute60,

    #[serde(rename = "120")]
    Minute120,

    #[serde(rename = "240")]
    Minute240,
}
