use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawApiStatus {
    pub message: String,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ApiStatus {
    #[default]
    Unknown,
    Running,
}
impl From<RawApiStatus> for ApiStatus {
    fn from(value: RawApiStatus) -> Self {
        let res = match value.message.as_str() {
            "OK" => Self::Running,
            _ => Self::Unknown,
        };
        res
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RawStatus {
    pub message: String,
}
