use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct ContextConfig {
    pub name: String,
    pub features: Vec<FeatureConfig>,
    pub segments: Vec<SegmentConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct FeatureConfig {
    pub name: String,
    pub description: String,
    pub treatments: Vec<TreatmentConfig>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct TreatmentConfig {
    pub probability: i8,
    pub segments: Vec<String>,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SegmentConfig {
    pub name: String,
    pub user_identifiers: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct TogglesResponse {
    pub toggles: Vec<ToggleResponse>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ToggleResponse {
    pub name: String,
    pub value: String,
}
