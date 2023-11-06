use super::*;
use serde_json;

#[test]
fn test_property_def_serde() {
    let data = r#"
        {
            "name": "propertyA",
            "default_value": [0.0, 0.0, 0.0]
        }
    "#;

    let _: PropertyDef = serde_json::from_str(data).unwrap();
}

#[test]
fn test_property_state_serde() {
    let data = r#"
    {
        "value": {
          "x": 0.0,
          "y": 0.0,
          "z": 0.0
        },
        "binding": {
          "namespace": "namespace",
          "property": "property"
        }
      }
    "#;
    let state: PropertyState = serde_json::from_str(data).unwrap();
    println!("{:?}", state);
    assert!(matches!(state, PropertyState::Bound { .. }));

    let data = r#"
        {
            "value": [0.0, 0.0, 0.0]
        }
    "#;
    let state: PropertyState = serde_json::from_str(data).unwrap();
    assert!(matches!(state, PropertyState::Unbound { .. }));
}

// #[test]
// fn test_property_binding_serde() {
//     let data = r#"
//         {
//             "namespace": "controllerA",
//             "property": "position"
//         }
//     "#;

//     let _: PropertyBinding = serde_json::from_str(data).unwrap();
// }
