pub mod command;
pub mod domain;
pub mod maybe_vec;
pub mod pagination;

pub use command::Command;
pub use domain::Domain;
pub use maybe_vec::MaybeVec;
pub use pagination::PaginationParam;

use either::Either;
use serde::{Deserialize, Deserializer};

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
