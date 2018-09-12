//! Defines the Equipment field.

/// The Equipment field.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
pub enum Equipment {
    Raw,
    Wraps,
    #[serde(rename(deserialize = "Single-ply"))]
    #[strum(serialize = "Single-ply")]
    Single,
    #[serde(rename(deserialize = "Multi-ply"))]
    #[strum(serialize = "Multi-ply")]
    Multi,
    Straps,
}

#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
pub enum SquatEquipment {
    Raw,
    Wraps,
    #[serde(rename(deserialize = "Single-ply"))]
    #[strum(serialize = "Single-ply")]
    Single,
    #[serde(rename(deserialize = "Multi-ply"))]
    #[strum(serialize = "Multi-ply")]
    Multi,
}

#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
pub enum BenchEquipment {
    Raw,
    #[serde(rename(deserialize = "Single-ply"))]
    #[strum(serialize = "Single-ply")]
    Single,
    #[serde(rename(deserialize = "Multi-ply"))]
    #[strum(serialize = "Multi-ply")]
    Multi,
}

#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize)]
pub enum DeadliftEquipment {
    Raw,
    #[serde(rename(deserialize = "Single-ply"))]
    #[strum(serialize = "Single-ply")]
    Single,
    #[serde(rename(deserialize = "Multi-ply"))]
    #[strum(serialize = "Multi-ply")]
    Multi,
    Straps,
}