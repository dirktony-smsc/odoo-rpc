use serde::{Deserialize, Serialize};

pub mod sales_order;

#[derive(Debug, Clone)]
pub struct Many2OneRepr {
    pub id: u64,
    pub name: String,
}

impl<'de> Deserialize<'de> for Many2OneRepr {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (id, name) = <(u64, String)>::deserialize(deserializer)?;
        Ok(Self { id, name })
    }
}

impl Serialize for Many2OneRepr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (self.id, &self.name).serialize(serializer)
    }
}
