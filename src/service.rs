use crate::client::ClientManager;
use futures::Stream;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::{api, proto, runtime};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status};

type DefaultHandler = runtime::Handle<runtime::DefaultCommand, runtime::DefaultCommand>;

pub struct IndieMotionService {
    runtime: DefaultHandler,
}

impl IndieMotionService {
    pub fn new(runtime: DefaultHandler) -> Self {
        Self { runtime }
    }
}

#[tonic::async_trait]
impl proto::indie_motion_service_server::IndieMotionService for IndieMotionService {
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
        let channel = self.runtime.connect_as(client_info.into()).await.unwrap(); //FIXME: handle error

        // let client_manager = self.client_manager.lock().await;
        // let Ok(uid) = Uuid::parse_str(&client_info.id) else {
        //     return Err(Status::invalid_argument("invalid client id, must be uuidv4"));
        // };

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
        // Ok(Response::new(Box::pin(stream) as Self::ConnectAsStream))
        Err(tonic::Status::unimplemented("not implemented"))
    }
}
