//! Enums for buses and operators
//! Used for transforming stringly typed data from API to enums

pub mod prelude {
    pub use crate::bus_enums::{BusCategory, BusFeature, BusLoad, BusType, Operator};
}

use serde::{Deserialize, Serialize};

/// SBST -> SBS Transit
///
/// SMRT -> SMRT Corporation
///
/// TTS -> Tower Transit Singapore
///
/// GAS -> Go Ahead Singapore
#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, Default)]
pub enum Operator {
    #[serde(rename = "SBST")]
    Sbst,

    #[serde(rename = "SMRT")]
    Smrt,

    #[serde(rename = "TTS")]
    Tts,

    #[serde(rename = "GAS")]
    Gas,

    #[default]
    #[serde(other)]
    Unknown,
}

/// SD -> Single Decker
///
/// DD -> Double Decker
///
/// BD -> Bendy
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Default)]
#[serde(rename_all = "PascalCase")]
pub enum BusType {
    #[serde(rename = "SD")]
    SingleDecker,

    #[serde(rename = "DD")]
    DoubleDecker,

    #[serde(rename = "BD")]
    Bendy,

    #[default]
    #[serde(other)]
    Unknown,
}

/// SEA -> Seats available
///
/// SDA -> Standing available
///
/// LSD -> Limited standing
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Default)]
pub enum BusLoad {
    #[serde(rename = "SEA")]
    SeatsAvailable,

    #[serde(rename = "SDA")]
    StandingAvailable,

    #[serde(rename = "LSD")]
    LimitedStanding,

    #[default]
    #[serde(other)]
    Unknown,
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Default)]
pub enum BusFeature {
    #[serde(rename = "WAB")]
    WheelChairAccessible,

    #[default]
    #[serde(other)]
    Unknown,
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize, Default)]
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

    #[default]
    #[serde(other)]
    Unknown,
}
