//! Data Structures for lta-rs and for LTA datamall APIs

// Forbid warnings in release builds:
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_flatten,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]

pub mod bus;
pub mod bus_enums;
pub mod crowd;
pub mod facility;
pub mod geo;
pub mod taxi;
pub mod traffic;
pub mod train;
pub mod utils;

/// Data structures for all data
pub mod prelude {
    pub use {
        crate::bus::prelude::*, crate::bus_enums::prelude::*, crate::crowd::prelude::*,
        crate::facility::prelude::*, crate::geo::prelude::*, crate::taxi::prelude::*,
        crate::traffic::prelude::*, crate::train::prelude::*,
    };
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use serde::{de::DeserializeOwned, Deserialize, Serialize};
    use std::fmt::Debug;
    use time::{macros::datetime, OffsetDateTime};

    fn generate_test<'de, I, S, F>(input_fn: F) -> (String, Vec<u8>, S, S)
    where
        F: FnOnce() -> &'de str,
        I: Deserialize<'de> + Into<S>,
        S: DeserializeOwned + Serialize + Debug,
    {
        let data = input_fn();
        let de: S = serde_json::from_str::<I>(data).map(|f| f.into()).unwrap();
        let ser_json = serde_json::to_string(&de).unwrap();

        // round trip check
        let de_2 = serde_json::from_str::<S>(&ser_json).unwrap();

        let ser_json_new = serde_json::to_string(&de_2).unwrap();
        let ser_bincode = bincode::serialize(&de).unwrap();

        println!("{}", ser_json_new);
        // println!("{:X?}", ser_bincode);

        let de_bincode = bincode::deserialize::<S>(&ser_bincode[..]).unwrap();

        (ser_json_new, ser_bincode, de, de_bincode)
    }

    macro_rules! gen_test {
        ($a:ty, $b:ty, $c:expr) => {
            generate_test::<$a, $b, _>(|| include_str!($c))
        };
    }

    #[test]
    fn bike_parking() {
        gen_test!(
            BikeParkingResp,
            Vec<BikeParking>,
            "../dumped_data/bike_parking.json"
        );
    }

    #[test]
    fn test_bincode() {
        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct LelRaw {
            inner: Inner,
            str_bool: String,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Lel {
            inner: Inner,
            str_bool: bool,
        }

        #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
        enum Inner {
            #[serde(rename = "x")]
            A,

            #[serde(rename = "y")]
            B,

            #[serde(rename = "z")]
            C,

            #[serde(other)]
            Unknown,
        }

        let json = r#"{"Inner":"x","StrBool":"Y"}"#;
        let lel_raw = serde_json::from_str::<LelRaw>(json).unwrap();
        let lel_str = serde_json::to_string(&lel_raw).unwrap();
        assert_eq!(json, lel_str.as_str());

        let str_bool = lel_raw.str_bool == "Y";

        let lel = Lel {
            inner: lel_raw.inner.clone(),
            str_bool,
        };

        let bc_buf = bincode::serialize(&lel_raw).unwrap();
        let lel2 = bincode::deserialize::<Lel>(&bc_buf[..]).unwrap();
        assert_eq!(lel, lel2);
    }

    #[test]
    fn test_bc_nextbus() {
        let sample_data = NextBus {
            origin_code: 77009,
            dest_code: 77009,
            est_arrival: datetime!(2023-04-06 14:47:57 +8),
            lat: 1.314452,
            long: 103.910009,
            visit_no: 1,
            load: BusLoad::SeatsAvailable,
            feature: BusFeature::WheelChairAccessible,
            bus_type: BusType::SingleDecker,
        };

        let ser = bincode::serialize(&sample_data).unwrap();
        let de = bincode::deserialize::<NextBus>(&ser[..]).unwrap();
        assert_eq!(de, sample_data);
    }

    #[test]
    fn test_bc_datetime() {
        use time::serde::iso8601;

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Dt {
            #[serde(
                rename = "EstimatedArrival",
                deserialize_with = "iso8601::deserialize",
                serialize_with = "iso8601::serialize"
            )]
            inner: OffsetDateTime,
        }

        let dt = Dt {
            inner: datetime!(2023-04-06 14:47:57 +8),
        };

        let ser = bincode::serialize(&dt).unwrap();
        let de = bincode::deserialize::<Dt>(&ser[..]).unwrap();
        assert_eq!(de, dt);
    }

    #[test]
    fn test_json_datetime() {
        use time::serde::timestamp;

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        pub struct Dt {
            #[serde(rename = "EstimatedArrival", with = "timestamp")]
            inner: OffsetDateTime,
        }

        let dt = Dt {
            inner: datetime!(2023-04-06 14:47:57 +8),
        };

        let ser = serde_json::to_string(&dt).unwrap();
        println!("{}", ser);
        let de = serde_json::from_str::<Dt>(&ser).unwrap();
        assert_eq!(de, dt);
    }

    #[test]
    fn bus_arrival() {
        let (_, _, bus, bus_bincode) = gen_test!(
            RawBusArrivalResp,
            BusArrivalResp,
            "../dumped_data/bus_arrival.json"
        );

        assert_eq!(bus.bus_stop_code, 83139);
        assert_eq!(bus.services.len(), 3);
        assert_eq!(bus.services[0].operator, Operator::Gas);
        assert_eq!(bus.services[1].operator, Operator::Sbst);
        assert_eq!(bus.services[2].operator, Operator::Sbst);

        let sample_data = NextBus {
            origin_code: 77009,
            dest_code: 77009,
            est_arrival: datetime!(2023-04-06 14:47:57 +8),
            lat: 1.314452,
            long: 103.910009,
            visit_no: 1,
            load: BusLoad::SeatsAvailable,
            feature: BusFeature::WheelChairAccessible,
            bus_type: BusType::SingleDecker,
        };

        assert_eq!(bus.services[0].next_bus[0], Some(sample_data));
        assert_eq!(bus, bus_bincode);
        println!("NextBus: {}", std::mem::size_of::<NextBus>());
        println!("BusLoad: {}", std::mem::size_of::<BusLoad>());
        println!("BusFeature: {}", std::mem::size_of::<BusFeature>());
        println!("OffsetDateTime: {}", std::mem::size_of::<OffsetDateTime>());
    }

    #[test]
    fn bus_routes() {
        gen_test!(BusRouteResp, Vec<BusRoute>, "../dumped_data/bus_route.json");
    }

    #[test]
    fn bus_service() {
        gen_test!(
            BusServiceResp,
            Vec<BusService>,
            "../dumped_data/bus_services.json"
        );

        println!("Sz BusService: {}", std::mem::size_of::<BusService>());
        println!("Sz BusFreq: {}", std::mem::size_of::<BusFreq>());
    }

    #[test]
    fn bus_stops() {
        gen_test!(BusStopsResp, Vec<BusStop>, "../dumped_data/bus_stops.json");
    }

    #[test]
    fn carpark_avail() {
        gen_test!(
            CarparkAvailResp,
            Vec<CarPark>,
            "../dumped_data/carpark_avail.json"
        );
    }

    #[test]
    fn erp_rates() {
        gen_test!(ErpRatesResp, Vec<ErpRate>, "../dumped_data/erp_rates.json");
    }

    #[test]
    fn faulty_traffic_lights() {
        gen_test!(
            FaultyTrafficLightResp,
            Vec<FaultyTrafficLight>,
            "../dumped_data/faulty_traffic_lights.json"
        );
    }

    #[test]
    fn passenger_vol_bus_stops() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_bus_stops.json"
        );
    }

    #[test]
    fn passenger_vol_od_bus_stops() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_od_bus_stops.json"
        );
    }

    #[test]
    fn passenger_vol_od_train() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_od_train.json"
        );
    }

    #[test]
    fn passenger_vol_train() {
        gen_test!(
            PassengerVolRawResp,
            Vec<String>,
            "../dumped_data/passenger_vol_train.json"
        );
    }

    #[test]
    fn traffic_speed_bands() {
        gen_test!(
            TrafficSpeedBandResp,
            Vec<TrafficSpeedBand>,
            "../dumped_data/traffic_speed_bands.json"
        );
    }

    #[test]
    fn vms_emas() {
        gen_test!(VMSResp, Vec<Vms>, "../dumped_data/vms_emas.json");
    }

    #[test]
    fn taxi_avail() {
        gen_test!(
            TaxiAvailResp,
            Vec<Coordinates>,
            "../dumped_data/taxi_avail.json"
        );
    }

    #[test]
    fn taxi_stands() {
        gen_test!(
            TaxiStandsResp,
            Vec<TaxiStand>,
            "../dumped_data/taxi_stands.json"
        );
    }

    #[test]
    fn train_service_alert() {
        gen_test!(
            TrainServiceAlertResp,
            TrainServiceAlert,
            "../dumped_data/train_service_alert.json"
        );
    }

    #[test]
    fn est_travel_time() {
        gen_test!(
            EstTravelTimeResp,
            Vec<EstTravelTime>,
            "../dumped_data/est_travel_time.json"
        );
    }

    #[test]
    fn crowd_density_rt() {
        gen_test!(
            StationCrowdLevelRawResp,
            Vec<StationCrowdLevel>,
            "../dumped_data/crowd_density_rt.json"
        );
    }

    #[test]
    fn crowd_density_forecast() {
        // not in snake_case
        gen_test!(
            CrowdDensityForecastRawResp,
            CrowdDensityForecast,
            "../dumped_data/crowd_density_forecast.json"
        );
    }

    #[test]
    fn road_works() {
        // not in snake case
        gen_test!(
            RoadDetailsResp,
            Vec<RoadDetails>,
            "../dumped_data/road_works.json"
        );
    }

    #[test]
    fn geospatial_whole_island() {
        // not in snake_case
        gen_test!(
            GeospatialWholeIslandRawResp,
            Vec<String>,
            "../dumped_data/geospatial_whole_island.json"
        );
    }

    #[test]
    fn traffic_flow() {
        // not in snake_case
        gen_test!(
            TrafficFlowRawResp,
            Vec<String>,
            "../dumped_data/traffic_flow.json"
        );
    }

    #[test]
    fn traffic_images() {
        // not in snake_case
        gen_test!(
            TrafficImageResp,
            Vec<TrafficImage>,
            "../dumped_data/traffic_images.json"
        );
    }

    #[test]
    fn traffic_incidents() {
        // not in snake_case
        gen_test!(
            TrafficIncidentResp,
            Vec<TrafficIncident>,
            "../dumped_data/traffic_incidents.json"
        );
    }

    #[test]
    fn facilities_maintenance() {
        // not in snake_case
        gen_test!(
            FacilityMaintenanceRawResp,
            Vec<String>,
            "../dumped_data/facilities_maintainence.json"
        );
    }
}
