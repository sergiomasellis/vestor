use crate::configuration::{Settings};
use crate::routes::{health_check};
use actix_web::dev::Server;
// use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use std::net::TcpListener;
use tracing_actix_web::TracingLogger;


pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        // let connection_pool = get_connection_pool(&configuration.database)
        //     .await
        //     .expect("Failed to connect to Postgres.");

        // let sender_email = configuration
        //     .email_client
        //     .sender()
        //     .expect("Invalid sender email address.");
        // let email_client = EmailClient::new(
        //     configuration.email_client.base_url,
        //     sender_email,
        //     configuration.email_client.authorization_token,
        // );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(&address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(
            listener,
            // connection_pool,
            // email_client,
            configuration.application.base_url,
        )?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub struct ApplicationBaseUrl(pub String);

fn run(
    listener: TcpListener,
    // db_pool: PgPool,
    // email_client: EmailClient,
    base_url: String,
) -> Result<Server, std::io::Error> {
    // let db_pool = Data::new(db_pool);
    // let email_client = Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .data(ApplicationBaseUrl(base_url.clone()))
    })
        .listen(listener)?
        .run();
    Ok(server)
}
