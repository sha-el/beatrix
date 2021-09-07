use beatrix_macro::RelationalModel;

#[derive(Debug, Clone, RelationalModel)]
struct User {
    id: i32,
    name: String,
}

#[test]
pub fn test_default_entity_name() {
    let user = User {
        id: 1,
        name: "hello".into(),
    };
}