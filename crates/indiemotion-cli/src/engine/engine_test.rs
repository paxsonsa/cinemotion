use std::collections::HashMap;

use api::models::{Property, Value};

use super::*;
use crate::api;
use crate::api::name;

macro_rules! with_command {
    ($command:expr, $model:expr, mut& $engine:ident, $state:ident, $block:block) => {
        let cmd = $command($model);
        $engine.apply(0, cmd).await.unwrap();
        let $state = $engine.tick().await.unwrap();
        $block
    };
    ($command:expr, $model:expr, mut& $engine:ident) => {{
        let cmd = $command($model);
        $engine.apply(cmd).await.unwrap();
        $engine.tick().await
    }};
}

macro_rules! engine {
    () => {{
        engine::Engine::default()
    }};
}

#[tokio::test]
async fn test_basic_runtime() {
    let mut engine = engine!();

    test_default_engine_state(&mut engine).await;

    with_command!(
        api::Command::Controller,
        api::models::ControllerDef::new(
            "controllerA".into(),
            vec![
                api::models::ControllerPropertyDef::new(
                    name!("position"),
                    (0.0, 0.0, 0.0).into(),
                )
            ],
        ),
        mut &engine,
        state,
        {
            let con = state.controllers.get(&name!("controllerA")).expect("the controller should be added after applying the command");
            con.value(&name!("position")).expect("the controller should have a position property");
        }
    );

    with_command!(
        api::Command::SceneObject,
        api::models::SceneObject::new(
            "objectA".into(),
            HashMap::from([
                (name!("position"), Property::bind(name!("controllerA"), name!("position"), Value::vec3())),
                (name!("rotate"), Value::vec3().into()),
                (name!("scale"), Value::vec3().into()),
            ]),
        ),
        mut &engine,
        state,
        {
            assert_eq!(state.scene.objects().len(), 2);
            assert_eq!(state.scene.object(&name!("objectA")).unwrap().name(), &"objectA".into());
        }
    );
    with_command!(
        api::Command::Sample,
        api::models::Sample::new(
            vec![
                api::models::SampleProperty {
                    name: name!("position"),
                    value: (1.0, 1.0, 1.0).into(),
                }
            ],
        ),
        mut &engine,
        state,
        {
            // Without the mode being set to Live or Recording, the sample should not be applied.
            let expected: (f64, f64, f64) = (0.0, 0.0, 0.0);

            let obj = state.scene.object(&name!("objectA")).unwrap();
            let vec3 = obj.property(&name!("position")).unwrap().value().as_vec3().unwrap();
            assert_eq!(vec3, expected);
        }
    );

    with_command!(
        api::Command::Mode,
        api::models::Mode::Live,
        mut &engine,
        state,
        {
            assert!(matches!(state.mode, api::models::Mode::Live));
        }
    );

    println!("Sample being applied.");

    with_command!(
        api::Command::Sample,
        api::models::Sample::new(
            vec![
                api::models::SampleProperty {
                    name: name!("position"),
                    value: (1.0, 1.0, 1.0).into(),
                }
            ],
        ),
        mut &engine,
        state,
        {
            // The mode being set to Live or Recording, the sample should be applied.

            let expected: (f64, f64, f64) = (1.0, 1.0, 1.0);

            let obj = state.scene.object(&name!("objectA")).unwrap();
            let vec3 = obj.property(&name!("position")).unwrap().value().as_vec3().unwrap();
            assert_eq!(vec3, expected);
        }
    );

    with_command!(
        api::Command::Mode,
        api::models::Mode::Idle,
        mut &engine,
        state,
        {
            engine.tick().await.expect("the engine should tick without error");
            assert!(matches!(state.mode, api::models::Mode::Idle));

            let expected: (f64, f64, f64) = (0.0, 0.0, 0.0);

            // Should reset to default value.
            let obj = state.scene.object(&name!("objectA")).unwrap();
            let vec3 = obj.property(&name!("position")).unwrap().value().as_vec3().unwrap();
            assert_eq!(vec3, expected);

        }
    );

    // TODO Add Object Property Offset
    // TODO Add Controller Property Scale
    // TODO Add Object Property Effects
    // -Velocity - Acceleration - Filter

    // Event Tracking
    // TODO Add Touch Events
    // TODO Add Trigger Events

    // Blender/Unity/Unreal Integration (Python)
}

async fn test_default_engine_state(engine: &mut Engine) {
    let state = engine.tick().await.unwrap();
    assert_eq!(state.scene.name, "default");
    assert_eq!(state.scene.objects().len(), 1);

    let obj = state.scene.object(&name!("default")).unwrap();
    assert_eq!(obj.name(), &"default".into());
    assert_eq!(obj.properties().len(), 3);
    assert!(obj.property(&name!("position")).is_some());
    assert!(obj.property(&name!("orientation")).is_some());
    assert!(obj.property(&name!("velocity")).is_some());
}
