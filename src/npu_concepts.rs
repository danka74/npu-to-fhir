use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Npubegreber {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    #[serde(rename = "Npubegreb")]
    pub npubegreb: Vec<Npubegreb>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Npubegreb {
    #[serde(rename = "$text")]
    pub text: Option<String>,
    pub created_date: String,
    pub change_date: String,
    pub change_comment: String,
    pub npu_code: String,
    pub short_definition: String,
    pub system_short: String,
    pub sys_spec_short: String,
    pub component_short: String,
    pub comp_spec_short: String,
    pub kind_of_property_short: String,
    pub proc_short: String,
    pub unit_short: String,
    pub full_definition: String,
    pub system: String,
    pub sys_spec: String,
    pub component: String,
    pub comp_spec: String,
    pub kind_of_property: String,
    pub proc: String,
    pub unit: String,
    pub specialty: String,
    pub context_dependent: String,
    pub group: String,
    pub scale_type: String,
    pub replaces: String,
    pub replaced_by: String,
    pub effective_from: String,
    pub effective_to: String,
    pub active: String,
    pub current_version: String,
}

