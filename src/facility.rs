use chrono::{DateTime, FixedOffset};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct FacilityMaintenanceRawResp {
    pub value: Vec<FacilityLink>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "PascalCase"))]
pub struct FacilityLink {
    pub link: String,
    pub timestamp: DateTime<FixedOffset>
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum FacilityId {
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
    WordMarking
}