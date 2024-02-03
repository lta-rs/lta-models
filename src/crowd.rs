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
    use time::{
        serde::{iso8601, timestamp},
        OffsetDateTime,
    };

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
    pub enum CrowdLevel {
        #[serde(rename = "l")]
        Low,

        #[serde(rename = "h")]
        High,

        #[serde(rename = "m")]
        Moderate,

        #[default]
        #[serde(other)]
        Na,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct StationCrowdLevelRaw {
        pub station: StationCode,

        /// Time in GMT+8
        #[serde(with = "iso8601")]
        pub start_time: OffsetDateTime,

        /// Time in GMT+8
        #[serde(with = "iso8601")]
        pub end_time: OffsetDateTime,

        pub crowd_level: CrowdLevel,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StationCrowdLevel {
        pub station: StationCode,
        #[serde(with = "timestamp")]
        pub start_time: OffsetDateTime,
        #[serde(with = "timestamp")]
        pub end_time: OffsetDateTime,
        pub crowd_level: CrowdLevel,
    }

    impl From<StationCrowdLevelRaw> for StationCrowdLevel {
        fn from(r: StationCrowdLevelRaw) -> Self {
            Self {
                station: r.station,
                start_time: r.start_time,
                end_time: r.end_time,
                crowd_level: r.crowd_level,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct StationCrowdLevelRawResp {
        value: Vec<StationCrowdLevelRaw>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct CrowdDensityForecastRaw {
        /// Time in GMT+8
        #[serde(alias = "Date", with = "iso8601")]
        pub datetime: OffsetDateTime,
        pub stations: Vec<StationCrowdForecast>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CrowdDensityForecast {
        #[serde(with = "timestamp")]
        pub datetime: OffsetDateTime,
        pub stations: Vec<StationCrowdForecast>,
    }

    impl From<CrowdDensityForecastRaw> for CrowdDensityForecast {
        fn from(r: CrowdDensityForecastRaw) -> Self {
            Self {
                datetime: r.datetime,
                stations: r.stations
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct CrowdDensityForecastRawResp {
        value: Vec<CrowdDensityForecastRaw>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct StationCrowdForecast {
        pub station: StationCode,
        pub interval: Vec<CrowdInterval>,
    }

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct CrowdInterval {
        #[serde(with = "iso8601")]
        pub start: OffsetDateTime,
        pub crowd_level: CrowdLevel,
    }

    impl From<StationCrowdLevelRawResp> for Vec<StationCrowdLevel> {
        fn from(data: StationCrowdLevelRawResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }

    impl From<CrowdDensityForecastRawResp> for CrowdDensityForecast {
        fn from(data: CrowdDensityForecastRawResp) -> Self {
            Self::from(data.value[0].clone())
        }
    }
}

pub mod passenger_vol {
    use serde::{Deserialize, Serialize};

    pub const FORMAT: &str = "%Y%m";

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
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

        #[default]
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
