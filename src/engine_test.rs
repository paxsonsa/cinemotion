use super::*;

#[tokio::test]
async fn test_engine_properties() {
    let mut engine = Engine::new();

    let property = api::Property::new(
        "namespace".to_string(),
        "name".to_string(),
        api::PropertyValue::Int(1),
    );
    let prop_id = property.id().clone();
    let other_id = api::ProperyID::try_from("other.name").expect("Failed to create ProperyID");

    engine.add_property(property);
    assert_eq!(
        engine.property_table.len(),
        1,
        "Expect the property table to have a single entry from previous call"
    );
    assert_eq!(
        engine.update_queue.lock().await.len(),
        0,
        "No pending property updates should be present."
    );

    engine
        .append_property_update(prop_id.clone(), api::PropertyValue::Int(2))
        .await
        .expect("Failed to append property update 1");
    engine
        .append_property_update(prop_id.clone(), api::PropertyValue::Int(3))
        .await
        .expect("Failed to append property update 2");
    engine
        .append_property_update(prop_id.clone(), api::PropertyValue::Float(42.0))
        .await
        .expect_err("Should error when adding a property update for a property that changes it's value type.");
    engine
        .append_property_update(other_id, api::PropertyValue::Int(10))
        .await
        .expect_err(
            "Should error when adding a property update for a property that does not exist",
        );
    assert_eq!(engine.update_queue.lock().await.len(), 2);

    let property_values = engine.step().await.expect("Failed to step engine");

    assert_eq!(property_values.len(), 1);
    assert_eq!(engine.property_table.len(), 1);
    assert_eq!(engine.update_queue.lock().await.len(), 0);

    assert_eq!(
        property_values.get(&prop_id).unwrap(),
        &api::PropertyValue::Int(3)
    );
}
