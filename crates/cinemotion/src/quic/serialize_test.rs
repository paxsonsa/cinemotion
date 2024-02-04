use bytes::{BufMut, BytesMut};
use pretty_assertions_sorted::assert_eq_sorted;

use super::*;
use crate::*;

#[test]
fn test_value_float_deserialization() {
    let mut bytes = BytesMut::new();
    bytes.put_u8(1);
    bytes.put_f64(88.0);

    let bytes = bytes.freeze();
    let value = data::Value::try_from(&mut bytes.into()).expect("the value should be parsable.");
    assert_eq!(value, data::Value::Float(88.0));
}

#[test]
fn test_value_vec3_deserialization() {
    let mut bytes = BytesMut::new();
    bytes.put_u8(2);
    bytes.put_f64(1.0);
    bytes.put_f64(2.0);
    bytes.put_f64(3.0);

    let bytes = bytes.freeze();
    let value = data::Value::try_from(&mut bytes.into()).expect("the value should be parsable.");
    assert_eq!(value, data::Value::Vec3(data::Vec3::from((1.0, 2.0, 3.0))));
}

#[test]
fn test_value_vec4_deserialization() {
    let mut bytes = BytesMut::new();
    bytes.put_u8(3);
    bytes.put_f64(1.0);
    bytes.put_f64(2.0);
    bytes.put_f64(3.0);
    bytes.put_f64(4.0);

    let bytes = bytes.freeze();
    let value = data::Value::try_from(&mut bytes.into()).expect("the value should be parsable.");
    assert_eq!(
        value,
        data::Value::Vec4(data::Vec4::from((1.0, 2.0, 3.0, 4.0)))
    );
}

#[test]
fn test_value_matrix44_deserialization() {
    let mut bytes = BytesMut::new();
    bytes.put_u8(4);
    bytes.put_f64(1.0);
    bytes.put_f64(2.0);
    bytes.put_f64(3.0);
    bytes.put_f64(4.0);
    bytes.put_f64(5.0);
    bytes.put_f64(6.0);
    bytes.put_f64(7.0);
    bytes.put_f64(8.0);
    bytes.put_f64(9.0);
    bytes.put_f64(10.0);
    bytes.put_f64(11.0);
    bytes.put_f64(12.0);
    bytes.put_f64(13.0);
    bytes.put_f64(14.0);
    bytes.put_f64(15.0);
    bytes.put_f64(16.0);

    let bytes = bytes.freeze();
    let value = data::Value::try_from(&mut bytes.into()).expect("the value should be parsable.");
    assert_eq!(
        value,
        data::Value::Matrix44(data::Matrix44::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 10.0, 11.0, 12.0],
            [13.0, 14.0, 15.0, 16.0]
        ]))
    );
}

#[test]
fn test_init_deserialization() {
    let mut bytes = BytesMut::new();

    // Controller name
    let name: Bytes = "my controller 🕹️".into();
    bytes.put_u16(name.len() as u16);
    bytes.put(name);

    // Add Properties
    bytes.put_u16(4); // number of properties

    let name = Bytes::from("propertyA");
    bytes.put_u16(name.len() as u16);
    bytes.put(name);
    bytes.put_u8(1);
    bytes.put_f64(1.0);

    let name = Bytes::from("propertyB");
    bytes.put_u16(name.len() as u16);
    bytes.put(name);
    bytes.put_u8(2);
    bytes.put_f64(1.0);
    bytes.put_f64(2.0);
    bytes.put_f64(3.0);

    let name = Bytes::from("propertyC");
    bytes.put_u16(name.len() as u16);
    bytes.put(name);
    bytes.put_u8(3);
    bytes.put_f64(1.0);
    bytes.put_f64(2.0);
    bytes.put_f64(3.0);
    bytes.put_f64(4.0);

    let name = Bytes::from("propertyD");
    bytes.put_u16(name.len() as u16);
    bytes.put(name);
    bytes.put_u8(4);
    bytes.put_f64(1.0);
    bytes.put_f64(2.0);
    bytes.put_f64(3.0);
    bytes.put_f64(4.0);
    bytes.put_f64(5.0);
    bytes.put_f64(6.0);
    bytes.put_f64(7.0);
    bytes.put_f64(8.0);
    bytes.put_f64(9.0);
    bytes.put_f64(10.0);
    bytes.put_f64(11.0);
    bytes.put_f64(12.0);
    bytes.put_f64(13.0);
    bytes.put_f64(14.0);
    bytes.put_f64(15.0);
    bytes.put_f64(16.0);

    let controller = data::Controller {
        name: name!("my controller 🕹️"),
        properties: vec![
            data::Property::with_default_value(name!("propertyA"), 1.0.into()),
            data::Property::with_default_value(name!("propertyB"), (1.0, 2.0, 3.0).into()),
            data::Property::with_default_value(name!("propertyC"), (1.0, 2.0, 3.0, 4.0).into()),
            data::Property::with_default_value(
                name!("propertyD"),
                [
                    [1.0, 2.0, 3.0, 4.0],
                    [5.0, 6.0, 7.0, 8.0],
                    [9.0, 10.0, 11.0, 12.0],
                    [13.0, 14.0, 15.0, 16.0],
                ]
                .into(),
            ),
        ]
        .into_iter()
        .map(|item| (item.name.clone(), item))
        .collect(),
    };

    let parsed: messages::Init = (&mut QuicBytes::new(bytes.freeze()))
        .try_into()
        .expect("the init should be deserializable.");

    assert_eq_sorted!(parsed.peer, controller);
}
