use serde_json::json;

#[test]
fn newjs() {
    let map = tool::newjs!(("hey", "hello"), ("hi", 3.7));

    assert_eq!(map["hey"], "hello");
    assert_eq!(map["hi"], json!(3.7));
}

#[test]
fn addjs() {
    let mut map = json!({});
    tool::addjs!(&mut map, ("hey", "hello"), ("hi", 3.7));

    assert_eq!(map["hey"], "hello");
    assert_eq!(map["hi"], json!(3.7));
}

#[test]
fn writejs() {
    let map = tool::newjs!(("hey", "hello"), ("hi", 3.7));
    tool::writejs!("rsc/tests/writejs.json", map);
}
