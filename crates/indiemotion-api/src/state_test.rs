use crate::name;

use super::*;
use serde_json;

#[test]
fn test_global_state_serde() {
    let data = r#"
    {
        "name": "controllerA",
        "properties": [
            {
                "name": "propertyA",
                "default_value": 10.0
            },
            {
                "name": "propertyB",
                "default_value": [0.0, 0.0, 0.0]
            }
        ]
    }
    "#;
}
