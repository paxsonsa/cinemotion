use super::*;
use serde_json;

#[test]
fn test_mode_serde() {
    let data = r#"
        {
            "mode": "idle"
        }
    "#;

    let _: Mode = serde_json::from_str(data).unwrap();
}
