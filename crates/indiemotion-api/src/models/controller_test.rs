use crate::name;

use super::*;
use serde_json;

#[test]
fn test_controller_property_def_serde() {
    let data = r#"{
        "name": "propertyA",
        "default_value": 10.0
    }"#;
    let property: ControllerPropertyDef = serde_json::from_str(data).unwrap();
    assert_eq!(property.name(), &name!("propertyA"));
    assert!(matches!(property.default_value, Value::Float(_)));
}

#[test]
fn test_controller_def_serde() {
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
    let def: ControllerDef = serde_json::from_str(data).unwrap();
    assert_eq!(def.name(), &name!("controllerA"));
    assert_eq!(def.properties().len(), 2);
}

#[test]
fn test_controller_state_serde() {
    let data = r#"
    {
        "values": {
            "propertyA": 10.0,
            "propertyB": [0.0, 0.0, 0.0]
        },
        "metadata": {
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
    }
    "#;
    let def: ControllerState = serde_json::from_str(data).unwrap();
    assert_eq!(def.name(), &name!("controllerA"));
    assert_eq!(def.values().len(), 2);
}

#[test]
fn test_controller_state_value_updates() {
    let def = ControllerDef::new(
        name!("controllerA"),
        vec![
            PropertyDef::new(name!("propertyA"), 10.0.into()),
            PropertyDef::new(name!("propertyB"), (0.0, 1.0, 0.0).into()),
        ],
    );

    let mut state = ControllerState::from(def);

    assert_eq!(
        state
            .value(&name!("propertyA"))
            .expect("propertyA should exist from the definition")
            .as_f64()
            .unwrap(),
        &10.0
    );

    state
        .value_mut(&name!("propertyA"))
        .expect("propertyA should exist from the definition")
        .update(&20.0_f64.into())
        .expect("the value should be updated");

    state
        .value_mut(&name!("propertyB"))
        .expect("propertyB should exist from the definition")
        .update(&20.0_f64.into())
        .expect_err("the value should not be updated");

    state
        .value_mut(&name!("propertyB"))
        .expect("propertyB should exist from the definition")
        .update(&(10.0, 10.0, 10.0).into())
        .expect("the value should be updated");

    assert_eq!(
        state
            .value(&name!("propertyA"))
            .expect("propertyA should exist from the definition")
            .as_f64()
            .unwrap(),
        &20.0
    );

    assert_eq!(
        state
            .value(&name!("propertyB"))
            .expect("propertyB should exist from the definition")
            .as_vec3()
            .unwrap(),
        &(10.0, 10.0, 10.0)
    );

    state.reset();

    assert_eq!(
        state
            .value(&name!("propertyA"))
            .expect("propertyA should exist from the definition")
            .as_f64()
            .unwrap(),
        &10.0
    );

    assert_eq!(
        state
            .value(&name!("propertyB"))
            .expect("propertyB should exist from the definition")
            .as_vec3()
            .unwrap(),
        &(0.0, 1.0, 0.0)
    );

    let def = ControllerDef::new(
        name!("controllerA"),
        vec![
            PropertyDef::new(name!("propertyA"), 10.0.into()),
            PropertyDef::new(name!("propertyB"), (0.0, 1.0, 0.0).into()),
            PropertyDef::new(name!("propertyC"), (0.0, 1.0, 0.0).into()),
        ],
    );
    state.redefine(def);

    assert_eq!(state.values().len(), 3);
}
