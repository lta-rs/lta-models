pub mod prelude {
    pub use crate::geo::geospatial_whole_island::*;
}

pub mod geospatial_whole_island {
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct GeospatialWholeIslandRawResp {
        pub value: Vec<GeospatialLink>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct GeospatialLink {
        pub link: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
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
    }

    impl Into<Vec<String>> for GeospatialWholeIslandRawResp {
        fn into(self) -> Vec<String> {
            self.value.into_iter().map(|v| v.link).collect()
        }
    }
}