use super::*;
use serde_json;

#[test]
fn test_sample_serde() {
    let data = r#"
        {
            "properties": {
                "position": {
                    "x": 1.0,
                    "y": 2.0,
                    "z": 3.0
                },
                "scale": 10.0
            }
        }
    "#;

    let _: Sample = serde_json::from_str(data).unwrap();
}
