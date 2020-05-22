use beatrix::Entity;
use beatrix_macro::Entity;

#[test]
pub fn test_default_entity_table_name() {
    #[derive(Entity)]
    struct User {}

    let user = User {};
    assert_eq!(user.table_name(), "user");
}

#[test]
pub fn test_custom_entity_table_name() {
    #[derive(Entity)]
    #[table_name = "user_table"]
    struct User {}

    let user = User {};
    assert_eq!(user.table_name(), "user_table");
}
