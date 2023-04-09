//! Traffic structs and data structures

pub mod prelude {
    pub use {
        crate::traffic::bike_parking::{BikeParking, BikeParkingResp, RackType},
        crate::traffic::carpark_avail::{Agency, CarPark, CarparkAvailResp},
        crate::traffic::erp_rates::{DayType, ErpRate, ErpRatesResp, VehicleError},
        crate::traffic::est_travel_time::{
            EstTravelTime, EstTravelTimeResp, Highway, HighwayDirection,
        },
        crate::traffic::faulty_traffic_lights::{
            FaultyTrafficLight, FaultyTrafficLightResp, TechnicalAlarmType,
        },
        crate::traffic::road::{RoadDetails, RoadDetailsResp, RoadDetailsType},
        crate::traffic::traffic_flow::{TrafficFlowLink, TrafficFlowRawResp},
        crate::traffic::traffic_images::{TrafficImage, TrafficImageResp},
        crate::traffic::traffic_incidents::{IncidentType, TrafficIncident, TrafficIncidentResp},
        crate::traffic::traffic_speed_bands::{
            RoadCategory, TrafficSpeedBand, TrafficSpeedBandResp,
        },
        crate::traffic::vms_emas::{VMSResp, Vms},
    };
}

pub mod erp_rates {
    use core::fmt;
    use serde::{Deserialize, Serialize};
    use std::fmt::Formatter;
    use std::str::FromStr;
    use time::{Date, Time};

    use crate::utils::{
        de::{delimited, Sep},
        serde_date::{
            str_date,
            str_time_option::{de_str_time_opt_erp, ser_str_time_opt},
        },
    };

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/ERPRates";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum VehicleType {
        #[serde(alias = "Passenger Cars")]
        PassengerCars,

        #[serde(alias = "Motorcycles")]
        Motorcycles,

        #[serde(alias = "Light Goods Vehicles")]
        LightGoodsVehicles,

        #[serde(alias = "Heavy Goods Vehicles")]
        HeavyGoodsVehicles,

        #[serde(alias = "Very Heavy Goods Vehicles")]
        VeryHeavyGoodsVehicles,

        #[serde(alias = "Taxis")]
        Taxis,

        #[serde(alias = "Big Buses")]
        BigBuses,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct VehicleError;

    impl fmt::Display for VehicleError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "VehicleErr")
        }
    }

    impl FromStr for VehicleType {
        type Err = VehicleError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let res = match s {
                "Passenger Cars" => VehicleType::PassengerCars,
                "Motorcycles" => VehicleType::Motorcycles,
                "Light Goods Vehicles" => VehicleType::LightGoodsVehicles,
                "Heavy Goods Vehicles" => VehicleType::HeavyGoodsVehicles,
                "Very Heavy Goods Vehicles" => VehicleType::VeryHeavyGoodsVehicles,
                "Taxis" => VehicleType::Taxis,
                "Big Buses" => VehicleType::BigBuses,
                _ => VehicleType::Unknown,
            };

            Ok(res)
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum DayType {
        Weekdays,
        Saturday,
    }

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum ZoneId {
        CT2,
        PE4,
        AY1,
        AYC,
        AYT,
        BKE,
        BKZ,
        BMC,
        CBD,
        OC1,
        CT1,
        PE1,
        CT4,
        PE2,
        THM,
        OR1,
        PE3,
        DZ1,
        CT5,
        OC2,
        OC3,
        KP2,
        CT6,
        UBT,
        TPZ,
        KBZ,
        GBZ,
        SR2,
        SR1,
        KAL,
        EC3,
        MC1,
        MC2,

        #[serde(other)]
        Unknown,
    }

    impl Sep for VehicleType {
        fn delimiter() -> &'static str {
            "/"
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct ErpRateRaw {
        #[serde(deserialize_with = "delimited")]
        pub vehicle_type: Vec<VehicleType>,

        pub day_type: DayType,

        #[serde(
            deserialize_with = "de_str_time_opt_erp",
            serialize_with = "ser_str_time_opt"
        )]
        pub start_time: Option<Time>,

        #[serde(
            deserialize_with = "de_str_time_opt_erp",
            serialize_with = "ser_str_time_opt"
        )]
        pub end_time: Option<Time>,

        #[serde(alias = "ZoneID")]
        pub zone_id: ZoneId,

        #[serde(alias = "ChargeAmount")]
        pub charge_amt: f32,

        #[serde(with = "str_date")]
        pub effective_date: Date,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct ErpRate {
        pub vehicle_type: Vec<VehicleType>,
        pub day_type: DayType,
        pub start_time: Option<Time>,
        pub end_time: Option<Time>,
        pub zone_id: ZoneId,
        pub charge_amt: f32,
        pub effective_date: Date,
    }

    impl From<ErpRateRaw> for ErpRate {
        #[inline(always)]
        fn from(r: ErpRateRaw) -> Self {
            Self {
                vehicle_type: r.vehicle_type,
                day_type: r.day_type,
                start_time: r.start_time,
                end_time: r.end_time,
                zone_id: r.zone_id,
                charge_amt: r.charge_amt,
                effective_date: r.effective_date,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct ErpRatesResp {
        pub value: Vec<ErpRateRaw>,
    }

    impl From<ErpRatesResp> for Vec<ErpRate> {
        fn from(data: ErpRatesResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}

pub mod carpark_avail {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str_to_coords;
    use crate::utils::Coordinates;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/CarParkAvailabilityv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum LotType {
        C,
        L,
        Y,
        H,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[allow(clippy::upper_case_acronyms)]
    pub enum Agency {
        HDB,
        URA,
        LTA,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct CarParkRaw {
        #[serde(rename = "CarParkID")]
        pub carpark_id: String,

        pub area: String,

        #[serde(rename = "Development")]
        pub dev: String,

        #[serde(alias = "Location", deserialize_with = "from_str_to_coords")]
        pub coords: Option<Coordinates>,

        #[serde(alias = "AvailableLots")]
        pub avail_lots: u32,

        pub lot_type: LotType,

        pub agency: Agency,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct CarPark {
        pub carpark_id: String,
        pub area: String,
        pub dev: String,
        pub coords: Option<Coordinates>,
        pub avail_lots: u32,
        pub lot_type: LotType,
        pub agency: Agency,
    }

    impl From<CarParkRaw> for CarPark {
        #[inline(always)]
        fn from(r: CarParkRaw) -> Self {
            Self {
                carpark_id: r.carpark_id,
                area: r.area,
                dev: r.dev,
                coords: r.coords,
                avail_lots: r.avail_lots,
                lot_type: r.lot_type,
                agency: r.agency,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct CarparkAvailResp {
        pub value: Vec<CarParkRaw>,
    }

    impl From<CarparkAvailResp> for Vec<CarPark> {
        fn from(data: CarparkAvailResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}

pub mod est_travel_time {
    use serde::{Deserialize, Serialize};
    use serde_repr::*;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/EstTravelTimes";

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum Highway {
        PIE,
        AYE,
        NSC,
        ECP,
        CTE,
        TPE,
        KPE,
        SLE,
        BKE,
        KJE,
        MCE,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(u32)]
    pub enum HighwayDirection {
        EastToWest = 1,
        WestToEast = 2,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct EstTravelTimeRaw {
        pub name: Highway,

        pub direction: HighwayDirection,

        #[serde(alias = "FarEndPoint")]
        pub far_end_pt: String,

        #[serde(alias = "StartPoint")]
        pub start_pt: String,

        #[serde(alias = "EndPoint")]
        pub end_pt: String,

        #[serde(alias = "EstTime")]
        pub est_travel_time: u32,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct EstTravelTime {
        pub name: Highway,
        pub direction: HighwayDirection,
        pub far_end_pt: String,
        pub start_pt: String,
        pub end_pt: String,
        pub est_travel_time: u32,
    }

    impl From<EstTravelTimeRaw> for EstTravelTime {
        fn from(r: EstTravelTimeRaw) -> Self {
            Self {
                name: r.name,
                direction: r.direction,
                far_end_pt: r.far_end_pt,
                start_pt: r.start_pt,
                end_pt: r.end_pt,
                est_travel_time: r.est_travel_time
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct EstTravelTimeResp {
        pub value: Vec<EstTravelTimeRaw>,
    }

    impl From<EstTravelTimeResp> for Vec<EstTravelTime> {
        fn from(data: EstTravelTimeResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}

pub mod faulty_traffic_lights {
    use serde::{Deserialize, Serialize};
    use time::OffsetDateTime;

    use crate::utils::serde_date::ymd_hms_option;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/FaultyTrafficLights";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum TechnicalAlarmType {
        Blackout = 4,
        FlashingYellow = 13,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct FaultyTrafficLight {
        #[serde(alias = "AlarmID")]
        pub alarm_id: String,

        #[serde(alias = "NodeID")]
        pub node_id: String,

        #[serde(alias = "Type")]
        pub technical_alarm_type: TechnicalAlarmType,

        #[serde(with = "ymd_hms_option")]
        pub start_date: Option<OffsetDateTime>,

        #[serde(with = "ymd_hms_option")]
        pub end_date: Option<OffsetDateTime>,

        pub message: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct FaultyTrafficLightResp {
        pub value: Vec<FaultyTrafficLight>,
    }

    impl From<FaultyTrafficLightResp> for Vec<FaultyTrafficLight> {
        fn from(data: FaultyTrafficLightResp) -> Self {
            data.value
        }
    }
}

pub mod road {
    use serde::{Deserialize, Serialize};
    use time::Date;

    use crate::utils::serde_date::str_date;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL_ROAD_OPENING: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/RoadOpenings";

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL_ROAD_WORKS: &str = "http://datamall2.mytransport.sg/ltaodataservice/RoadWorks";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum RoadDetailsType {
        RoadOpening,
        RoadWorks,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct RoadDetails {
        #[serde(alias = "EventID")]
        pub event_id: String,

        #[serde(with = "str_date")]
        pub start_date: Date,

        #[serde(with = "str_date")]
        pub end_date: Date,

        #[serde(alias = "SvcDept")]
        pub service_dept: String,

        pub road_name: String,

        pub other: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct RoadDetailsResp {
        pub value: Vec<RoadDetails>,
    }

    impl From<RoadDetailsResp> for Vec<RoadDetails> {
        fn from(data: RoadDetailsResp) -> Self {
            data.value
        }
    }
}

pub mod traffic_images {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/Traffic-Imagesv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficImageRaw {
        #[serde(alias = "CameraID", deserialize_with = "from_str")]
        pub camera_id: u32,

        #[serde(alias = "Latitude")]
        pub lat: f64,

        #[serde(alias = "Longitude")]
        pub long: f64,

        #[serde(alias = "ImageLink")]
        pub image_link: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficImage {
        pub camera_id: u32,
        pub lat: f64,
        pub long: f64,
        pub image_link: String,
    }

    impl From<TrafficImageRaw> for TrafficImage {
        #[inline(always)]
        fn from(r: TrafficImageRaw) -> Self {
            Self {
                camera_id: r.camera_id,
                lat: r.lat,
                long: r.long,
                image_link: r.image_link,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficImageResp {
        pub value: Vec<TrafficImageRaw>,
    }

    impl From<TrafficImageResp> for Vec<TrafficImage> {
        fn from(data: TrafficImageResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}

pub mod traffic_incidents {
    use serde::{Deserialize, Serialize};

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/TrafficIncidents";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum IncidentType {
        Accident,

        #[serde(alias = "Road Works")]
        RoadWorks,

        #[serde(alias = "Vehicle breakdown")]
        VehicleBreakdown,

        Weather,

        Obstacle,

        #[serde(alias = "Road Block")]
        RoadBlock,

        #[serde(alias = "Heavy Traffic")]
        HeavyTraffic,

        #[serde(alias = "Misc.")]
        Misc,

        Diversion,

        #[serde(alias = "Unattended Vehicle")]
        UnattendedVehicle,

        Roadwork,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficIncident {
        #[serde(alias = "Type")]
        pub incident_type: IncidentType,

        #[serde(alias = "Latitude")]
        pub lat: f64,

        #[serde(alias = "Longitude")]
        pub long: f64,

        #[serde(alias = "Message")]
        pub msg: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficIncidentResp {
        pub value: Vec<TrafficIncident>,
    }

    impl From<TrafficIncidentResp> for Vec<TrafficIncident> {
        fn from(data: TrafficIncidentResp) -> Self {
            data.value
        }
    }
}

pub mod traffic_speed_bands {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str;

    #[cfg(feature = "fastfloat")]
    use crate::utils::de::from_str_fast_float;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/TrafficSpeedBandsv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum RoadCategory {
        #[serde(alias = "A")]
        Expressway,

        #[serde(alias = "B")]
        MajorArterialRoads,

        #[serde(alias = "C")]
        ArterialRoads,

        #[serde(alias = "D")]
        MinorArterialRoads,

        #[serde(alias = "E")]
        SmallRoads,

        #[serde(alias = "F")]
        SlipRoads,

        #[serde(alias = "G")]
        NoCategoryInfoAvail,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrafficSpeedBandRaw {
        #[serde(alias = "LinkID", deserialize_with = "from_str")]
        pub link_id: u64,

        pub road_name: String,

        pub road_category: RoadCategory,

        pub speed_band: u32,

        #[serde(alias = "MinimumSpeed", deserialize_with = "from_str")]
        pub min_speed: u32,

        #[serde(alias = "MaximumSpeed", deserialize_with = "from_str")]
        pub max_speed: u32,

        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub start_lon: f64,

        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub start_lat: f64,

        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub end_lon: f64,

        #[cfg_attr(not(feature = "fastfloat"), serde(deserialize_with = "from_str"))]
        #[cfg_attr(feature = "fastfloat", serde(deserialize_with = "from_str_fast_float"))]
        pub end_lat: f64,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficSpeedBand {
        pub link_id: u64,
        pub road_name: String,
        pub road_category: RoadCategory,
        pub speed_band: u32,
        pub min_speed: u32,
        pub max_speed: u32,
        pub start_lon: f64,
        pub start_lat: f64,
        pub end_lon: f64,
        pub end_lat: f64,
    }

    impl From<TrafficSpeedBandRaw> for TrafficSpeedBand {
        #[inline(always)]
        fn from(r: TrafficSpeedBandRaw) -> Self {
            Self {
                link_id: r.link_id,
                road_name: r.road_name,
                road_category: r.road_category,
                speed_band: r.speed_band,
                min_speed: r.min_speed,
                max_speed: r.max_speed,
                start_lon: r.start_lon,
                start_lat: r.start_lat,
                end_lon: r.end_lon,
                end_lat: r.end_lat,
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficSpeedBandResp {
        pub value: Vec<TrafficSpeedBandRaw>,
    }

    impl From<TrafficSpeedBandResp> for Vec<TrafficSpeedBand> {
        fn from(data: TrafficSpeedBandResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}

pub mod vms_emas {
    use serde::{Deserialize, Serialize};

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/VMS";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct Vms {
        #[serde(alias = "EquipmentID")]
        pub equipment_id: String,

        #[serde(alias = "Latitude")]
        pub lat: f64,

        #[serde(alias = "Longitude")]
        pub long: f64,

        #[serde(alias = "Message")]
        pub msg: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct VMSResp {
        pub value: Vec<Vms>,
    }

    impl From<VMSResp> for Vec<Vms> {
        fn from(data: VMSResp) -> Self {
            data.value
        }
    }
}

pub mod bike_parking {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str_to_bool;

    #[deprecated(since = "0.5.0", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/BicycleParkingv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum RackType {
        #[serde(alias = "Yellow Box")]
        YellowBox,

        #[serde(alias = "Yellow Box_Private")]
        YellowBoxPrivate,

        #[serde(alias = "Racks_MRT")]
        RacksMRT,

        #[serde(alias = "Racks_Bus Stop")]
        RacksBusStop,

        #[serde(alias = "Racks_URA")]
        RacksURA,

        #[serde(alias = "Racks_AVA")]
        RacksAVA,

        #[serde(alias = "Racks_ITE")]
        RacksITE,

        #[serde(alias = "Racks_JTC")]
        RacksJTC,

        #[serde(alias = "Racks_PA")]
        RacksPA,

        #[serde(alias = "Racks_NParks")]
        RacksNParks,

        #[serde(alias = "Racks_HDB")]
        RacksHDB,

        #[serde(alias = "Racks_NLB")]
        RacksNLB,

        #[serde(alias = "Racks_NEA")]
        RacksNEA,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct BikeParkingRaw {
        #[serde(rename = "Description")]
        pub desc: String,

        #[serde(rename = "Latitude")]
        pub lat: f64,

        #[serde(rename = "Longitude")]
        pub long: f64,

        pub rack_type: RackType,

        pub rack_count: u32,

        #[serde(deserialize_with = "from_str_to_bool")]
        pub shelter_indicator: bool,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BikeParking {
        pub desc: String,
        pub lat: f64,
        pub long: f64,
        pub rack_type: RackType,
        pub rack_count: u32,
        pub shelter_indicator: bool,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BikeParkingResp {
        pub value: Vec<BikeParkingRaw>,
    }

    impl From<BikeParkingRaw> for BikeParking {
        fn from(r: BikeParkingRaw) -> Self {
            Self {
                desc: r.desc,
                lat: r.lat,
                long: r.long,
                rack_type: r.rack_type,
                rack_count: r.rack_count,
                shelter_indicator: r.shelter_indicator,
            }
        }
    }

    impl From<BikeParkingResp> for Vec<BikeParking> {
        fn from(data: BikeParkingResp) -> Self {
            data.value.into_iter().map(Into::into).collect()
        }
    }
}

pub mod traffic_flow {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct TrafficFlowLink {
        link: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficFlowRawResp {
        pub value: Vec<TrafficFlowLink>,
    }

    impl From<TrafficFlowRawResp> for Vec<String> {
        fn from(data: TrafficFlowRawResp) -> Self {
            data.value.into_iter().map(|v| v.link).collect()
        }
    }
}
