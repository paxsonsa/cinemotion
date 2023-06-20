use super::*;
use crate::api;

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

macro_rules! apply_command {
    ($command:expr, $model:expr, mut& $engine:ident) => {{
        let cmd = $command($model);
        $engine.apply(0, cmd).await
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
        api::models::Controller::new(
            "controllerA".to_string(),
            vec![
                api::models::ControllerProperty::new(
                    "translate".to_string(),
                    (0.0, 0.0, 0.0).into(),
                )
            ],
        ),
        mut &engine,
        state,
        {
            let con = state.controllers.get("controllerA").expect("the controller should be added after applying the command");
            con.property("translate").expect("the controller should have a translate property");
        }
    );

    with_command!(
        api::Command::SceneObject,
        api::models::SceneObject::new(
            "objectA".into(),
            vec![
                api::models::ObjectProperty::new(
                    "translate".to_string(),
                    (0.0, 0.0, 0.0).into(),
                    Some("controllerA.translate".into()),
                )
            ],
            ),
        mut &engine,
        state,
        {
            engine.tick().await.unwrap();
            assert_eq!(state.scene.objects().len(), 3);
            assert_eq!(state.scene.object("objectA".into()).unwrap().name(), &"objectA".into());
        }
    );

    with_command!(
        api::Command::Sample,
        api::models::Sample::new(
            vec![
                api::models::SampleProperty {
                    name: "translate".to_string(),
                    value: (1.0, 1.0, 1.0).into(),
                }
            ],
        ),
        mut &engine,
        state,
        {
            let expected: (f64, f64, f64) = (1.0, 1.0, 1.0);

            let obj = state.scene.object("objectA".into()).unwrap();
            let vec3 = obj.property("translate").unwrap().as_vec3().unwrap();
            assert_eq!(vec3, expected);

            let obj = state.scene.object("controllerA".into()).unwrap();
            let vec3 = obj.property("translate").unwrap().as_vec3().unwrap();
            assert_eq!(vec3, expected);
        }
    );

    // Client Setup
    // TODO define client attribute mappings

    // Live/Recording Mode
    // TODO Add mode change command

    // Event Tracking
    // TODO Add Touch Events
    // TODO Add Trigger Events

    // Blender/Unity/Unreal Integration (Python)
}

async fn test_default_engine_state(engine: &mut Engine) {
    let state = engine.tick().await.unwrap();
    assert_eq!(state.scene.name, "default");
    assert_eq!(state.scene.objects().len(), 1);

    let obj = state.scene.object("default".into()).unwrap();
    assert_eq!(obj.name(), &"default".into());
    assert_eq!(obj.properties().len(), 3);
    assert!(obj.property("translate").is_some());
    assert!(obj.property("orientation").is_some());
    assert!(obj.property("velocity").is_some());
}
