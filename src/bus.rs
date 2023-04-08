//! Bus structs and data structures

pub mod prelude {
    pub use {
        crate::bus::bus_arrival::{BusArrivalResp, RawBusArrivalResp},
        crate::bus::bus_routes::{BusRoute, BusRouteResp},
        crate::bus::bus_services::{BusService, BusServiceResp},
        crate::bus::bus_stops::{BusStop, BusStopsResp},
    };
}

pub mod bus_arrival {
    use serde::{Deserialize, Serialize};
    use time::{serde::iso8601, OffsetDateTime};

    use crate::bus_enums::{BusFeature, BusLoad, BusType, Operator};
    use crate::utils::de::{from_str, treat_error_as_none};

    #[cfg(feature = "fastfloat")]
    use crate::utils::de::from_str_fast_float;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusArrivalv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct RawArrivalBusService {
        pub service_no: String,

        pub operator: Operator,

        #[serde(deserialize_with = "treat_error_as_none")]
        pub next_bus: Option<NextBus>,

        #[serde(deserialize_with = "treat_error_as_none")]
        pub next_bus_2: Option<NextBus>,

        #[serde(deserialize_with = "treat_error_as_none")]
        pub next_bus_3: Option<NextBus>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct ArrivalBusService {
        pub service_no: String,

        pub operator: Operator,

        pub next_bus: [Option<NextBus>; 3],
    }

    impl From<RawArrivalBusService> for ArrivalBusService {
        fn from(data: RawArrivalBusService) -> Self {
            Self {
                service_no: data.service_no,
                operator: data.operator,
                next_bus: [data.next_bus, data.next_bus_2, data.next_bus_3],
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct NextBus {
        #[serde(deserialize_with = "from_str")]
        pub origin_code: u32,

        #[serde(deserialize_with = "from_str", alias = "DestinationCode")]
        pub dest_code: u32,

        /// Time in GMT+8
        #[serde(alias = "EstimatedArrival", deserialize_with = "iso8601::deserialize")]
        pub est_arrival: OffsetDateTime,

        #[serde(alias = "Latitude")]
        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub lat: f64,

        #[serde(alias = "Longitude")]
        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub long: f64,

        #[serde(deserialize_with = "from_str", alias = "VisitNumber")]
        pub visit_no: u32,

        pub load: BusLoad,

        pub feature: Option<BusFeature>,

        #[serde(alias = "Type")]
        pub bus_type: BusType,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct RawBusArrivalResp {
        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,
        pub services: Vec<RawArrivalBusService>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct BusArrivalResp {
        pub bus_stop_code: u32,
        pub services: Vec<ArrivalBusService>,
    }

    impl From<RawBusArrivalResp> for BusArrivalResp {
        fn from(data: RawBusArrivalResp) -> Self {
            Self {
                bus_stop_code: data.bus_stop_code,
                services: data.services.into_iter().map(|v| v.into()).collect(),
            }
        }
    }
}

pub mod bus_services {
    use crate::bus_enums::{BusCategory, Operator};
    use crate::utils::de::from_str_error_as_none;
    use crate::utils::regex::BUS_FREQ_RE;
    use serde::{Deserialize, Deserializer, Serialize};

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusServices";

    /// Both min and max are in terms of minutes
    #[derive(Debug, Clone, PartialEq, Serialize)]
    pub struct BusFreq {
        pub min: Option<u32>,
        pub max: Option<u32>,
    }

    impl BusFreq {
        pub fn new(min: u32, max: u32) -> Self {
            BusFreq {
                min: Some(min),
                max: Some(max),
            }
        }

        pub fn no_max(min: u32) -> Self {
            BusFreq {
                min: Some(min),
                max: None,
            }
        }

        pub fn no_timing() -> Self {
            BusFreq {
                min: None,
                max: None,
            }
        }
    }

    impl Default for BusFreq {
        fn default() -> Self {
            BusFreq::new(0, 0)
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct BusService {
        pub service_no: String,

        pub operator: Operator,

        #[serde(alias = "Direction")]
        pub no_direction: u32,

        pub category: BusCategory,

        #[serde(deserialize_with = "from_str_error_as_none")]
        pub origin_code: Option<u32>,

        #[serde(deserialize_with = "from_str_error_as_none", alias = "DestinationCode")]
        pub dest_code: Option<u32>,

        #[serde(alias = "AM_Peak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub am_peak_freq: BusFreq,

        #[serde(alias = "AM_Offpeak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub am_offpeak_freq: BusFreq,

        #[serde(alias = "PM_Peak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub pm_peak_freq: BusFreq,

        #[serde(alias = "PM_Offpeak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub pm_offpeak_freq: BusFreq,

        pub loop_desc: Option<String>,
    }

    fn from_str_to_bus_freq<'de, D>(deserializer: D) -> Result<BusFreq, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = String::deserialize(deserializer)?;

        let caps = BUS_FREQ_RE.captures(&s).unwrap();
        let min = caps.get(1).map_or(0, |m| m.as_str().parse().unwrap());
        let max = caps.get(2).map_or(0, |m| m.as_str().parse().unwrap());

        let bus_freq = if min == 0 && max == 0 {
            BusFreq::no_timing()
        } else if min != 0 && max == 0 {
            BusFreq::no_max(min)
        } else {
            BusFreq::new(min, max)
        };

        Ok(bus_freq)
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusServiceResp {
        pub value: Vec<BusService>,
    }

    impl From<BusServiceResp> for Vec<BusService> {
        fn from(data: BusServiceResp) -> Self {
            data.value
        }
    }
}
pub mod bus_routes {
    use serde::{Deserialize, Serialize};
    use time::Time;

    use crate::bus_enums::Operator;
    use crate::utils::de::from_str;
    use crate::utils::serde_date::str_time_option::{de_str_time_opt_br, ser_str_time_opt};

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusRoutes";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct BusRoute {
        pub service_no: String,

        pub operator: Operator,

        pub direction: u32,

        #[serde(alias = "StopSequence")]
        pub stop_seq: u32,

        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,

        #[serde(alias = "Distance")]
        pub dist: f64,

        #[serde(
            alias = "WD_FirstBus",
            deserialize_with = "de_str_time_opt_br",
            serialize_with = "ser_str_time_opt"
        )]
        pub wd_first: Option<Time>,

        #[serde(
            alias = "WD_LastBus",
            deserialize_with = "de_str_time_opt_br",
            serialize_with = "ser_str_time_opt"
        )]
        pub wd_last: Option<Time>,

        #[serde(
            alias = "SAT_FirstBus",
            deserialize_with = "de_str_time_opt_br",
            serialize_with = "ser_str_time_opt"
        )]
        pub sat_first: Option<Time>,

        #[serde(
            alias = "SAT_LastBus",
            deserialize_with = "de_str_time_opt_br",
            serialize_with = "ser_str_time_opt"
        )]
        pub sat_last: Option<Time>,

        #[serde(
            alias = "SUN_FirstBus",
            deserialize_with = "de_str_time_opt_br",
            serialize_with = "ser_str_time_opt"
        )]
        pub sun_first: Option<Time>,

        #[serde(
            alias = "SUN_LastBus",
            deserialize_with = "de_str_time_opt_br",
            serialize_with = "ser_str_time_opt"
        )]
        pub sun_last: Option<Time>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusRouteResp {
        pub value: Vec<BusRoute>,
    }

    impl From<BusRouteResp> for Vec<BusRoute> {
        fn from(data: BusRouteResp) -> Self {
            data.value
        }
    }
}
pub mod bus_stops {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusStops";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct BusStop {
        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,

        pub road_name: String,

        #[serde(alias = "Description")]
        pub desc: String,

        #[serde(alias = "Latitude")]
        pub lat: f64,

        #[serde(alias = "Longitude")]
        pub long: f64,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusStopsResp {
        pub value: Vec<BusStop>,
    }

    impl From<BusStopsResp> for Vec<BusStop> {
        fn from(data: BusStopsResp) -> Self {
            data.value
        }
    }
}
