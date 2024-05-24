use crate::name;

use super::*;

#[tokio::test]
async fn test_device_registration() {
    let mut engine = Engine::new();

    engine.setup().expect("setup should not fail");

    let mut command = commands::DeviceRegister::new("deviceA");
    command.attributes.push(Attribute::new(
        name!("transform"),
        AttributeValue::matrix44(),
    ));
    let (command, mut receiver) = CommandInfo::with(command);
    engine
        .process(command)
        .await
        .expect("device registration to complete");

    let Ok(Some(reply)) = receiver.try_recv().unwrap() else {
        panic!("should receive reply with device id.")
    };

    assert!(matches!(
        CommandReply::EntityId(0),
        CommandReply::EntityId(_reply)
    ));

    engine.tick().await.expect("tick should not failed");

    let world = engine.get_world_mut();
    for item in world.query::<&Device>().iter(&world) {
        assert_eq!(item.name(), name!("deviceA"));
        assert_eq!(item.attributes().len(), 1);
    }
}
