use super::db::Database;

pub trait Sql {
    fn to_sql<DB>(self, db: &DB) -> String where DB: Database;
}