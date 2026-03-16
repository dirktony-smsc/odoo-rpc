pub(crate) fn empty_dict() -> serde_json::Value {
    serde_json::Value::Object(Default::default())
}
