pub mod prelude {
    pub use crate::facility::facilities_maintenance::*;
}

pub mod facilities_maintenance {
    use chrono::{DateTime, FixedOffset};

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct FacilityMaintenanceRawResp {
        pub value: Vec<FacilityLink>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct FacilityLink {
        pub link: String,
        pub timestamp: DateTime<FixedOffset>,
    }

    impl Into<Vec<FacilityLink>> for FacilityMaintenanceRawResp {
        fn into(self) -> Vec<FacilityLink> {
            self.value.into_iter().map(|v| v).collect()
        }
    }
}
