use super::*;
use crate::attributes::AttributeValue;
use crate::commands::CommandReply;
use crate::name;

#[tokio::test]
async fn test_device_system_command_registration() {
    let mut world = crate::world::new();
    let mut device = Device::new("deviceA");
    device.insert_attribute(Attribute::new("transform", AttributeValue::matrix44()));
    let command = Command::Register(device);

    let _reply = commands::process(&mut world, command)
        .unwrap()
        .expect("no device id returned");

    assert!(matches!(
        CommandReply::EntityId(0),
        CommandReply::EntityId(_reply),
    ));
    let devices = world
        .query::<&Device>()
        .iter(&world)
        .collect::<Vec<&Device>>();

    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].name, name!("deviceA"));
    assert!(devices[0].attributes().get(&name!("transform")).is_some());
}

#[tokio::test]
async fn test_device_system_command_update() {
    let mut world = crate::world::new();
    let mut device = Device::new("deviceA");
    device.insert_attribute(Attribute::new_matrix44("transform"));

    let id = commands::add_device(&mut world, device.clone());
    println!("id: {}", id);

    // Add a new attribute
    device.insert_attribute(Attribute::new_vec3("vel"));
    device.set_id(id);

    let command = Command::Update(device);

    let _reply = commands::process(&mut world, command)
        .unwrap()
        .expect("no device id returned");

    assert!(matches!(
        CommandReply::EntityId(0),
        CommandReply::EntityId(_reply),
    ));
    let devices = world
        .query::<&Device>()
        .iter(&world)
        .collect::<Vec<&Device>>();

    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].name, name!("deviceA"));
    assert!(devices[0].attributes().get(&name!("transform")).is_some());
    assert!(devices[0].attributes().get(&name!("vel")).is_some());
}

#[tokio::test]
async fn test_device_system_command_remove() {
    let mut world = crate::world::new();
    let mut device = Device::new("deviceA");
    device.insert_attribute(Attribute::new_matrix44("transform"));

    let id = commands::add_device(&mut world, device.clone());
    println!("id: {}", id);

    let command = Command::Remove(id);

    let _reply = commands::process(&mut world, command)
        .unwrap()
        .expect("no device id returned");

    assert!(matches!(
        CommandReply::EntityId(0),
        CommandReply::EntityId(_reply),
    ));
    let devices = world
        .query::<&Device>()
        .iter(&world)
        .collect::<Vec<&Device>>();

    assert_eq!(devices.len(), 0);
}
