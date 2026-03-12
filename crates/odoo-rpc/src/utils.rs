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

impl<T> From<NumOrVec<T>> for Vec<T> {
    fn from(value: NumOrVec<T>) -> Self {
        match value.0 {
            Either::Left(v) => vec![v],
            Either::Right(vs) => vs,
        }
    }
}

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

/// Ref: https://www.odoo.com/documentation/18.0/developer/reference/backend/orm.html#odoo.fields.Command
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Command<T> {
    Create { values: Vec<T> },
    Update { id: u64, value: T },
    Delete { id: u64 },
    Unlink { id: u64 },
    Link { id: u64 },
    Clear,
    Set { ids: Vec<u64> },
}

#[derive(Serialize)]
struct CommandRepr(u8, u64, serde_json::Value);

impl<T> Serialize for Command<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let command_repr: CommandRepr = match self {
            Command::Create { values } => CommandRepr(
                0,
                0,
                values
                    .serialize(serde_json::value::Serializer)
                    .map_err(serde::ser::Error::custom)?,
            ),
            Command::Update { id, value } => CommandRepr(
                1,
                *id,
                value
                    .serialize(serde_json::value::Serializer)
                    .map_err(serde::ser::Error::custom)?,
            ),
            Command::Delete { id } => CommandRepr(2, *id, 0u8.into()),
            Command::Unlink { id } => CommandRepr(3, *id, 0u8.into()),
            Command::Link { id } => CommandRepr(4, *id, 0u8.into()),
            Command::Clear => CommandRepr(5, 0, 0u8.into()),
            Command::Set { ids } => CommandRepr(
                6,
                0,
                ids.serialize(serde_json::value::Serializer)
                    .map_err(serde::ser::Error::custom)?,
            ),
        };
        command_repr.serialize(serializer)
    }
}
