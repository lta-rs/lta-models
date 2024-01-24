//! Enums for buses and operators
//! Used for transforming stringly typed data from API to enums

pub mod prelude {
    pub use {
        crate::bus_enums::{Operator, BusCategory, BusFeature, BusLoad, BusType}
    };
}

use serde::{Deserialize, Serialize};

/// SBST -> SBS Transit
///
/// SMRT -> SMRT Corporation
///
/// TTS -> Tower Transit Singapore
///
/// GAS -> Go Ahead Singapore
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub enum Operator {
    #[serde(rename = "SBST")]
    Sbst,
    
    #[serde(rename = "SMRT")]
    Smrt,
    
    #[serde(rename = "TTS")]
    Tts,
    
    #[serde(rename = "GAS")]
    Gas,

    #[serde(other)]
    Unknown,
}

/// SD -> Single Decker
///
/// DD -> Double Decker
///
/// BD -> Bendy
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub enum BusType {
    #[serde(rename = "SD")]
    SingleDecker,

    #[serde(rename = "DD")]
    DoubleDecker,

    #[serde(rename = "BD")]
    Bendy,

    #[serde(other)]
    Unknown,
}

impl Default for BusType {
    fn default() -> Self {
        BusType::Unknown
    }
}

/// SEA -> Seats available
///
/// SDA -> Standing available
///
/// LSD -> Limited standing
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BusLoad {
    #[serde(rename = "SEA")]
    SeatsAvailable,

    #[serde(rename = "SDA")]
    StandingAvailable,

    #[serde(rename = "LSD")]
    LimitedStanding,

    #[serde(other)]
    Unknown,
}

impl Default for BusLoad {
    fn default() -> Self {
        BusLoad::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BusFeature {
    #[serde(rename = "WAB")]
    WheelChairAccessible,

    #[serde(other)]
    Unknown,
}

impl Default for BusFeature {
    fn default() -> Self {
        BusFeature::Unknown
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
pub enum BusCategory {
    #[serde(alias = "EXPRESS")]
    Express,

    #[serde(alias = "FEEDER")]
    Feeder,

    #[serde(alias = "INDUSTRIAL")]
    Industrial,

    #[serde(alias = "TOWNLINK")]
    TownLink,

    #[serde(alias = "TRUNK")]
    Trunk,

    #[serde(alias = "2-TIER FLAT FARE")]
    TwoTierFlatFare,

    #[serde(alias = "FLATFEE")]
    FlatFee,

    #[serde(alias = "NIGHT SERVICE")]
    NightService,

    #[serde(alias = "CITY_LINK")]
    CityLink,

    #[serde(alias = "FLAT FARE $2.00")]
    FlatFareTwoDollar,

    #[serde(other)]
    Unknown,
}
