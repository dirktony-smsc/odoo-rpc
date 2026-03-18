pub mod fields_get;
pub mod version;

use either::Either;
pub(crate) use odoo_api_commons::{Domain, MaybeVec, PaginationParam};
use serde::Deserialize;

pub(crate) struct MaybeSomething<T>(pub T);

impl<'de, T> Deserialize<'de> for MaybeSomething<T>
where
    T: Deserialize<'de> + Default,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let inner: Either<T, bool> = either::serde_untagged::deserialize(deserializer)?;
        match inner {
            Either::Left(v) => Ok(Self(v)),
            Either::Right(_) => Ok(Self(Default::default())),
        }
    }
}
