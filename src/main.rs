mod settings;

use std::net::SocketAddr;

use bb8_postgres::PostgresConnectionManager;
use nauthz_grpc::authorization_server::{Authorization, AuthorizationServer};
use nauthz_grpc::{Decision, Event, EventReply, EventRequest};
use settings::Settings;
use tokio_postgres::NoTls;
use tonic::{transport::Server, Request, Response, Status};
use tracing::{debug, info};

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
        let event: Event = req.clone().event.unwrap();
        let content_sample: String = event.content.chars().take(40).collect();

        let author = hex::encode(event.pubkey);

        info!(
            "recvd event, [kind={}, origin={:?}, author={:?}, tag_count={}, content_sample={:?}]",
            event.kind,
            req.origin,
            author,
            event.tags.len(),
            content_sample
        );

        // Fetch DB connection from connection pool
        let conn = self.connection_pool.get().await.unwrap();

        // ================================
        // Check if pubkey is blacklisted
        // ================================
        debug!("checking if publick key is blacklisted [pubkey={}]", author);
        let author_blacklisted = conn
            .query(
                "SELECT EXISTS(SELECT 1 from blacklisted_pubkeys WHERE pubkey = $1)",
                &[&author],
            )
            .await
            .unwrap();
        if author_blacklisted[0].get(0) {
            info!("event denied: blacklisted pubkey [pubkey={}]", author);
            return Ok(Response::new(nauthz_grpc::EventReply {
                decision: Decision::Deny as i32,
                message: Some("Event denied".to_string()),
            }));
        }

        // ===============================================
        // Check if content contains blacklisted content
        // ===============================================
        let contains_blacklisted_content = conn
            .query(
                "SELECT EXISTS(SELECT 1 FROM blacklisted_words WHERE word = $1)",
                &[&event.content],
            )
            .await
            .unwrap();

        if contains_blacklisted_content[0].get(0) {
            info!(
                "event denied, blacklisted content [content={:?}]",
                content_sample
            );
            return Ok(Response::new(nauthz_grpc::EventReply {
                decision: Decision::Deny as i32,
                message: Some("Event denied".to_string()),
            }));
        }

        // None of the previous checks triggered, permit event
        debug!("event permitted");

        return Ok(Response::new(nauthz_grpc::EventReply {
            decision: Decision::Permit as i32,
            message: Some("Ok".to_string()),
        }));
    }
}

async fn create_pool(connection_string: String) -> bb8::Pool<PostgresConnectionManager<NoTls>> {
    let manager = PostgresConnectionManager::new(
        connection_string
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
    info!("Starting SpamVoid");
    // initialize settings
    let settings = Settings::new()?;

    // set up tokio tracing subscriber
    let subscriber = tracing_subscriber::FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)?;

    // set up DB connection
    info!("Connecting to the database");
    let host = settings.database.host;
    let user = settings.database.user;
    let password = settings.database.password;
    let db_name = settings.database.database_name;
    let connection_string: String = format!(
        "host={host} user={user} password={password} dbname={db_name}"
    );
    let connection_pool = create_pool(connection_string).await;
    info!("Database connected");

    let ev_auth = EventAuthz { connection_pool };

    // start serving the GRPC service
    let addr: SocketAddr = "[::1]:50051".parse().unwrap();
    Server::builder()
        .add_service(AuthorizationServer::new(ev_auth))
        .serve(addr)
        .await?;

    Ok(())
}
