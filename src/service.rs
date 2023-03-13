use futures::Stream;
use std::collections::HashMap;
use std::pin::Pin;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::StreamExt;

use crate::{api, proto, runtime};
use tonic::{Request, Response, Status};

pub struct IndieMotionService {
    runtime: runtime::Handle,
}

impl IndieMotionService {
    pub fn new(runtime: runtime::Handle) -> Self {
        Self { runtime }
    }
}

#[tonic::async_trait]
impl proto::indie_motion_service_server::IndieMotionService for IndieMotionService {
    async fn ping(
        &self,
        request: tonic::Request<proto::PingRequest>,
    ) -> Result<Response<proto::PingResponse>, Status> {
        let request = request.into_inner();
        let server_ts = chrono::Utc::now().timestamp_millis();
        let runtime_ts = match self.runtime.ping().await {
            Ok(ts) => ts,
            Err(err) => return Err(err.into()),
        };

        let response = proto::PingResponse {
            client_timestamp: request.timestamp,
            server_timestamp: server_ts,
            runtime_timestamp: runtime_ts,
        };

        Ok(Response::new(response))
    }

    async fn server_info(
        &self,
        _request: tonic::Request<proto::ServerInfoRequest>,
    ) -> Result<Response<proto::ServerInfoResponse>, Status> {
        let response = proto::ServerInfoResponse {
            name: "IndieMotion".to_string(),
            version: "0.1.0".to_string(),
            clients: HashMap::new(),
        };
        Ok(Response::new(response))
    }

    type ConnectAsStream =
        Pin<Box<dyn Stream<Item = std::result::Result<proto::ConnectAsResponse, Status>> + Send>>;

    async fn connect_as(
        &self,
        request: Request<proto::ConnectAsRequest>,
    ) -> Result<Response<Self::ConnectAsStream>, Status> {
        let request = request.into_inner();

        let Some(client_info) = request.client_info else {
            return Err(Status::invalid_argument("client_info is required"));
        };
        let stream = self
            .runtime
            .connect_as(client_info.into())
            .await
            .unwrap()
            .filter_map(|item| match item {
                Ok(_) => {
                    tracing::info!("sending update to client");
                    Some(Ok(proto::ConnectAsResponse {}))
                }
                Err(error) => match error {
                    BroadcastStreamRecvError::Lagged(_) => {
                        tracing::warn!("client lagged");
                        None
                    }
                    _ => {
                        tracing::error!("unknown error occured while updating client");
                        None
                    }
                },
            });

        //FIXME: handle error

        // if let Some(_) = client_manager.get(uid) {
        //     return Err(Status::already_exists("client already exists"));
        // }

        // let Some(role) = proto::ClientRole::from_i32(client_info.role) else {
        //     return Err(Status::invalid_argument("invalid client role"));
        // };
        // drop(client_manager);

        // println!("openning channel");
        // let (tx, rx) = mpsc::channel(128);

        // let client = api::Client {
        //     meta: meta.clone(),
        //     relay: None, // TODO: Remove Relay
        // };

        // println!("updating runtime");
        // let mut runtime = self.runtime.lock().await;
        // println!("adding client");
        // {
        //     let mut client_manager = self.client_manager.lock().await;
        //     client_manager.add(client);
        // }
        // println!("updating runtime clients");
        // if let Err(err) = runtime.add_client(meta).await {
        //     let err = match err {
        //         crate::Error::InvalidRecordingOperation(e) => Err(Status::failed_precondition(e)),
        //         _ => Err(Status::internal(err.to_string())),
        //     };
        //     {
        //         let mut client_manager = self.client_manager.lock().await;
        //         client_manager.remove(uid);
        //     }
        //     return err;
        // }
        // println!("creating stream");
        // let stream = ReceiverStream::new(rx).map(|_: StateUpdate| Ok(proto::ConnectAsResponse {}));
        // println!("done.");
        Ok(Response::new(Box::pin(stream) as Self::ConnectAsStream))
        // Err(tonic::Status::unimplemented("not implemented"))
    }
}
