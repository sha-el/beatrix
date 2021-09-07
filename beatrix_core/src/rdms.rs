use select::Select;
use table::Table;

pub mod error;
pub mod mysql;
pub mod query;
pub mod select;
pub mod table;

pub use sqlx::mysql::MySql;

pub trait QueryBuilder {
    const SYSTEM_IDENTIFIER_START: &'static str;
    const SYSTEM_IDENTIFIER_END: &'static str;

    fn build<T>(query: T) -> error::Result<(String, Vec<String>)>
    where
        T: Into<query::QueryType>;

    fn build_select(select: Select) -> error::Result<(String, Vec<String>)>;
    fn build_table(table: Table) -> error::Result<String>;
}
