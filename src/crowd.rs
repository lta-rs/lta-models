//! Crowd structs and data structures

pub mod prelude {
    pub use crate::crowd::crowd_density::{
        CrowdDensityForecast, CrowdDensityForecastRawResp, CrowdInterval, CrowdLevel,
        StationCrowdForecast, StationCrowdLevel, StationCrowdLevelRawResp,
    };
    pub use crate::crowd::passenger_vol::{Link, PassengerVolRawResp, VolType};
}

pub mod crowd_density {
    use crate::train::StationCode;
    use serde::{Deserialize, Serialize};
    use time::{serde::iso8601, OffsetDateTime};

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_CROWD_DENSITY_RT: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PCDRealTime";

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_CROWD_FORECAST: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PCDForecast";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "lowercase"))]
    pub enum CrowdLevel {
        
        #[serde(alias = "l")]
        Low,
        
        #[serde(alias = "h")]
        High,
        
        #[serde(alias = "m")]
        Moderate,

        #[serde(other)]
        Na,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct StationCrowdLevel {
        pub station: StationCode,

        /// Time in GMT+8
        #[serde(deserialize_with = "iso8601::deserialize")]
        pub start_time: OffsetDateTime,

        /// Time in GMT+8
        #[serde(deserialize_with = "iso8601::deserialize")]
        pub end_time: OffsetDateTime,

        pub crowd_level: CrowdLevel,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StationCrowdLevelRawResp {
        value: Vec<StationCrowdLevel>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct CrowdDensityForecast {
        /// Time in GMT+8
        #[serde(alias = "Date", deserialize_with = "iso8601::deserialize")]
        pub datetime: OffsetDateTime,
        pub stations: Vec<StationCrowdForecast>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CrowdDensityForecastRawResp {
        value: Vec<CrowdDensityForecast>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct StationCrowdForecast {
        pub station: StationCode,
        pub interval: Vec<CrowdInterval>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct CrowdInterval {
        
        #[serde(deserialize_with = "iso8601::deserialize")]
        pub start: OffsetDateTime,
        pub crowd_level: CrowdLevel,
    }

    impl From<StationCrowdLevelRawResp> for Vec<StationCrowdLevel> {
        fn from(data: StationCrowdLevelRawResp) -> Self {
            data.value
        }
    }

    impl From<CrowdDensityForecastRawResp> for CrowdDensityForecast {
        fn from(mut data: CrowdDensityForecastRawResp) -> Self {
            data.value.pop().unwrap()
        }
    }
}

pub mod passenger_vol {
    use serde::{Deserialize, Serialize};

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_BY_BUS_STOPS: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Bus";

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_BY_OD_BUS_STOPS: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/ODBus";

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_BY_TRAIN: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Train";

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_BY_OD_TRAIN: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/ODTrain";

    pub const FORMAT: &str = "%Y%m";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub enum VolType {
        /// Returns tap in and tap out passenger volume by weekdays and
        /// weekends for individual bus stop
        BusStops,

        /// Returns number of trips by weekdays and weekends from origin to
        /// destination bus stops
        OdBusStop,

        /// Returns number of trips by weekdays and weekends from origin to
        /// destination train stations
        Train,

        /// Returns tap in and tap out passenger volume by weekdays and
        /// weekends for individual train station
        OdTrain,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct Link {
        #[serde(alias = "Link")]
        pub link: String,
    }

    impl From<Link> for String {
        fn from(data: Link) -> Self {
            data.link
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct PassengerVolRawResp {
        pub value: Vec<Link>,
    }

    impl From<PassengerVolRawResp> for Vec<String> {
        fn from(data: PassengerVolRawResp) -> Self {
            data.value.into_iter().map(|f| f.link).collect()
        }
    }
}
