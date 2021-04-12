use serde_json::{json, Value};
use std::collections::BTreeMap;

#[test]
fn newjs() {
    let map: BTreeMap<String, Value> = tool::newjs!(("hey", "hello"), ("hi", 3.7));

    assert_eq!(map["hey"], "hello");
    assert_eq!(map["hi"], json!(3.7));
}

#[test]
fn newjs_empty() {
    let map: BTreeMap<String, Value> = tool::newjs!();

    assert_eq!(map.len(), 0);
}

#[test]
fn addjs() {
    let mut map = BTreeMap::<String, Value>::new();
    tool::addjs!(&mut map, ("hey", "hello"), ("hi", 3.7));

    assert_eq!(map["hey"], "hello");
    assert_eq!(map["hi"], json!(3.7));
}

#[test]
fn writejs() {
    let map: BTreeMap<String, Value> = tool::newjs!(("hey", "hello"), ("hi", 3.7));
    tool::writejs!("rsc/tests/writejs.json", map);
}
