use tokio_postgres::{tls::NoTlsStream, Client, Connection, Error, NoTls, Socket};

use super::{DBType, Database};

pub struct Postgres(Client);

#[async_trait::async_trait]
impl Database for Postgres {
    /// Opening backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_OPEN: &'static str = "\"";
    /// Closing backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_CLOSE: &'static str = "\"";
    /// Wildcard character to be used in `LIKE` queries.
    const C_WILDCARD: &'static str = "%";

    async fn new(connection_string: &str) -> Result<Self, Error> {
        let (client, connection) = tokio_postgres::connect(connection_string, NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        Ok(Self(client))
    }

    fn client(&self) -> DBType {
        DBType::Postgres(&self.0)
    }
}
