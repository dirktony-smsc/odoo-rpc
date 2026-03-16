pub mod operators;

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Domain {
    Condition(String, String, serde_json::Value),
    #[serde(rename = "&")]
    And,
    #[serde(rename = "|")]
    Or,
    #[serde(rename = "!")]
    Not,
}

impl Domain {
    pub fn new<A, B, C>(a: A, b: B, c: C) -> Domain
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<serde_json::Value>,
    {
        Self::Condition(a.into(), b.into(), c.into())
    }
}

#[cfg(test)]
mod tests {
    use crate::Domain;

    #[test]
    fn test_default() {}
}
