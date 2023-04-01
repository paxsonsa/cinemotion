use indiemotion_api as api;
use indiemotion_proto as proto;
use tonic::transport::Uri;
use uuid::Uuid;

use crate::{CLIResult, Error};

pub struct ContextBuilder {
    addr: Option<Uri>,
}

impl ContextBuilder {
    pub fn with_addr(mut self, addr: Uri) -> Self {
        self.addr = Some(addr);
        self
    }

    pub(crate) async fn build(self) -> Result<Context, Error> {
        let mut ctx = Context {
            uid: Uuid::new_v4(),
            name: hostname::get().unwrap().to_string_lossy().to_string(),
            role: api::ClientRole::PrimaryController,
            address: self.addr.clone(),
            client: None,
            main_loop: None,
        };

        if ctx.address.is_some() {
            ctx.connect().await?;
        }

        Ok(ctx)
    }
}

pub struct Context {
    pub uid: Uuid,
    pub name: String,
    pub role: api::ClientRole,
    pub address: Option<Uri>,
    pub client: Option<
        proto::indie_motion_service_client::IndieMotionServiceClient<tonic::transport::Channel>,
    >,
    pub main_loop: Option<tokio::task::JoinHandle<()>>,
}

impl Context {
    pub fn builder() -> ContextBuilder {
        ContextBuilder { addr: None }
    }

    pub(crate) async fn connect(&mut self) -> CLIResult<()> {
        let address = self.address.clone().unwrap_or_else(|| {
            let uri = format!("http://127.0.0.1:{}", indiemotion::DEFAULT_GRPC_PORT);
            let uri: Uri = uri.parse().unwrap();
            self.address = Some(uri.clone());
            uri
        });

        self.client = match proto::indie_motion_service_client::IndieMotionServiceClient::connect(
            address.clone(),
        )
        .await
        {
            Ok(client) => Some(client),
            Err(err) => {
                tracing::error!("Failed to connect to server {}: {}", address, err);
                None
            }
        };
        Ok(())
    }
}