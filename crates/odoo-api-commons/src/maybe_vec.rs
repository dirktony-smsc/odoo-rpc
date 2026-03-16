use either::Either;
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Debug, Clone)]
pub struct MaybeVec<T>(pub Either<T, Vec<T>>);

impl<T> From<MaybeVec<T>> for Vec<T> {
    fn from(value: MaybeVec<T>) -> Self {
        match value.0 {
            Either::Left(v) => vec![v],
            Either::Right(vs) => vs,
        }
    }
}

impl<T> Serialize for MaybeVec<T>
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

impl<'de, T> Deserialize<'de> for MaybeVec<T>
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
