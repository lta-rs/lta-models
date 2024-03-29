//! Geospatial structs and data structures

pub mod prelude {
    pub use crate::geo::geospatial_whole_island::*;
}

pub mod geospatial_whole_island {
    use std::default;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct GeospatialWholeIslandRawResp {
        pub value: Vec<GeospatialLink>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all = "PascalCase")]
    pub struct GeospatialLink {
        pub link: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize, Default)]
    pub enum GeospatialLayerId {
        ArrowMarking,
        Bollard,
        BusStopLocation,
        ControlBox,
        ConvexMirror,
        CoveredLinkWay,
        CyclingPath,
        CyclingPathConstruction,
        DetectorLoop,
        EmergencyGate,
        ERPGantry,
        Footpath,
        GuardRail,
        KerbLine,
        LampPost,
        LaneMarking,
        ParkingStandardsZone,
        PassengerPickupBay,
        #[serde(alias = "PedestrainOverheadbridge_UnderPass")]
        PedestrianOverheadBridgeOrUnderPass,
        RailConstruction,
        Railing,
        RetainingWall,
        RoadConstruction,
        RoadCrossing,
        RoadHump,
        RoadSectionLine,
        SchoolZone,
        SilverZone,
        SpeedRegulatingStrip,
        StreetPaint,
        TaxiStand,
        TrafficLight,
        TrafficSign,
        TrainStation,
        TrainStationExit,
        #[serde(alias = "VehicularBridge_Flyover_Underpass")]
        VehicularBridgeOrFlyoverOrUnderpass,
        WordMarking,

        #[default]
        #[serde(other)]
        Unknown
    }

    impl From<GeospatialWholeIslandRawResp> for Vec<String> {
        fn from(data: GeospatialWholeIslandRawResp) -> Self {
            data.value.into_iter().map(|v| v.link).collect()
        }
    }
}
