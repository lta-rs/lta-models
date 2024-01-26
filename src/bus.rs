//! Bus structs and data structures

pub mod prelude {
    pub use {
        crate::bus::bus_arrival::{BusArrivalResp, NextBus, BusArrivalRespRaw},
        crate::bus::bus_routes::{BusRoute, BusRouteResp},
        crate::bus::bus_services::{BusFreq, BusService, BusServiceResp},
        crate::bus::bus_stops::{BusStop, BusStopsResp},
    };
}

pub mod bus_arrival {
    use serde::{Deserialize, Serialize};
    use time::{serde::iso8601, serde::timestamp, OffsetDateTime};

    use crate::bus_enums::{BusFeature, BusLoad, BusType, Operator};
    use crate::utils::de::{from_str, treat_error_as_none};

    #[cfg(feature = "fastfloat")]
    use crate::utils::de::from_str_fast_float;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusArrivalv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct RawArrivalBusService {
        pub service_no: String,

        pub operator: Operator,

        #[serde(deserialize_with = "treat_error_as_none")]
        pub next_bus: Option<NextBusRaw>,

        #[serde(deserialize_with = "treat_error_as_none")]
        pub next_bus_2: Option<NextBusRaw>,

        #[serde(deserialize_with = "treat_error_as_none")]
        pub next_bus_3: Option<NextBusRaw>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
                next_bus: [
                    data.next_bus.map(Into::into),
                    data.next_bus_2.map(Into::into),
                    data.next_bus_3.map(Into::into),
                ],
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct NextBusRaw {
        #[serde(deserialize_with = "from_str")]
        pub origin_code: u32,

        #[serde(deserialize_with = "from_str", rename = "DestinationCode")]
        pub dest_code: u32,

        /// Time in GMT+8
        #[serde(
            rename = "EstimatedArrival",
            deserialize_with = "iso8601::deserialize",
            serialize_with = "iso8601::serialize"
        )]
        pub est_arrival: OffsetDateTime,

        #[serde(rename = "Latitude")]
        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub long: f64,

        #[serde(deserialize_with = "from_str", rename = "VisitNumber")]
        pub visit_no: u8,

        pub load: BusLoad,

        pub feature: BusFeature,

        #[serde(alias = "Type")]
        pub bus_type: BusType,
    }

    #[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize, Serialize)]
    pub struct NextBus {
        pub origin_code: u32,
        pub dest_code: u32,
        /// Time in GMT+8
        #[serde(with = "timestamp")]
        pub est_arrival: OffsetDateTime,
        pub lat: f64,
        pub long: f64,
        pub visit_no: u8,
        pub load: BusLoad,
        pub feature: BusFeature,
        pub bus_type: BusType,
    }

    impl From<NextBusRaw> for NextBus {
        #[inline(always)]
        fn from(r: NextBusRaw) -> Self {
            Self {
                origin_code: r.origin_code,
                dest_code: r.dest_code,
                est_arrival: r.est_arrival,
                lat: r.lat,
                long: r.long,
                visit_no: r.visit_no,
                load: r.load,
                feature: r.feature,
                bus_type: r.bus_type,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusArrivalRespRaw {
        #[serde(deserialize_with = "from_str")]
        pub bus_stop_code: u32,
        pub services: Vec<RawArrivalBusService>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusArrivalResp {
        pub bus_stop_code: u32,
        pub services: Vec<ArrivalBusService>,
    }

    impl From<BusArrivalRespRaw> for BusArrivalResp {
        fn from(data: BusArrivalRespRaw) -> Self {
            Self {
                bus_stop_code: data.bus_stop_code,
                services: data.services.into_iter().map(|v| v.into()).collect(),
            }
        }
    }
}

pub mod bus_services {
    use std::num::NonZeroU8;

    use crate::bus_enums::{BusCategory, Operator};
    use crate::utils::de::from_str_error_as_none;
    use crate::utils::regex::BUS_FREQ_RE;
    use serde::{Deserialize, Deserializer, Serialize};

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusServices";

    /// Both min and max are in terms of minutes
    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    pub struct BusFreq {
        pub min: Option<NonZeroU8>,
        pub max: Option<NonZeroU8>,
    }

    impl BusFreq {
        pub fn new(min: u8, max: u8) -> Self {
            BusFreq {
                min: Some(NonZeroU8::new(min).unwrap()),
                max: Some(NonZeroU8::new(max).unwrap()),
            }
        }

        pub fn no_max(min: u8) -> Self {
            BusFreq {
                min: Some(NonZeroU8::new(min).unwrap()),
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
    #[serde(rename_all = "PascalCase")]
    pub struct BusServiceRaw {
        pub service_no: String,

        pub operator: Operator,

        #[serde(alias = "Direction")]
        pub no_direction: u8,

        pub category: BusCategory,

        #[serde(deserialize_with = "from_str_error_as_none")]
        pub origin_code: Option<NonZeroU8>,

        #[serde(deserialize_with = "from_str_error_as_none", alias = "DestinationCode")]
        pub dest_code: Option<NonZeroU8>,

        #[serde(alias = "AM_Peak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub am_peak_freq: BusFreq,

        #[serde(alias = "AM_Offpeak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub am_offpeak_freq: BusFreq,

        #[serde(alias = "PM_Peak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub pm_peak_freq: BusFreq,

        #[serde(alias = "PM_Offpeak_Freq", deserialize_with = "from_str_to_bus_freq")]
        pub pm_offpeak_freq: BusFreq,

        pub loop_desc: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusService {
        pub service_no: String,
        pub operator: Operator,
        pub no_direction: u8,
        pub category: BusCategory,
        pub origin_code: Option<NonZeroU8>,
        pub dest_code: Option<NonZeroU8>,
        pub am_peak_freq: BusFreq,
        pub am_offpeak_freq: BusFreq,
        pub pm_peak_freq: BusFreq,
        pub pm_offpeak_freq: BusFreq,
        pub loop_desc: String,
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

    impl From<BusServiceRaw> for BusService {
        fn from(r: BusServiceRaw) -> Self {
            Self {
                service_no: r.service_no,
                operator: r.operator,
                no_direction: r.no_direction,
                category: r.category,
                origin_code: r.origin_code,
                dest_code: r.dest_code,
                am_offpeak_freq: r.am_offpeak_freq,
                am_peak_freq: r.am_peak_freq,
                pm_offpeak_freq: r.pm_offpeak_freq,
                pm_peak_freq: r.pm_peak_freq,
                loop_desc: r.loop_desc,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusServiceResp {
        pub value: Vec<BusServiceRaw>,
    }

    impl From<BusServiceResp> for Vec<BusService> {
        fn from(data: BusServiceResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
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
    #[serde(rename_all = "PascalCase")]
    pub struct BusRouteRaw {
        pub service_no: String,

        pub operator: Operator,

        pub direction: u8,

        #[serde(alias = "StopSequence")]
        pub stop_seq: u8,

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
    pub struct BusRoute {
        pub service_no: String,
        pub operator: Operator,
        pub direction: u8,
        pub stop_seq: u8,
        pub bus_stop_code: u32,
        pub dist: f64,
        pub wd_first: Option<Time>,
        pub wd_last: Option<Time>,
        pub sat_first: Option<Time>,
        pub sat_last: Option<Time>,
        pub sun_first: Option<Time>,
        pub sun_last: Option<Time>,
    }

    impl From<BusRouteRaw> for BusRoute {
        #[inline(always)]
        fn from(r: BusRouteRaw) -> Self {
            Self {
                service_no: r.service_no,
                operator: r.operator,
                direction: r.direction,
                stop_seq: r.stop_seq,
                bus_stop_code: r.bus_stop_code,
                dist: r.dist,
                wd_first: r.wd_first,
                wd_last: r.wd_last,
                sat_first: r.sat_first,
                sat_last: r.sat_last,
                sun_first: r.sun_first,
                sun_last: r.sun_last,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusRouteResp {
        pub value: Vec<BusRouteRaw>,
    }

    impl From<BusRouteResp> for Vec<BusRoute> {
        fn from(data: BusRouteResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}
pub mod bus_stops {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BusStops";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BusStopRaw {
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
    pub struct BusStop {
        pub bus_stop_code: u32,
        pub road_name: String,
        pub desc: String,
        pub lat: f64,
        pub long: f64,
    }

    impl From<BusStopRaw> for BusStop {
        #[inline(always)]
        fn from(r: BusStopRaw) -> Self {
            Self {
                bus_stop_code: r.bus_stop_code,
                road_name: r.road_name,
                desc: r.desc,
                lat: r.lat,
                long: r.long,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BusStopsResp {
        pub value: Vec<BusStopRaw>,
    }

    impl From<BusStopsResp> for Vec<BusStop> {
        fn from(data: BusStopsResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}
