use serde::{Deserialize, Serialize};
        use schemars::JsonSchema;
        
#[doc = "Flavor"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"title\": \"Flavor\",\n  \"type\": [\n    \"object\"\n  ],\n  \"required\": [\n    \"nova_object.data\",\n    \"nova_object.name\",\n    \"nova_object.namespace\",\n    \"nova_object.version\"\n  ],\n  \"properties\": {\n    \"nova_object.changes\": {\n      \"type\": \"array\",\n      \"items\": {\n        \"type\": \"string\"\n      }\n    },\n    \"nova_object.data\": {\n      \"description\": \"fields of Flavor\",\n      \"type\": \"object\",\n      \"required\": [\n        \"deleted\",\n        \"disabled\",\n        \"ephemeral_gb\",\n        \"extra_specs\",\n        \"flavorid\",\n        \"id\",\n        \"is_public\",\n        \"memory_mb\",\n        \"projects\",\n        \"root_gb\",\n        \"swap\",\n        \"vcpus\"\n      ],\n      \"properties\": {\n        \"created_at\": {\n          \"type\": [\n            \"string\",\n            \"null\"\n          ],\n          \"format\": \"date-time\",\n          \"readonly\": false\n        },\n        \"deleted\": {\n          \"default\": false,\n          \"type\": [\n            \"boolean\"\n          ],\n          \"readonly\": false\n        },\n        \"deleted_at\": {\n          \"type\": [\n            \"string\",\n            \"null\"\n          ],\n          \"format\": \"date-time\",\n          \"readonly\": false\n        },\n        \"description\": {\n          \"type\": [\n            \"string\",\n            \"null\"\n          ],\n          \"readonly\": false\n        },\n        \"disabled\": {\n          \"type\": [\n            \"boolean\"\n          ],\n          \"readonly\": false\n        },\n        \"ephemeral_gb\": {\n          \"type\": [\n            \"integer\"\n          ],\n          \"readonly\": false\n        },\n        \"extra_specs\": {\n          \"type\": [\n            \"object\"\n          ],\n          \"additionalProperties\": {\n            \"type\": [\n              \"string\"\n            ],\n            \"readonly\": false\n          },\n          \"readonly\": false\n        },\n        \"flavorid\": {\n          \"type\": [\n            \"string\"\n          ],\n          \"readonly\": false\n        },\n        \"id\": {\n          \"type\": [\n            \"integer\"\n          ],\n          \"readonly\": false\n        },\n        \"is_public\": {\n          \"type\": [\n            \"boolean\"\n          ],\n          \"readonly\": false\n        },\n        \"memory_mb\": {\n          \"type\": [\n            \"integer\"\n          ],\n          \"readonly\": false\n        },\n        \"name\": {\n          \"type\": [\n            \"string\",\n            \"null\"\n          ],\n          \"readonly\": false\n        },\n        \"projects\": {\n          \"type\": [\n            \"array\"\n          ],\n          \"items\": {\n            \"type\": [\n              \"string\"\n            ],\n            \"readonly\": false\n          },\n          \"readonly\": false\n        },\n        \"root_gb\": {\n          \"type\": [\n            \"integer\"\n          ],\n          \"readonly\": false\n        },\n        \"rxtx_factor\": {\n          \"default\": 1.0,\n          \"type\": [\n            \"number\",\n            \"null\"\n          ],\n          \"readonly\": false\n        },\n        \"swap\": {\n          \"type\": [\n            \"integer\"\n          ],\n          \"readonly\": false\n        },\n        \"updated_at\": {\n          \"type\": [\n            \"string\",\n            \"null\"\n          ],\n          \"format\": \"date-time\",\n          \"readonly\": false\n        },\n        \"vcpu_weight\": {\n          \"type\": [\n            \"integer\",\n            \"null\"\n          ],\n          \"readonly\": false\n        },\n        \"vcpus\": {\n          \"type\": [\n            \"integer\"\n          ],\n          \"readonly\": false\n        }\n      }\n    },\n    \"nova_object.name\": {\n      \"type\": \"string\"\n    },\n    \"nova_object.namespace\": {\n      \"type\": \"string\"\n    },\n    \"nova_object.version\": {\n      \"type\": \"string\"\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum Flavor {
    Variant0 {
        #[serde(
            rename = "nova_object.changes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        nova_object_changes: Vec<String>,
        #[serde(rename = "nova_object.data")]
        nova_object_data: FlavorVariant0NovaObjectData,
        #[serde(rename = "nova_object.name")]
        nova_object_name: String,
        #[serde(rename = "nova_object.namespace")]
        nova_object_namespace: String,
        #[serde(rename = "nova_object.version")]
        nova_object_version: String,
    },
}
impl From<&Flavor> for Flavor {
    fn from(value: &Flavor) -> Self {
        value.clone()
    }
}
#[doc = "fields of Flavor"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"description\": \"fields of Flavor\",\n  \"type\": \"object\",\n  \"required\": [\n    \"deleted\",\n    \"disabled\",\n    \"ephemeral_gb\",\n    \"extra_specs\",\n    \"flavorid\",\n    \"id\",\n    \"is_public\",\n    \"memory_mb\",\n    \"projects\",\n    \"root_gb\",\n    \"swap\",\n    \"vcpus\"\n  ],\n  \"properties\": {\n    \"created_at\": {\n      \"type\": [\n        \"string\",\n        \"null\"\n      ],\n      \"format\": \"date-time\",\n      \"readonly\": false\n    },\n    \"deleted\": {\n      \"default\": false,\n      \"type\": [\n        \"boolean\"\n      ],\n      \"readonly\": false\n    },\n    \"deleted_at\": {\n      \"type\": [\n        \"string\",\n        \"null\"\n      ],\n      \"format\": \"date-time\",\n      \"readonly\": false\n    },\n    \"description\": {\n      \"type\": [\n        \"string\",\n        \"null\"\n      ],\n      \"readonly\": false\n    },\n    \"disabled\": {\n      \"type\": [\n        \"boolean\"\n      ],\n      \"readonly\": false\n    },\n    \"ephemeral_gb\": {\n      \"type\": [\n        \"integer\"\n      ],\n      \"readonly\": false\n    },\n    \"extra_specs\": {\n      \"type\": [\n        \"object\"\n      ],\n      \"additionalProperties\": {\n        \"type\": [\n          \"string\"\n        ],\n        \"readonly\": false\n      },\n      \"readonly\": false\n    },\n    \"flavorid\": {\n      \"type\": [\n        \"string\"\n      ],\n      \"readonly\": false\n    },\n    \"id\": {\n      \"type\": [\n        \"integer\"\n      ],\n      \"readonly\": false\n    },\n    \"is_public\": {\n      \"type\": [\n        \"boolean\"\n      ],\n      \"readonly\": false\n    },\n    \"memory_mb\": {\n      \"type\": [\n        \"integer\"\n      ],\n      \"readonly\": false\n    },\n    \"name\": {\n      \"type\": [\n        \"string\",\n        \"null\"\n      ],\n      \"readonly\": false\n    },\n    \"projects\": {\n      \"type\": [\n        \"array\"\n      ],\n      \"items\": {\n        \"type\": [\n          \"string\"\n        ],\n        \"readonly\": false\n      },\n      \"readonly\": false\n    },\n    \"root_gb\": {\n      \"type\": [\n        \"integer\"\n      ],\n      \"readonly\": false\n    },\n    \"rxtx_factor\": {\n      \"default\": 1.0,\n      \"type\": [\n        \"number\",\n        \"null\"\n      ],\n      \"readonly\": false\n    },\n    \"swap\": {\n      \"type\": [\n        \"integer\"\n      ],\n      \"readonly\": false\n    },\n    \"updated_at\": {\n      \"type\": [\n        \"string\",\n        \"null\"\n      ],\n      \"format\": \"date-time\",\n      \"readonly\": false\n    },\n    \"vcpu_weight\": {\n      \"type\": [\n        \"integer\",\n        \"null\"\n      ],\n      \"readonly\": false\n    },\n    \"vcpus\": {\n      \"type\": [\n        \"integer\"\n      ],\n      \"readonly\": false\n    }\n  }\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
pub struct FlavorVariant0NovaObjectData {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    pub deleted: FlavorVariant0NovaObjectDataDeleted,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deleted_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub disabled: FlavorVariant0NovaObjectDataDisabled,
    pub ephemeral_gb: FlavorVariant0NovaObjectDataEphemeralGb,
    pub extra_specs: FlavorVariant0NovaObjectDataExtraSpecs,
    pub flavorid: FlavorVariant0NovaObjectDataFlavorid,
    pub id: FlavorVariant0NovaObjectDataId,
    pub is_public: FlavorVariant0NovaObjectDataIsPublic,
    pub memory_mb: FlavorVariant0NovaObjectDataMemoryMb,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub projects: FlavorVariant0NovaObjectDataProjects,
    pub root_gb: FlavorVariant0NovaObjectDataRootGb,
    #[serde(default = "defaults::flavor_variant0_nova_object_data_rxtx_factor")]
    pub rxtx_factor: Option<f64>,
    pub swap: FlavorVariant0NovaObjectDataSwap,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<chrono::DateTime<chrono::offset::Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vcpu_weight: Option<i64>,
    pub vcpus: FlavorVariant0NovaObjectDataVcpus,
}
impl From<&FlavorVariant0NovaObjectData> for FlavorVariant0NovaObjectData {
    fn from(value: &FlavorVariant0NovaObjectData) -> Self {
        value.clone()
    }
}
#[doc = "FlavorVariant0NovaObjectDataDeleted"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"default\": false,\n  \"type\": [\n    \"boolean\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataDeleted {
    Variant0(bool),
}
impl From<&FlavorVariant0NovaObjectDataDeleted> for FlavorVariant0NovaObjectDataDeleted {
    fn from(value: &FlavorVariant0NovaObjectDataDeleted) -> Self {
        value.clone()
    }
}
impl Default for FlavorVariant0NovaObjectDataDeleted {
    fn default() -> Self {
        FlavorVariant0NovaObjectDataDeleted::Variant0(false)
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataDeleted {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataDeleted {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataDeleted {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataDeleted {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataDeleted {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<bool> for FlavorVariant0NovaObjectDataDeleted {
    fn from(value: bool) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataDisabled"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"boolean\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataDisabled {
    Variant0(bool),
}
impl From<&FlavorVariant0NovaObjectDataDisabled> for FlavorVariant0NovaObjectDataDisabled {
    fn from(value: &FlavorVariant0NovaObjectDataDisabled) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataDisabled {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataDisabled {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataDisabled {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataDisabled {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataDisabled {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<bool> for FlavorVariant0NovaObjectDataDisabled {
    fn from(value: bool) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataEphemeralGb"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"integer\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataEphemeralGb {
    Variant0(i64),
}
impl From<&FlavorVariant0NovaObjectDataEphemeralGb> for FlavorVariant0NovaObjectDataEphemeralGb {
    fn from(value: &FlavorVariant0NovaObjectDataEphemeralGb) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataEphemeralGb {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataEphemeralGb {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataEphemeralGb {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataEphemeralGb {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataEphemeralGb {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<i64> for FlavorVariant0NovaObjectDataEphemeralGb {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataExtraSpecs"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"object\"\n  ],\n  \"additionalProperties\": {\n    \"type\": [\n      \"string\"\n    ],\n    \"readonly\": false\n  },\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataExtraSpecs {
    Variant0(
        std::collections::HashMap<String, FlavorVariant0NovaObjectDataExtraSpecsVariant0Value>,
    ),
}
impl From<&FlavorVariant0NovaObjectDataExtraSpecs> for FlavorVariant0NovaObjectDataExtraSpecs {
    fn from(value: &FlavorVariant0NovaObjectDataExtraSpecs) -> Self {
        value.clone()
    }
}
impl From<std::collections::HashMap<String, FlavorVariant0NovaObjectDataExtraSpecsVariant0Value>>
    for FlavorVariant0NovaObjectDataExtraSpecs
{
    fn from(
        value: std::collections::HashMap<
            String,
            FlavorVariant0NovaObjectDataExtraSpecsVariant0Value,
        >,
    ) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataExtraSpecsVariant0Value"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"string\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataExtraSpecsVariant0Value {
    Variant0(String),
}
impl From<&FlavorVariant0NovaObjectDataExtraSpecsVariant0Value>
    for FlavorVariant0NovaObjectDataExtraSpecsVariant0Value
{
    fn from(value: &FlavorVariant0NovaObjectDataExtraSpecsVariant0Value) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataExtraSpecsVariant0Value {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataExtraSpecsVariant0Value {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataExtraSpecsVariant0Value {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataExtraSpecsVariant0Value {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataExtraSpecsVariant0Value {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
#[doc = "FlavorVariant0NovaObjectDataFlavorid"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"string\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataFlavorid {
    Variant0(String),
}
impl From<&FlavorVariant0NovaObjectDataFlavorid> for FlavorVariant0NovaObjectDataFlavorid {
    fn from(value: &FlavorVariant0NovaObjectDataFlavorid) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataFlavorid {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataFlavorid {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataFlavorid {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataFlavorid {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataFlavorid {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
#[doc = "FlavorVariant0NovaObjectDataId"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"integer\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataId {
    Variant0(i64),
}
impl From<&FlavorVariant0NovaObjectDataId> for FlavorVariant0NovaObjectDataId {
    fn from(value: &FlavorVariant0NovaObjectDataId) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataId {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataId {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataId {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataId {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataId {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<i64> for FlavorVariant0NovaObjectDataId {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataIsPublic"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"boolean\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataIsPublic {
    Variant0(bool),
}
impl From<&FlavorVariant0NovaObjectDataIsPublic> for FlavorVariant0NovaObjectDataIsPublic {
    fn from(value: &FlavorVariant0NovaObjectDataIsPublic) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataIsPublic {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataIsPublic {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataIsPublic {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataIsPublic {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataIsPublic {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<bool> for FlavorVariant0NovaObjectDataIsPublic {
    fn from(value: bool) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataMemoryMb"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"integer\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataMemoryMb {
    Variant0(i64),
}
impl From<&FlavorVariant0NovaObjectDataMemoryMb> for FlavorVariant0NovaObjectDataMemoryMb {
    fn from(value: &FlavorVariant0NovaObjectDataMemoryMb) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataMemoryMb {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataMemoryMb {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataMemoryMb {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataMemoryMb {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataMemoryMb {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<i64> for FlavorVariant0NovaObjectDataMemoryMb {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataProjects"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"array\"\n  ],\n  \"items\": {\n    \"type\": [\n      \"string\"\n    ],\n    \"readonly\": false\n  },\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataProjects {
    Variant0(Vec<FlavorVariant0NovaObjectDataProjectsVariant0Item>),
}
impl From<&FlavorVariant0NovaObjectDataProjects> for FlavorVariant0NovaObjectDataProjects {
    fn from(value: &FlavorVariant0NovaObjectDataProjects) -> Self {
        value.clone()
    }
}
impl From<Vec<FlavorVariant0NovaObjectDataProjectsVariant0Item>>
    for FlavorVariant0NovaObjectDataProjects
{
    fn from(value: Vec<FlavorVariant0NovaObjectDataProjectsVariant0Item>) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataProjectsVariant0Item"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"string\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataProjectsVariant0Item {
    Variant0(String),
}
impl From<&FlavorVariant0NovaObjectDataProjectsVariant0Item>
    for FlavorVariant0NovaObjectDataProjectsVariant0Item
{
    fn from(value: &FlavorVariant0NovaObjectDataProjectsVariant0Item) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataProjectsVariant0Item {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataProjectsVariant0Item {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataProjectsVariant0Item {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataProjectsVariant0Item {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataProjectsVariant0Item {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
#[doc = "FlavorVariant0NovaObjectDataRootGb"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"integer\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataRootGb {
    Variant0(i64),
}
impl From<&FlavorVariant0NovaObjectDataRootGb> for FlavorVariant0NovaObjectDataRootGb {
    fn from(value: &FlavorVariant0NovaObjectDataRootGb) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataRootGb {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataRootGb {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataRootGb {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataRootGb {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataRootGb {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<i64> for FlavorVariant0NovaObjectDataRootGb {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataSwap"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"integer\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataSwap {
    Variant0(i64),
}
impl From<&FlavorVariant0NovaObjectDataSwap> for FlavorVariant0NovaObjectDataSwap {
    fn from(value: &FlavorVariant0NovaObjectDataSwap) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataSwap {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataSwap {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataSwap {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataSwap {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataSwap {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<i64> for FlavorVariant0NovaObjectDataSwap {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
#[doc = "FlavorVariant0NovaObjectDataVcpus"]
#[doc = r""]
#[doc = r" <details><summary>JSON schema</summary>"]
#[doc = r""]
#[doc = r" ```json"]
#[doc = "{\n  \"type\": [\n    \"integer\"\n  ],\n  \"readonly\": false\n}"]
#[doc = r" ```"]
#[doc = r" </details>"]
#[derive(Clone, Debug, Deserialize, JsonSchema, Serialize)]
#[serde(untagged)]
pub enum FlavorVariant0NovaObjectDataVcpus {
    Variant0(i64),
}
impl From<&FlavorVariant0NovaObjectDataVcpus> for FlavorVariant0NovaObjectDataVcpus {
    fn from(value: &FlavorVariant0NovaObjectDataVcpus) -> Self {
        value.clone()
    }
}
impl std::str::FromStr for FlavorVariant0NovaObjectDataVcpus {
    type Err = &'static str;
    fn from_str(value: &str) -> Result<Self, &'static str> {
        if let Ok(v) = value.parse() {
            Ok(Self::Variant0(v))
        } else {
            Err("string conversion failed for all variants")
        }
    }
}
impl std::convert::TryFrom<&str> for FlavorVariant0NovaObjectDataVcpus {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<&String> for FlavorVariant0NovaObjectDataVcpus {
    type Error = &'static str;
    fn try_from(value: &String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl std::convert::TryFrom<String> for FlavorVariant0NovaObjectDataVcpus {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, &'static str> {
        value.parse()
    }
}
impl ToString for FlavorVariant0NovaObjectDataVcpus {
    fn to_string(&self) -> String {
        match self {
            Self::Variant0(x) => x.to_string(),
        }
    }
}
impl From<i64> for FlavorVariant0NovaObjectDataVcpus {
    fn from(value: i64) -> Self {
        Self::Variant0(value)
    }
}
pub mod defaults {
    pub(super) fn flavor_variant0_nova_object_data_rxtx_factor() -> Option<f64> {
        Some(1.0_f64)
    }
}
