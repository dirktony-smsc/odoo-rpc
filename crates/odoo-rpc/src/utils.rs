pub mod fields_get;
pub mod version;

use either::Either;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct PaginationParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Domain(pub String, pub String, pub serde_json::Value);

impl Domain {
    pub fn new<A, B, C>(a: A, b: B, c: C) -> Domain
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<serde_json::Value>,
    {
        Self(a.into(), b.into(), c.into())
    }
}

pub fn deserialize_and_default_if_false<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de> + Default,
{
    let val: Either<bool, T> = either::serde_untagged::deserialize(deserializer)?;
    match val {
        Either::Left(_) => Ok(Default::default()),
        Either::Right(t) => Ok(t),
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NumOrVec<T>(pub Either<T, Vec<T>>);

impl<T> Serialize for NumOrVec<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        either::serde_untagged::serialize(&self.0, serializer)
    }
}

impl<'de, T> Deserialize<'de> for NumOrVec<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        Ok(Self(either::serde_untagged::deserialize(deserializer)?))
    }
}
