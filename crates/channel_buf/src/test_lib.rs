use super::*;

#[tokio::test]
async fn test_channel_buffer_lifetime() {
    let (tx, rx) = mpsc::channel(1);
    let mut buf = ChannelBuffer::new(rx, 5);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    tx.send(3).await.unwrap();

    // Small wait to ensure that the buffer is filled
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let flush = buf.flush().await;
    assert_eq!(flush, vec![1, 2, 3]);

    tx.send(4).await.unwrap();
    tx.send(5).await.unwrap();
    tx.send(6).await.unwrap();

    // Small wait to ensure that the buffer is filled
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let flush = buf.flush().await;
    assert_eq!(flush, vec![4, 5, 6]);

    tx.send(4).await.unwrap();
    tx.send(5).await.unwrap();
    tx.send(6).await.unwrap();

    // Small wait to ensure that the buffer is filled
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let flush = buf.flush().await;
    assert_eq!(flush, vec![4, 5, 6]);

    tx.send(1).await.unwrap();
    tx.send(2).await.unwrap();
    tx.send(3).await.unwrap();
    tx.send(4).await.unwrap();
    tx.send(5).await.unwrap();
    tx.send(6).await.unwrap();
    tx.send(7).await.unwrap();

    // Small wait to ensure that the buffer is filled
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    let flush = buf.flush().await;
    assert_eq!(flush.len(), 5);
}
