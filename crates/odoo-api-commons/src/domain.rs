use serde::Serialize;

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
