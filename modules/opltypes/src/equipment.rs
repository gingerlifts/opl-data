//! Defines the Equipment field.

/// The Equipment field.
#[derive(Clone, Copy, Debug, Deserialize, EnumString, PartialEq, Serialize, PartialOrd)]
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

impl fmt::Display for Equipment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Equipment::Raw(n) => write!(f,"Raw"),
            Equipment::Wraps(n) => write!(f,"Wraps"),
            Equipment::Single(n) => write!(f,"Single-Ply"),
            Equipment::Multi(n) => write!(f,"Multi-Ply"),
            Equipment::Straps(n) => write!(f,"Straps"),
        }
}