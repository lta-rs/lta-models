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

    use crate::utils::{
        de::{delimited, Sep},
        serde_date::{
            str_date,
            str_time_option::{de_str_time_opt_erp, ser_str_time_opt},
        },
    };
    use chrono::{NaiveDate, NaiveTime};

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
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
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct ErpRate {
        #[serde(deserialize_with = "delimited")]
        pub vehicle_type: Vec<VehicleType>,

        pub day_type: DayType,

        #[serde(
            deserialize_with = "de_str_time_opt_erp",
            serialize_with = "ser_str_time_opt"
        )]
        pub start_time: Option<NaiveTime>,

        #[serde(
            deserialize_with = "de_str_time_opt_erp",
            serialize_with = "ser_str_time_opt"
        )]
        pub end_time: Option<NaiveTime>,

        #[serde(alias = "ZoneID")]
        pub zone_id: ZoneId,

        #[serde(alias = "ChargeAmount")]
        pub charge_amt: f32,

        #[serde(with = "str_date")]
        pub effective_date: NaiveDate,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct ErpRatesResp {
        pub value: Vec<ErpRate>,
    }

    impl From<ErpRatesResp> for Vec<ErpRate> {
        fn from(data: ErpRatesResp) -> Self {
            data.value
        }
    }
}

pub mod carpark_avail {
    use serde::{Deserialize, Serialize};

    use crate::utils::de::from_str_to_coords;
    use crate::utils::Coordinates;

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
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
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct CarPark {
        #[serde(alias = "CarParkID")]
        pub carpark_id: String,

        pub area: String,

        #[serde(alias = "Development")]
        pub dev: String,

        #[serde(alias = "Location", deserialize_with = "from_str_to_coords")]
        pub coords: Option<Coordinates>,

        #[serde(alias = "AvailableLots")]
        pub avail_lots: u32,

        pub lot_type: LotType,

        pub agency: Agency,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct CarparkAvailResp {
        pub value: Vec<CarPark>,
    }

    impl From<CarparkAvailResp> for Vec<CarPark> {
        fn from(data: CarparkAvailResp) -> Self {
            data.value
        }
    }
}

pub mod est_travel_time {
    use serde::{Deserialize, Serialize};
    use serde_repr::*;

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
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
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct EstTravelTime {
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
    pub struct EstTravelTimeResp {
        pub value: Vec<EstTravelTime>,
    }

    impl From<EstTravelTimeResp> for Vec<EstTravelTime> {
        fn from(data: EstTravelTimeResp) -> Self {
            data.value
        }
    }
}

pub mod faulty_traffic_lights {
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::serde_date::ymd_hms_option;

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/FaultyTrafficLights";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum TechnicalAlarmType {
        Blackout = 4,
        FlashingYellow = 13,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct FaultyTrafficLight {
        #[serde(alias = "AlarmID")]
        pub alarm_id: String,

        #[serde(alias = "NodeID")]
        pub node_id: String,

        #[serde(alias = "Type")]
        pub technical_alarm_type: TechnicalAlarmType,

        #[serde(with = "ymd_hms_option")]
        pub start_date: Option<DateTime<Utc>>,

        #[serde(with = "ymd_hms_option")]
        pub end_date: Option<DateTime<Utc>>,

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
    use chrono::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::utils::serde_date::str_date;

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_ROAD_OPENING: &str =
        "http://datamall2.mytransport.sg/ltaodataservice/RoadOpenings";
        
    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL_ROAD_WORKS: &str = "http://datamall2.mytransport.sg/ltaodataservice/RoadWorks";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum RoadDetailsType {
        RoadOpening,
        RoadWorks,
        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct RoadDetails {
        #[serde(alias = "EventID")]
        pub event_id: String,

        #[serde(with = "str_date")]
        pub start_date: NaiveDate,

        #[serde(with = "str_date")]
        pub end_date: NaiveDate,

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

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/Traffic-Imagesv2";

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficImage {
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
    pub struct TrafficImageResp {
        pub value: Vec<TrafficImage>,
    }

    impl From<TrafficImageResp> for Vec<TrafficImage> {
        fn from(data: TrafficImageResp) -> Self {
            data.value
        }
    }
}

pub mod traffic_incidents {
    use serde::{Deserialize, Serialize};

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
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

    use crate::utils::de::{from_str, from_str_loc_to_loc};
    use crate::utils::Location;

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
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
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct TrafficSpeedBand {
        #[serde(alias = "LinkID", deserialize_with = "from_str")]
        pub link_id: u64,

        pub road_name: String,

        pub road_category: RoadCategory,

        pub speed_band: u32,

        #[serde(alias = "MinimumSpeed", deserialize_with = "from_str")]
        pub min_speed: u32,

        #[serde(alias = "MaximumSpeed", deserialize_with = "from_str")]
        pub max_speed: u32,

        #[serde(alias = "Location", deserialize_with = "from_str_loc_to_loc")]
        pub coord_start_end: Option<Location>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrafficSpeedBandResp {
        pub value: Vec<TrafficSpeedBand>,
    }

    impl From<TrafficSpeedBandResp> for Vec<TrafficSpeedBand> {
        fn from(data: TrafficSpeedBandResp) -> Self {
            data.value
        }
    }
}

pub mod vms_emas {
    use serde::{Deserialize, Serialize};

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
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

    #[deprecated(since = "0.5", note = "Will be removed in future versions")]
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
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct BikeParking {
        #[serde(alias = "Description")]
        pub desc: String,

        #[serde(alias = "Latitude")]
        pub lat: f64,

        #[serde(alias = "Longitude")]
        pub long: f64,

        pub rack_type: RackType,

        pub rack_count: u32,

        #[serde(deserialize_with = "from_str_to_bool")]
        pub shelter_indicator: bool,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct BikeParkingResp {
        pub value: Vec<BikeParking>,
    }

    impl From<BikeParkingResp> for Vec<BikeParking> {
        fn from(data: BikeParkingResp) -> Self {
            data.value
        }
    }
}
