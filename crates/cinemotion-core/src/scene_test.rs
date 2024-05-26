use super::*;

use crate::name;

#[tokio::test]
async fn test_scene_system_init() {
    let mut world = world::new();
    system::init(&mut world);

    let objects = world
        .query::<&SceneObject>()
        .iter(&world)
        .collect::<Vec<&SceneObject>>();

    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].name, "default".into());
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

    let objects = world
        .query::<&SceneObject>()
        .iter(&world)
        .collect::<Vec<&SceneObject>>();

    assert_eq!(objects.len(), 1);
    assert_eq!(objects[0].name, name!("camera1"));
    println!("{:?}", objects[0].attributes());
    assert!(objects[0].attributes().get(&name!("transform")).is_some());
}
