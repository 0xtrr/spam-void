use std::net::SocketAddr;

use bb8_postgres::PostgresConnectionManager;
use log::{debug, error, info};
use nauthz_grpc::authorization_server::{Authorization, AuthorizationServer};
use nauthz_grpc::{Decision, EventReply, EventRequest, Event};
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};

pub struct EventAuthz {
    connection_pool: bb8::Pool<PostgresConnectionManager<NoTls>>,
}

pub mod nauthz_grpc {
    tonic::include_proto!("nauthz");
}

#[tonic::async_trait]
impl Authorization for EventAuthz {
    async fn event_admit(
        &self,
        request: Request<EventRequest>,
    ) -> Result<Response<EventReply>, Status> {
        let req = request.into_inner();
        let event = req.clone().event.unwrap();
        let content_sample: String = event.content.chars().take(40).collect();

        let author = match req.auth_pubkey {
            Some(_) => req.auth_pubkey(),
            None => &event.pubkey,
        };

        let author = hex::encode(author);

        debug!("recvd event, [kind={}, origin={:?}, author={:?}, tag_count={}, content_sample={:?}]",
                    event.kind, req.origin, author, event.tags.len(), content_sample);

        let mut conn = self.connection_pool.get().await.unwrap();

        let is_blacklisted = conn
            .query(
                "SELECT EXISTS(SELECT 1 FROM blacklisted_words WHERE word = $1)",
                &[&event.content],
            )
            .await
            .unwrap();

        let is_blacklisted: bool = is_blacklisted[0].get(0);

        if is_blacklisted {
            info!("event denied, content=[{:?}]", content_sample);
            Ok(Response::new(nauthz_grpc::EventReply {
                decision: Decision::Deny as i32,
                message: Some("Event denied".to_string()),
            }))
        } else {
            return Ok(Response::new(nauthz_grpc::EventReply {
                decision: Decision::Permit as i32,
                message: Some("Ok".to_string()),
            }));
        }
    }
}

async fn create_pool() -> bb8::Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new(
        "host=localhost user=elliot password=elliot dbname=spamvoid"
            .parse()
            .unwrap(),
        NoTls,
    );

    bb8::Pool::builder()
        .build(manager)
        .await
        .expect("failed to create connection pool")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logger
    pretty_env_logger::init();

    let addr: SocketAddr = "[::1]:50051".parse().unwrap();

    let connection_pool = create_pool().await;

    let ev_auth = EventAuthz { connection_pool };
    // Start serving
    Server::builder()
        .add_service(AuthorizationServer::new(ev_auth))
        .serve(addr)
        .await?;

    Ok(())
}
