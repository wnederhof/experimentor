use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Clone)]
pub struct Context {
    pub name: String,
    pub features: Vec<Feature>,
    pub segments: Vec<Segment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Feature {
    pub name: String,
    pub description: String,
    pub treatments: Vec<Treatment>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Treatment {
    pub probability: i8,
    pub segments: Vec<String>,
    pub value: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Segment {
    pub name: String,
    pub user_identifiers: Vec<String>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Toggles {
    pub toggles: Vec<Toggle>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Toggle {
    name: String,
    value: String,
}
