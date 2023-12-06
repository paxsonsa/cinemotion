use tokio::time::Duration;
use anyhow::Result;
use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use std::sync::Arc;
use webrtc::{
    api::{media_engine::MediaEngine, APIBuilder},
    data_channel::{data_channel_init::RTCDataChannelInit, RTCDataChannel},
    peer_connection::{
        configuration::RTCConfiguration, peer_connection_state::RTCPeerConnectionState, sdp::session_description::RTCSessionDescription,
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    let m = MediaEngine::default();
    let api = APIBuilder::new().with_media_engine(m).build();

    let config = RTCConfiguration {
        ..Default::default()
    };

    let peer_connection = Arc::new(api.new_peer_connection(config).await?);

    let options = RTCDataChannelInit {
        ordered: Some(true),
        ..Default::default()
    };

    let (done_tx, mut done_rx) = tokio::sync::mpsc::channel::<()>(1);

    // Setup a connection state listener
    peer_connection.on_peer_connection_state_change(Box::new(move |s: RTCPeerConnectionState| {
        println!("Peer Connection State has changed: {s}");

        if s == RTCPeerConnectionState::Failed {
            // Wait until PeerConnection has had no network activity for 30 seconds or another failure. It may be reconnected using an ICE Restart.
            // Use webrtc.PeerConnectionStateDisconnected if you are interested in detecting faster timeout.
            // Note that the PeerConnection may come back from PeerConnectionStateDisconnected.
            println!("Peer Connection has gone to failed exiting");
            let _ = done_tx.try_send(());
        }

        Box::pin(async {})
    }));

    peer_connection.on_data_channel(Box::new(move |new_channel| {
        let id = new_channel.id();
        let label = new_channel.label().to_owned(); 
        println!("New DataChannel {label}::{id}");

        Box::pin(async move {
            let channel = Arc::clone(&new_channel);
            let channel_label = label.clone();
            let channel_id = id;

            new_channel.on_close(Box::new(move || {
                println!("Data channel closed up");
                Box::pin(async {})
            }));

            new_channel.on_open(Box::new(move || {
                println!("Data channel '{channel_label}'-'{channel_id}' open. Random messages will now be sent to any connected DataChannels every second");

                Box::pin(async move {
                    let mut result = Result::<usize>::Ok(0);

                    while result.is_ok() {
                        let timeout = tokio::time::sleep(Duration::from_secs(1));
                        tokio::pin!(timeout);

                        tokio::select! {
                            _ = timeout.as_mut() => {
                                let message = "hello, world!".to_string();
                                result = channel.send_text(message).await.map_err(Into::into);
                            }
                        }
                    }
                })
            }));

            new_channel.on_message(Box::new(move |message| {
                let msg_str = String::from_utf8(message.data.to_vec()).unwrap();
                println!("Message from DataChannel '{label}': '{msg_str}'");
                Box::pin(async {})
            }));
        })

    }));

    let line = must_read_stdin()?;
    let desc_data = decode(line.as_str())?;
    println!("{desc_data}");
    let offer = RTCSessionDescription::offer(desc_data)?;
    // let offer = serde_json::from_str::<RTCSessionDescription>(&desc_data)?;
    
    // Set the remote SessionDescription
    peer_connection.set_remote_description(offer).await?;

    // Create an answer
    let answer = peer_connection.create_answer(None).await?;

    // Create channel that is blocked until ICE Gathering is complete
    let mut gather_complete = peer_connection.gathering_complete_promise().await;

    // Sets the LocalDescription, and starts our UDP listeners
    peer_connection.set_local_description(answer).await?;

    // Block until ICE Gathering is complete, disabling trickle ICE
    // we do this because we only can exchange one signaling message
    // in a production application you should exchange ICE Candidates via OnICECandidate
    let _ = gather_complete.recv().await;

    // Output the answer in base64 so we can paste it in browser
    if let Some(local_desc) = peer_connection.local_description().await {
        // let json_str = serde_json::to_string(&local_desc)?;
        let b64 = encode(&local_desc.sdp);
        println!("{b64}");
    } else {
        println!("generate local_description failed!");
    }

    println!("Press ctrl-c to stop");
    tokio::select! {
        _ = done_rx.recv() => {
            println!("received done signal!");
        }
        _ = tokio::signal::ctrl_c() => {
            println!();
        }
    };

    peer_connection.close().await?;

    Ok(())
}

/// must_read_stdin blocks until input is received from stdin
pub fn must_read_stdin() -> Result<String> {
    let mut line = String::new();

    std::io::stdin().read_line(&mut line)?;
    line = line.trim().to_owned();
    println!();

    Ok(line)
}

// Allows compressing offer/answer to bypass terminal input limits.
// const COMPRESS: bool = false;

/// encode encodes the input in base64
/// It can optionally zip the input before encoding
pub fn encode(b: &str) -> String {
    //if COMPRESS {
    //    b = zip(b)
    //}

    BASE64_STANDARD_NO_PAD.encode(b)
}

/// decode decodes the input from base64
/// It can optionally unzip the input after decoding
pub fn decode(s: &str) -> Result<String> {
    let b = BASE64_STANDARD_NO_PAD.decode(s)?;

    //if COMPRESS {
    //    b = unzip(b)
    //}

    let s = String::from_utf8(b)?;
    Ok(s)
}
