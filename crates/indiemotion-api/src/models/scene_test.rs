use super::*;
use serde_json;

#[test]
fn test_scene_object_serde() {
    let mut obj = SceneObject::default();
    obj.properties_mut().insert(
        name!("position"),
        PropertyState::bind(name!("controllerA"), name!("position"), Value::vec3()),
    );

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());

    let data = r#"{
        "name": "default",
        "properties": {
          "position": {
            "value": {
              "x": 0.0,
              "y": 0.0,
              "z": 0.0
            },
            "binding": {
              "namespace": "controllerA",
              "property": "position"
            }
          },
          "orientation": {
            "value": {
              "x": 0.0,
              "y": 0.0,
              "z": 0.0
            }
          },
          "velocity": {
            "value": {
              "x": 0.0,
              "y": 0.0,
              "z": 0.0
            }
          }
        }
      }"#;

    let _: SceneObject = serde_json::from_str(data).unwrap();
}

#[test]
fn test_scene_serde() {
    let scene = Scene::default();
    let json = serde_json::to_string_pretty(&scene).unwrap();
    let _: Scene = serde_json::from_str(&json).unwrap();
}
