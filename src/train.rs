//! Train structs and data structures

pub mod prelude {
    pub use crate::train::train_service_alert::{
        AffectedSegment, MrtLine, TrainServiceAlert, TrainServiceAlertMessage,
        TrainServiceAlertResp, TrainStatus,
    };
    pub use crate::train::StationCode;
}

/// List obtained from [Wikipedia](https://en.wikipedia.org/wiki/List_of_Singapore_MRT_stations)
/// and [Wikipedia](https://en.wikipedia.org/wiki/List_of_Singapore_LRT_stations)
/// Some of the stations are commented out to prevent misuse as they are technically
/// not constructed yet or it has not been announced
#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum StationCode {
    NS1,
    NS2,
    NS3,
    NS4,
    NS5,
    // NS6,
    NS7,
    NS8,
    NS9,
    NS10,
    NS11,
    NS12,
    NS13,
    NS14,
    NS15,
    NS16,
    NS17,
    NS18,
    NS19,
    NS20,
    NS21,
    NS22,
    NS23,
    NS24,
    NS25,
    NS26,
    NS27,
    NS28,
    EW1,
    EW2,
    EW3,
    EW4,
    EW5,
    EW6,
    EW7,
    EW8,
    EW9,
    EW10,
    EW11,
    EW12,
    EW13,
    EW14,
    EW15,
    EW16,
    EW17,
    EW18,
    EW19,
    EW20,
    EW21,
    EW22,
    EW23,
    EW24,
    EW25,
    EW26,
    EW27,
    EW28,
    EW29,
    EW30,
    EW31,
    EW32,
    EW33,
    CG,
    CG1,
    CG2,
    NE1,
    // NE2, reserved for possible future MRT station
    NE3,
    NE4,
    NE5,
    NE6,
    NE7,
    NE8,
    NE9,
    NE10,
    NE11,
    NE12,
    NE13,
    NE14,
    NE15,
    NE16,
    NE17,
    CC1,
    CC2,
    CC3,
    CC4,
    CC5,
    CC6,
    CC7,
    CC8,
    CC9,
    CC10,
    CC11,
    CC12,
    CC13,
    CC14,
    CC15,
    CC16,
    CC17,
    //CC18, bukit brown, not constructed yet
    CC19,
    CC20,
    CC21,
    CC22,
    CC23,
    CC24,
    CC25,
    CC26,
    CC27,
    CC28,
    CC29,
    // CC30, keppel
    // CC31, cantonment
    // CC32, prince edward road
    CC33,
    CC34,
    CE1,
    CE2,
    DT1,
    DT2,
    DT3,
    // DT4, hume
    DT5,
    DT6,
    DT7,
    DT8,
    DT9,
    DT10,
    DT11,
    DT12,
    DT13,
    DT14,
    DT15,
    DT16,
    DT17,
    DT18,
    DT19,
    DT20,
    DT21,
    DT22,
    DT23,
    DT24,
    DT25,
    DT26,
    DT27,
    DT28,
    DT29,
    DT30,
    DT31,
    DT32,
    DT33,
    DT34,
    DT35,
    // DT36, Xilin
    // DT37, Sungei Bedok
    TE1,
    TE2,
    TE3,
    // TE4, Springleaf
    // TE5, Lentor
    // TE6, Mayflower
    // TE7, Bright Hill
    // TE8, Upper Thomson
    // TE9, Caldecott
    // TE10, Mount Pleasant
    // TE11, Stevens
    // TE12, Napier
    // TE13, Orchard Boulevard
    // TE14, Orchard
    // TE15, Great World
    // TE16, Havelock
    // TE17, Outram Park
    // TE18, Maxwell
    // TE19, Shenton Way
    // TE20, Marina Bay
    // TE21, Marina South
    // TE22, Gardens By The Bay
    // TE22A, Founders' Memorial
    // TE23, Tanjong Rhu
    // TE24, Katong Park
    // TE25, Tanjong Katong
    // TE26, Marine Parade
    // TE27, Marine Terrace
    // TE28, Siglap
    // TE29, Bayshore
    // TE30, Bedok South
    // TE31, Sungei Bedok
    BP1,
    BP2,
    BP3,
    BP4,
    BP5,
    BP6,
    BP7,
    BP8,
    BP9,
    BP10,
    BP11,
    BP12,
    BP13,
    BP14,
    STC,
    SE1,
    SE2,
    SE3,
    SE4,
    SE5,
    SW1,
    SW2,
    SW3,
    SW4,
    SW5,
    SW6,
    SW7,
    SW8,
    PTC,
    PE1,
    PE2,
    PE3,
    PE4,
    PE5,
    PE6,
    PE7,
    PW1,
    PW2,
    PW3,
    PW4,
    PW5,
    PW6,
    PW7,
    #[serde(other)]
    Unknown,
}

pub mod train_service_alert {
    use serde::{Deserialize, Serialize};
    use serde_repr::*;

    use crate::utils::de::{delimited, Sep, WrapErr};
    use std::ops::Deref;
    use std::str::FromStr;

    pub const URL: &str = "http://datamall2.mytransport.sg/ltaodataservice/TrainServiceAlerts";

    #[allow(clippy::upper_case_acronyms)]
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub enum MrtLine {
        CCL,
        CEL,
        CGL,
        DTL,
        EWL,
        NEL,
        NSL,
        PEL,
        PWL,
        SEL,
        SWL,
        BPL,

        #[serde(other)]
        Unknown,
    }

    #[derive(Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
    #[repr(u32)]
    pub enum TrainStatus {
        Normal = 1,
        Disrupted = 2,

        #[serde(other)]
        Unknown,
    }

    impl Sep for StringWrap {
        fn delimiter() -> &'static str {
            "-"
        }
    }

    impl Deref for StringWrap {
        type Target = String;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl FromStr for StringWrap {
        type Err = WrapErr;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            Ok(StringWrap(s.to_string()))
        }
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct StringWrap(String);

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct AffectedSegment {
        pub line: MrtLine,

        pub direction: String,

        #[serde(deserialize_with = "delimited")]
        pub stations: Vec<StringWrap>,

        pub free_public_bus: String,

        pub free_mrt_shuttle: String,

        #[serde(alias = "MRTShuttleDirection")]
        pub mrt_shuttle_dir: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct TrainServiceAlertMessage {
        pub content: String,
        pub created_date: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct TrainServiceAlert {
        pub status: TrainStatus,

        pub affected_segments: Vec<AffectedSegment>,

        pub message: Vec<TrainServiceAlertMessage>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct TrainServiceAlertResp {
        pub value: TrainServiceAlert,
    }

    impl From<TrainServiceAlertResp> for TrainServiceAlert {
        fn from(data: TrainServiceAlertResp) -> Self {
            data.value
        }
    }
}
