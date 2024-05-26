use super::*;

#[tokio::test]
async fn test_scene_system_init() {
    let mut world = world::new();
    system::init(&mut world);

    let objects = system::get_all(&mut world);

    assert_eq!(objects.len(), 1);
    let object = objects.first().unwrap();
    assert_eq!(object.name(), name!("default"));
}

#[tokio::test]
async fn test_scene_command_add_object() {
    let mut world = world::new();

    let mut object = SceneObject::new("camera1");
    object.insert_attribute(Attribute::new_matrix44("transform"));

    let command = Command::AddObject(object);
    let _reply = commands::procces(&mut world, command)
        .unwrap()
        .expect("expected a object id for the engine");

    let objects = system::get_all(&mut world);
    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].name(), name!("camera1"));
    println!("{:?}", objects[0].attributes());
    assert!(objects[0].attributes().get(&name!("transform")).is_some());
}

#[tokio::test]
async fn test_scene_command_update_object() {
    let mut world = world::new();

    let mut object = SceneObject::new("camera1");
    object.insert_attribute(Attribute::new_matrix44("transform"));
    let id = system::add_scene_object(&mut world, object.clone());

    object.insert_attribute(Attribute::new_vec3("vel"));

    let command = Command::UpdateObject(id, object);
    let _reply = commands::procces(&mut world, command)
        .unwrap()
        .expect("expected a object id for the engine");

    let objects = system::get_all(&mut world);
    assert_eq!(objects[0].name(), name!("camera1"));
    assert_eq!(objects[0].attributes().len(), 2);
    assert!(objects[0].attributes().get(&name!("transform")).is_some());
    assert!(objects[0].attributes().get(&name!("vel")).is_some());
}

#[tokio::test]
async fn test_scene_command_remove_object() {
    let mut world = world::new();

    let mut object = SceneObject::new("camera1");
    object.insert_attribute(Attribute::new_matrix44("transform"));
    let id = system::add_scene_object(&mut world, object.clone());

    object.insert_attribute(Attribute::new_vec3("vel"));

    let command = Command::RemoveObject(id);
    let _reply = commands::procces(&mut world, command)
        .unwrap()
        .expect("expected a object id for the engine");

    let objects = system::get_all(&mut world);
    assert_eq!(objects.len(), 0);
}

#[tokio::test]
async fn test_scene_system_attribute_links() {
    let mut world = world::new();

    let mut device = Device::new("root");
    device.insert_attribute(Attribute::new_matrix44("transform"));

    // TODO: Rename device to devices
    let device_id = crate::device::system::add_device(&mut world, device.clone());

    let mut object = SceneObject::new("camera1");
    object.insert_attribute(Attribute::new_matrix44("transform"));
    object
        .insert_link(AttributeLink::mapped(device_id, "transform"))
        .expect("should not fail");
    let id = system::add_scene_object(&mut world, object.clone());

    system::update(&mut world).expect("should not fail on first iteration");

    let object_ref = system::get_by_id(&mut world, id).expect("object should exist");
    let device_ref = device::system::get_by_id(&mut world, id).expect("device should exist");

    // The object's linked attribute should be updated to match the device's attribute.
    assert_eq!(
        object_ref.attribute("transform").value(),
        device_ref.attribute("transform").value()
    );

    // Update the device's transform to something and update the world with it.
    let mut value = AttributeValue::matrix44();
    value.as_matrix44_mut().unwrap().set(0, 1, 100.0);
    let mut attribute = Attribute::new("transform", value);
    device.insert_attribute(attribute);

    crate::device::system::set_device(&mut world, device_id, device);

    let object_ref = system::get_by_id(&mut world, id).expect("object should exist");
    let device_ref = device::system::get_by_id(&mut world, id).expect("device should exist");

    // The object's linked attribute should NOT be updated to match the device's attribute.
    assert_ne!(
        object_ref.attribute("transform").value(),
        device_ref.attribute("transform").value()
    );

    system::update(&mut world).expect("should not fail on first iteration");

    let object_ref = system::get_by_id(&mut world, id).expect("object should exist");
    let device_ref = device::system::get_by_id(&mut world, id).expect("device should exist");

    // The object's linked attribute should be updated to match the device's attribute.
    assert_ne!(
        object_ref.attribute("transform").value(),
        device_ref.attribute("transform").value()
    );
}
