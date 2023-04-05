//! Taxi structs and data structures

pub mod prelude {
    pub use {
        crate::taxi::{
            taxi_avail::TaxiAvailResp,
            taxi_stands::{TaxiStand, TaxiStandsResp},
        },
        crate::utils::Coordinates,
    };
}

pub mod taxi_avail {
    use crate::utils::Coordinates;
    use serde::{Deserialize, Serialize};

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/Taxi-Availability";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct InternalCoordinates {
        #[serde(alias = "Longitude")]
        pub long: f64,

        #[serde(alias = "Latitude")]
        pub lat: f64,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TaxiAvailResp {
        pub value: Vec<InternalCoordinates>,
    }

    impl From<TaxiAvailResp> for Vec<Coordinates> {
        fn from(data: TaxiAvailResp) -> Self {
            data.value.into_iter().map(|f| f.into()).collect()
        }
    }

    impl From<InternalCoordinates> for Coordinates {
        fn from(data: InternalCoordinates) -> Self {
            Self {
                lat: data.lat,
                long: data.long,
            }
        }
    }
}

pub mod taxi_stands {
    use crate::utils::de::from_str_to_bool;
    use serde::{Deserialize, Serialize};

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/TaxiStands";

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum TaxiStandOwner {
        LTA,
        CCS,
        Private,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum TaxiStandType {
        /// Allow taxis to queue in the taxi bays and wait for passengers
        Stand,

        /// Allow taxis to perform immediate pick up and drop off of passengers
        Stop,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct TaxiStand {
        pub taxi_code: String,

        #[serde(alias = "Latitude")]
        pub lat: f64,

        #[serde(alias = "Longitude")]
        pub long: f64,

        #[serde(alias = "Bfa", deserialize_with = "from_str_to_bool")]
        pub is_barrier_free: bool,

        #[serde(alias = "Ownership")]
        pub owner: TaxiStandOwner,

        #[serde(alias = "Type")]
        pub stand_type: TaxiStandType,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TaxiStandsResp {
        value: Vec<TaxiStand>,
    }

    impl From<TaxiStandsResp> for Vec<TaxiStand> {
        fn from(data: TaxiStandsResp) -> Self {
            data.value
        }
    }
}
