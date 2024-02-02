use super::*;
use bytes::{BufMut, BytesMut};

#[tokio::test]
async fn test_frame_from_stream() {
    let mut bytes = BytesMut::new();
    bytes.put_u8(1);
    bytes.put_u8(0);
    bytes.put_u32(4);
    bytes.put_u16(0);
    bytes.put_u8(1);
    bytes.put_u8(2);
    bytes.put_u8(3);
    bytes.put_u8(4);
    let mut cursor = std::io::Cursor::new(bytes);

    let frame = Frame::from_stream(&mut cursor).await.unwrap();

    assert_eq!(frame.api_version, 1);
    assert_eq!(frame.kind, 0);
    assert_eq!(frame.payload_length, 4);
    assert_eq!(frame.payload, Bytes::from_static(&[1, 2, 3, 4]));
}

