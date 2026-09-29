use sqlx::PgPool;
use tracing::info;
use sqlx::FromRow;
use crate::model::PgOutput;
#[derive(Debug, Clone, FromRow)]
struct Change {
    lsn: String, // high/low format
    xid: String,
    data: Vec<u8>,
}

pub async fn peek_changes(pool: &PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let rows: Vec<Change> = sqlx::query_as(
        "SELECT lsn::text AS lsn, xid::text AS xid, data
         FROM pg_logical_slot_peek_binary_changes(
             'datalake_slot', NULL, NULL,
             'proto_version', '1',
             'publication_names', 'datalake_pub')",
    )
    .fetch_all(pool)
    .await?;


    for row in rows {
        let data = PgOutput::from_binary(&row.data, false);

        info!("{:?}", data);
    }
    Ok(())
}