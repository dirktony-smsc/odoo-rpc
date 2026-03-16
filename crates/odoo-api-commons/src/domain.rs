pub mod operators;

use serde::{
    Deserialize, Serialize,
    de::{self, Visitor},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Domain {
    Condition(String, String, serde_json::Value),
    /// Represent "&" with 2 domains arity
    And,
    /// Represent "|" with 2 domains arity
    Or,
    /// Represent "!" with only 1 domain arity
    Not,
}

impl Serialize for Domain {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Domain::Condition(field, operator, value) => {
                (field, operator, value).serialize(serializer)
            }
            Domain::And => "&".serialize(serializer),
            Domain::Or => "|".serialize(serializer),
            Domain::Not => "!".serialize(serializer),
        }
    }
}

struct DomainVisitor;

impl<'de> Visitor<'de> for DomainVisitor {
    type Value = Domain;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "string or array with 3 element")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "&" => Ok(Domain::And),
            "|" => Ok(Domain::Or),
            "!" => Ok(Domain::Not),
            _ => Err(de::Error::invalid_value(de::Unexpected::Str(v), &self)),
        }
    }
    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: de::SeqAccess<'de>,
    {
        Ok(Domain::Condition(
            seq.next_element()?
                .ok_or(de::Error::invalid_length(0, &self))?,
            seq.next_element()?
                .ok_or(de::Error::invalid_length(1, &self))?,
            seq.next_element()?
                .ok_or(de::Error::invalid_length(2, &self))?,
        ))
    }
}

impl<'de> Deserialize<'de> for Domain {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let visitor = DomainVisitor;
        deserializer.deserialize_any(visitor)
    }
}

impl Domain {
    pub fn condition<A, B, C>(a: A, b: B, c: C) -> Domain
    where
        A: Into<String>,
        B: Into<String>,
        C: Into<serde_json::Value>,
    {
        Self::Condition(a.into(), b.into(), c.into())
    }
}

#[cfg(test)]
mod ser_tests {
    use super::operators;
    use serde_json::json;

    use crate::Domain;

    #[test]
    fn test_condition() {
        assert_eq!(
            serde_json::to_value(Domain::condition(
                "name",
                operators::EQUALS_TO,
                json!("aaaa")
            ))
            .unwrap(),
            json!(["name", operators::EQUALS_TO, "aaaa"])
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(serde_json::to_value(Domain::And).unwrap(), json!("&"));
    }

    #[test]
    fn test_or() {
        assert_eq!(serde_json::to_value(Domain::Or).unwrap(), json!("|"));
    }

    #[test]
    fn test_not() {
        assert_eq!(serde_json::to_value(Domain::Not).unwrap(), json!("!"));
    }
}

#[cfg(test)]
mod deser_tests {
    use super::operators;
    use serde_json::json;

    use crate::Domain;

    #[test]
    fn test_condition() {
        assert_eq!(
            Domain::condition("name", operators::EQUALS_TO, json!("aaaa")),
            serde_json::from_value(json!(["name", operators::EQUALS_TO, "aaaa"])).unwrap()
        );
    }

    #[test]
    fn test_and() {
        assert_eq!(Domain::And, serde_json::from_value(json!("&")).unwrap());
    }

    #[test]
    fn test_or() {
        assert_eq!(Domain::Or, serde_json::from_value(json!("|")).unwrap());
    }

    #[test]
    fn test_not() {
        assert_eq!(Domain::Not, serde_json::from_value(json!("!")).unwrap());
    }
}
