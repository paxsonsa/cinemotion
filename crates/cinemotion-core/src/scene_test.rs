use super::*;

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
