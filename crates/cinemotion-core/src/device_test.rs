use super::*;
use crate::attributes::AttributeValue;
use crate::commands::CommandReply;
use crate::prelude::name;

#[tokio::test]
async fn test_device_system_command_registration() {
    let mut world = crate::world::new();
    let mut device = Device::new("deviceA");
    device
        .attributes
        .insert(Attribute::new("transform", AttributeValue::matrix44()));
    let command = Command::Register(device);

    let _reply = commands::process(&mut world, command)
        .unwrap()
        .expect("no device id returned");

    assert!(matches!(
        CommandReply::EntityId(0),
        CommandReply::EntityId(_reply),
    ));
    let devices = system::get_all(&mut world);
    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].name(&world), &name!("deviceA"));
    assert!(devices[0]
        .attributes(&world)
        .get(&name!("transform"))
        .is_some());
}

#[tokio::test]
async fn test_device_system_command_update() {
    let mut world = crate::world::new();
    let mut device = Device::new("deviceA");
    device
        .attributes
        .insert(Attribute::new_matrix44("transform"));

    let id = system::add_device(&mut world, device.clone());

    // Add a new attribute
    device.attributes.insert(Attribute::new_vec3("vel"));

    let command = Command::Update((id, device));

    let _reply = commands::process(&mut world, command)
        .unwrap()
        .expect("no device id returned");

    assert!(matches!(
        CommandReply::EntityId(0),
        CommandReply::EntityId(_reply),
    ));
    let devices = system::get_all(&mut world);

    assert_eq!(devices.len(), 1);
    assert_eq!(devices[0].name(&world), &name!("deviceA"));
    assert!(devices[0]
        .attributes(&world)
        .get(&name!("transform"))
        .is_some());
    assert!(devices[0].attributes(&world).get(&name!("vel")).is_some());
}

#[tokio::test]
async fn test_device_system_command_remove() {
    let mut world = crate::world::new();
    let mut device = Device::new("deviceA");
    device
        .attributes
        .insert(Attribute::new_matrix44("transform"));

    let id = system::add_device(&mut world, device.clone());
    let command = Command::Remove(id);
    let _reply = commands::process(&mut world, command)
        .unwrap()
        .expect("no device id returned");

    assert!(matches!(
        CommandReply::EntityId(0),
        CommandReply::EntityId(_reply),
    ));
    let devices = system::get_all(&mut world);
    assert_eq!(devices.len(), 0);
}
