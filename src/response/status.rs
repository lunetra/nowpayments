use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Serialize, Deserialize)]
pub struct RawStatus {
    pub message: String,
}

#[derive(Default, Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub enum Status {
    #[default]
    Unknown,
    Dummy,
    Waiting,
    Confirming,
    Confirmed,
    Sending,
    PartiallyPaid,
    Finished,
    Failed,
    Refunded,
    Expired,
}
impl FromStr for Status {
    type Err = std::io::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let res = match value {
            "waiting" => Self::Waiting,
            "confirming" => Self::Confirming,
            "confirmed" => Self::Confirmed,
            "sending" => Self::Sending,
            "partially_paid" => Self::PartiallyPaid,
            "finished" => Self::Finished,
            "failed" => Self::Failed,
            "refunded" => Self::Refunded,
            "expired" => Self::Expired,
            "dummy" => Self::Dummy,
            _ => Self::Unknown,
        };
        Ok(res)
    }
}
