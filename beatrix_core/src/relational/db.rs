use tokio_postgres::{Client, Error};

#[cfg(feature = "postgres")]
pub(crate) mod postgres;

pub enum DBType<'a> {
    Postgres(&'a Client),
}


#[async_trait::async_trait]
pub trait Database
// where
//     'a: 'static,
//     T: sqlx::Database,
//     E: SqlxExec<'a, Database = T>,
{
    /// Opening backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_OPEN: &'static str;
    /// Closing backtick character to surround identifiers, such as column and table names.
    const C_BACKTICK_CLOSE: &'static str;
    /// Wildcard character to be used in `LIKE` queries.
    const C_WILDCARD: &'static str;

    fn backtick_open(&self) -> &'static str {
        Self::C_BACKTICK_OPEN
    }

    fn backtick_close(&self) -> &'static str {
        Self::C_BACKTICK_CLOSE
    }

    fn wildcard(&self) -> &'static str {
        Self::C_WILDCARD
    }

    async fn new(connection_string: &str) -> Result<Self, Error>
    where
        Self: Sized;

    fn client(&self) -> DBType;
}

// #[async_trait]
// pub trait Executor<'a> {
//     // fn query(&self) -> &str;

//     async fn fetch_one<T, DB, SDB, E>(&self, db: &'static DB) -> Result<T>
//     where
//         SDB: sqlx::Database + sqlx::Database<Row = T>,
//         E: SqlxExec<'static, Database = SDB>,
//         DB: 'static,
//         DB: Database<'static, SDB, E> + std::marker::Sync,
//         <SDB as sqlx::database::HasArguments<'static>>::Arguments: sqlx::IntoArguments<'static, SDB>;
//         // R: sqlx::SqlxExec<'a> + sqlx::Database + sqlx::Database<Row = T>,
//         // <R as sqlx::database::HasArguments<'a>>::Arguments: sqlx::IntoArguments<'a, R>
// }
