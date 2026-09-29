use clap::{Parser};
use pgiceberg::config::Config;
use pgiceberg::service::peek_changes;
use tracing::info;
use tracing_subscriber::EnvFilter;
use sqlx::postgres::PgPoolOptions;
#[derive(Parser)]
#[command(version)]
struct Cli {
    group: String,

    #[arg(short, long, global = true)]
    debug: bool,

    // todo config by cli
}
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::parse();
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| EnvFilter::new(if args.debug { "debug" } else { "info" })),
        )
        .init();
    info!(group = ?args.group, debug = args.debug, "Starting program");
    info!("Loading configuration...");
    let config = Config::from_env_or_file(Some(args.debug));
    info!("Configuration loaded.");
    info!("Connecting to pg database...");
    let pool = PgPoolOptions::new()
        .min_connections(1)
        .max_connections(5)
        .connect(&config.pg_connection_string)
        .await?;
    info!("Database connection established.");
    peek_changes(&pool).await?;
    info!("Program finished.");
    Ok(())
}
