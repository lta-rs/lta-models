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

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct InternalCoordinates {
        /// Original data already float
        #[serde(alias = "Longitude")]
        pub long: f64,

        /// Original data already float
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

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
    pub enum TaxiStandOwner {
        #[serde(rename = "LTA")]
        Lta,

        #[serde(rename = "CCS")]
        Ccs,

        Private,

        #[default]
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
    pub enum TaxiStandType {
        /// Allow taxis to queue in the taxi bays and wait for passengers
        Stand,

        /// Allow taxis to perform immediate pick up and drop off of passengers
        Stop,

        #[default]
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TaxiStandRaw {
        pub taxi_code: String,

        /// Original data already float
        #[serde(alias = "Latitude")]
        pub lat: f64,

        /// Original data already float
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
    pub struct TaxiStand {
        pub taxi_code: String,
        pub lat: f64,
        pub long: f64,
        pub is_barrier_free: bool,
        pub owner: TaxiStandOwner,
        pub stand_type: TaxiStandType,
        pub name: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TaxiStandsResp {
        value: Vec<TaxiStandRaw>,
    }

    impl From<TaxiStandRaw> for TaxiStand {

        #[inline(always)]
        fn from(r: TaxiStandRaw) -> Self {
            Self {
                taxi_code: r.taxi_code,
                lat: r.lat,
                long: r.long,
                is_barrier_free: r.is_barrier_free,
                owner: r.owner,
                stand_type: r.stand_type,
                name: r.name
            }
        }
    }

    impl From<TaxiStandsResp> for Vec<TaxiStand> {
        fn from(data: TaxiStandsResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}
