use super::*;
use serde_json;

#[test]
fn test_value_serde() {
    let data = r#"10.0"#;
    let value: Value = serde_json::from_str(data).unwrap();
    assert!(matches!(value, Value::Float(_)));

    let data = r#"[10.0, 5.0, 0.0]"#;
    let value: Value = serde_json::from_str(data).unwrap();
    assert!(matches!(value, Value::Vec3(_)));

    let data = r#"[10.0, 5.0, 0.0, 1.0]"#;
    let value: Value = serde_json::from_str(data).unwrap();
    assert!(matches!(value, Value::Vec4(_)));

    let data = r#"[
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0]
    ]"#;
    let value: Value = serde_json::from_str(data).unwrap();
    assert!(matches!(value, Value::Matrix44(_)));
}
