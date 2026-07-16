pub mod account_account;
pub mod account_journal;
pub mod account_move;
pub mod account_move_line;
pub mod account_tax;
pub mod crm_lead;
pub mod crm_stage;
pub mod hr_employee;
pub mod product_template;
pub mod resource_resource;
pub mod sales_order;
pub mod sales_order_line;

use serde::{Deserialize, Serialize};
use struct_field_names_as_array::FieldNamesAsSlice;

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

#[derive(Debug, Clone, Deserialize, FieldNamesAsSlice)]
pub struct IdNameRepr {
    pub id: u64,
    pub name: String,
}
