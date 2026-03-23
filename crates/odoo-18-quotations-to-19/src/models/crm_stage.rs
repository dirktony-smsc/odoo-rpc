use derive_more::derive::Display;
use serde::{Deserialize, Serialize};

#[derive(
    Debug, Clone, Copy, Deserialize, Serialize, PartialEq, Eq, PartialOrd, Ord, Hash, Display,
)]
#[serde(rename_all = "snake_case")]
pub enum CrmStagePriorities {
    #[display("Low")]
    #[serde(rename = "0")]
    P0,
    #[display("Medium")]
    #[serde(rename = "1")]
    P1,
    #[display("High")]
    #[serde(rename = "2")]
    P2,
    #[display("Very High")]
    #[serde(rename = "3")]
    P3,
}
