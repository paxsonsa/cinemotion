use super::*;
use crate::api;

macro_rules! with_command {
    ($command:expr, $model:expr, mut& $engine:ident, $state:ident, $block:block) => {
        let command = $command($model);
        let cmd = super::ClientCommand { client: 0, command };
        $engine.apply(cmd).await.unwrap();
        let $state = $engine.tick().await.unwrap();
        $block
    };
    ($command:expr, $model:expr, mut& $engine:ident) => {{
        let command = $command($model);
        let cmd = super::ClientCommand { client: 0, command };
        $engine.apply(cmd).await.unwrap();
        $engine.tick().await
    }};
}

macro_rules! apply_command {
    ($command:expr, $model:expr, mut& $engine:ident) => {{
        let command = $command($model);
        let cmd = super::ClientCommand { client: 0, command };
        $engine.apply(cmd).await
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
        api::Command::SetClient,
        api::models::Client::new(0, "clientA".to_string()),
        mut &engine,
        state,
        {
            assert_eq!(state.clients.len(), 1);
            assert_eq!(state.clients[0].id, 0);
            assert_eq!(state.clients[0].name, "clientA");
        }
    );

    with_command!(
        api::Command::SceneObject,
        api::models::SceneObject {
            id: None,
            name: "objectA".to_string(),
            attributes: vec![],
        },
        mut &engine,
        state,
        {
            println!("{:?}", state);
            assert_eq!(state.scene.objects.len(), 2);
            assert_eq!(state.scene.objects.get(&1).unwrap().name, "objectA");
        }
    );

    // TODO: Test rewrite of scene object.
    // TODO: Add Error Channel for Commands
    // Commands need client ID for routing
    // Engine tick should return a result type that includes errors for clients but also continues to send state updates.

    // Client Setup
    // TODO Add Scene, Add Scene Object, Add Scene Object Attribute
    // TODO define client attribute mappings

    // Live/Recording Mode
    // TODO Add mode change command

    // Event Tracking
    // TODO Add Touch Events
    // TODO Add Trigger Events

    // Blender/Unity/Unreal Integration (Python)
}

#[tokio::test]
async fn test_scene_object_behaviour() {
    let mut engine = engine!();

    let _ = with_command!(
        api::Command::SetClient,
        api::models::Client::new(0, "clientA".to_string()),
        mut &engine
    )
    .expect("setting client failed.");

    let state = with_command!(
        api::Command::SceneObject,
        api::models::SceneObject {
            id: None,
            name: "objectA".to_string(),
            attributes: vec![],
        },
        mut &engine
    )
    .expect("initial scene object should not fail");
    let id = state.scene.objects.get(&1).unwrap().id;

    let _ = apply_command!(
        api::Command::SceneObject,
        api::models::SceneObject {
            id: None,
            name: "objectA".to_string(),
            attributes: vec![],
        },
        mut &engine
    )
    .expect_err("adding new object with no id and a name should fail.");
    let _ = engine.tick().await.expect("tick should be fine.");

    let state = with_command!(
        api::Command::SceneObject,
        api::models::SceneObject {
            id,
            name: "newObjectB".to_string(),
            attributes: vec![],
        },
        mut &engine
    )
    .expect("initial scene object should not fail");

    assert_eq!(
        state.scene.objects.get(&1).unwrap().name,
        "newObjectB".to_string()
    );
}

async fn test_default_engine_state(engine: &mut Engine) {
    let state = engine.tick().await.unwrap();
    assert_eq!(state.scene.name, "default");
    assert_eq!(state.scene.objects.len(), 1);

    let obj = state.scene.objects.get(&0).unwrap();
    assert_eq!(obj.id, Some(0));
    assert_eq!(obj.attributes.len(), 3);
    assert_eq!(obj.attributes[0].name, "translate");
    assert_eq!(obj.attributes[1].name, "orientation");
    assert_eq!(obj.attributes[2].name, "velocity");
}
