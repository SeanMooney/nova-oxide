pub mod flavor;

// a struct representing a gen::flavor::Flavor::Variant0
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema, Default)]
pub struct FlavorVariant0 {
    #[serde(
        rename = "nova_object.changes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub nova_object_changes: Vec<String>,
    #[serde(rename = "nova_object.data")]
    pub nova_object_data: flavor::FlavorVariant0NovaObjectData,
    #[serde(rename = "nova_object.name")]
    pub  nova_object_name: String,
    #[serde(rename = "nova_object.namespace")]
    pub nova_object_namespace: String,
    #[serde(rename = "nova_object.version")]
    pub nova_object_version: String,
}

impl flavor::Flavor {
    // a convert into trait implemation for  gen::flavor::Flavor -> FlavorVariant0
    pub fn into(self) -> FlavorVariant0 {
        match self {
            flavor::Flavor::Variant0 { nova_object_changes, nova_object_data, nova_object_name, nova_object_namespace, nova_object_version } => {
                FlavorVariant0 {
                    nova_object_changes,
                    nova_object_data,
                    nova_object_name,
                    nova_object_namespace,
                    nova_object_version,
                }
            }
        }
    }

}

impl Default for flavor::FlavorVariant0NovaObjectData {
    fn default() -> Self {
        Self {
            id: 0.into(),
            name: None,
            memory_mb: 0.into(),
            vcpus: 0.into(),
            root_gb: 0.into(),
            ephemeral_gb: 0.into(),
            flavorid: flavor::FlavorVariant0NovaObjectDataFlavorid::Variant0("".to_string()),
            swap: 0.into(),
            rxtx_factor: Some(1.0),
            vcpu_weight: Some(1),
            disabled: false.into(),
            is_public: true.into(),
            extra_specs: std::collections::HashMap::new().into(),
            projects: vec![].into(),
            description: None,
            created_at: None,
            updated_at: None,
            deleted_at: None,
            deleted: false.into(),
        }
    }
}