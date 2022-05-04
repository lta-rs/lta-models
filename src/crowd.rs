//! Crowd structs and data structures

pub mod prelude {
    pub use crate::crowd::crowd_density::{
        CrowdInterval, CrowdLevel, StationCrowdForecast, StationCrowdLevel,
    };
    pub use crate::crowd::passenger_vol::{Link, PassengerVolRawResp, VolType};
}

pub mod crowd_density {
    use chrono::{DateTime, FixedOffset};
    use serde::{Deserialize, Serialize};
    use crate::train::StationCode;
    
    pub const URL_CROWD_DENSITY_RT: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PCDRealTime";

    pub const URL_CROWD_FORECAST: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PCDForecast";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "lowercase"))]
    pub enum CrowdLevel {
        Low,
        High,
        Moderate,

        #[serde(other)]
        Na,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct StationCrowdLevel {
        pub station: StationCode,
        pub start_time: DateTime<FixedOffset>,
        pub end_time: DateTime<FixedOffset>,
        pub crowd_level: CrowdLevel,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StationCrowdLevelRawResp {
        value: Vec<StationCrowdLevel>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct CrowdForecast {
        #[serde(alias = "Date")]
        pub datetime: DateTime<FixedOffset>,
        pub stations: Vec<StationCrowdForecast>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CrowdForecastRawResp {
        value: Vec<CrowdForecast>,
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
        pub start: DateTime<FixedOffset>,
        pub crowd_level: CrowdLevel,
    }

    impl From<StationCrowdLevelRawResp> for Vec<StationCrowdLevel> {
        fn from(data: StationCrowdLevelRawResp) -> Self {
            data.value
        }
    }

    impl From<CrowdForecastRawResp> for CrowdForecast {
        fn from(mut data: CrowdForecastRawResp) -> Self {
            data.value.pop().unwrap()
        }
    }
}

pub mod passenger_vol {
    use serde::{Deserialize, Serialize};

    pub const URL_BY_BUS_STOPS: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Bus";

    pub const URL_BY_OD_BUS_STOPS: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/PV/ODBus";

    pub const URL_BY_TRAIN: &str = "http://datamall2.mytransport.sg/ltaodataservice/PV/Train";

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
