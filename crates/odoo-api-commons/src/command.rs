use serde::{Deserialize, Serialize};

/// Ref: https://www.odoo.com/documentation/19.0/developer/reference/backend/orm.html#odoo.fields.Command
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Command<T> {
    Create { values: T },
    Update { id: u64, value: T },
    Delete { id: u64 },
    Unlink { id: u64 },
    Link { id: u64 },
    Clear,
    Set { ids: Vec<u64> },
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CommandRepr(u8, u64, serde_json::Value);

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
