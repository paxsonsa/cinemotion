use super::*;
use crate::api;

macro_rules! with_command {
    ($command:expr, $model:expr, mut& $engine:ident, $state:ident, $block:block) => {
        let command = $command($model);
        $engine.apply(command).await.unwrap();
        let $state = $engine.tick().await.unwrap();
        $block
    };
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
            println!("{:?}", state);
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
            assert_eq!(state.scene.objects[1].name, "objectA");
        }
    );

    // TODO: Test duplicate scene object name
    // TODO: Test rewrite of scene object.

    // Client Setup
    // TODO Add Scene, Add Scene Object, Add Scene Object Attribute
    // TODO define client attribute mappings

    // Live/Recording Mode
    // TODO Add mode change command

    // Event Tracking
    // TODO Add Touch Events
    // TODO Add Trigger Events
}

async fn test_default_engine_state(engine: &mut Engine) {
    let state = engine.tick().await.unwrap();
    assert_eq!(state.scene.name, "default");
    assert_eq!(state.scene.objects.len(), 1);
    assert_eq!(state.scene.objects[0].id, Some(0));
    assert_eq!(state.scene.objects[0].attributes.len(), 3);
    assert_eq!(state.scene.objects[0].attributes[0].name, "translate");
    assert_eq!(state.scene.objects[0].attributes[1].name, "orientation");
    assert_eq!(state.scene.objects[0].attributes[2].name, "velocity");
}
