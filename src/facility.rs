//! Facility structs and data structures

pub mod prelude {
    pub use crate::facility::facilities_maintenance::*;
}

pub mod facilities_maintenance {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct FacilityMaintenanceRawResp {
        pub value: Vec<FacilityLink>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct FacilityLink {
        pub link: String,
    }

    impl From<FacilityMaintenanceRawResp> for Vec<String> {
        fn from(data: FacilityMaintenanceRawResp) -> Self {
            data.value.into_iter().map(|v| v.link).collect()
        }
    }
}
