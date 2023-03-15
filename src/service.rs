use futures::Stream;
use std::collections::HashMap;
use std::pin::Pin;
use tokio_stream::wrappers::errors::BroadcastStreamRecvError;
use tokio_stream::StreamExt;

use crate::{api, engine, proto, runtime};
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
        Pin<Box<dyn Stream<Item = std::result::Result<proto::ContextUpdate, Status>> + Send>>;

    async fn connect_as(
        &self,
        request: Request<proto::ConnectAsRequest>,
    ) -> Result<Response<Self::ConnectAsStream>, Status> {
        let request = request.into_inner();

        let Some(client_info) = request.client_info else {
            return Err(Status::invalid_argument("client_info is required"));
        };
        let stream = match self.runtime.connect_as(client_info.into()).await {
            Ok(stream) => stream,
            Err(err) => return Err(err.into()),
        };

        let stream = stream.filter_map(|result| match result {
            Ok(item) => {
                tracing::info!("sending update to client");
                let item: proto::ContextUpdate = item.into();
                Some(Ok(item))
            }
            Err(error) => match error {
                BroadcastStreamRecvError::Lagged(_) => {
                    tracing::warn!("client lagged");
                    None
                }
            },
        });
        Ok(Response::new(Box::pin(stream) as Self::ConnectAsStream))
        // Err(tonic::Status::unimplemented("not implemented"))
    }
}
