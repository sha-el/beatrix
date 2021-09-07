use beatrix::rdms::{select::Select, QueryBuilder};
use beatrix::rdms::MySql;

#[test]
fn simple_select_statement() {
    let (query, _) = MySql::build(Select::new().and_select("*".into()).and_from("sqlx")).unwrap();

    assert_eq!(query, "SELECT * FROM sqlx;".to_string());
}
#[test]
fn select_with_table_alias() {
    let (query, _) = MySql::build(
        Select::new()
            .and_select("*".into())
            .and_from(("sqlx", "s", "sqlx_db")),
    )
    .unwrap();
    assert_eq!(query, "SELECT * FROM sqlx_db.sqlx as s;");
}

#[test]
fn select_statement_with_column_names() {
    let (query, _) = MySql::build(Select::new().and_select("name".into()).from("sqlx")).unwrap();

    assert_eq!(query, "SELECT name FROM sqlx;".to_string());
}

#[test]
fn select_statement_with_simple_conditions() {

}
