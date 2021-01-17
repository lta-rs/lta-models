pub mod prelude {
    pub use crate::facility::facilities_maintenance::*;
}

pub mod facilities_maintenance {

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    pub struct FacilityMaintenanceRawResp {
        pub value: Vec<FacilityLink>,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
    #[serde(rename_all(deserialize = "PascalCase"))]
    pub struct FacilityLink {
        pub link: String,
    }

    impl Into<Vec<String>> for FacilityMaintenanceRawResp {
        fn into(self) -> Vec<String> {
            self.value.into_iter().map(|v| v.link).collect()
        }
    }
}
