use beatrix::relational::db::Database;
use beatrix::relational::filters::{IntFilters, StringFilters};
use beatrix::relational::table::Table;
use beatrix::relational::Postgres;
use beatrix_macro::RelationalModel;
use serde::{Deserialize, Serialize};

#[test]
pub fn test_default_name() {
    #[derive(Debug, Clone, RelationalModel)]
    struct SomeComplexStructName {
        id: i32,
        name: String,
    }

    assert_eq!(
        SomeComplexStructName::table_details().table_name(),
        "some_complex_struct_name"
    );
}

#[test]
pub fn test_custom_enitity_name() {
    #[derive(Debug, Clone, RelationalModel)]
    #[table_name = "user"]
    struct UserInsert {
        id: i32,
        name: String,
    }

    assert_eq!(UserInsert::table_details().table_name(), "user");
}

#[test]
pub fn test_field_name() {
    #[derive(Debug, Clone, RelationalModel)]
    struct TableXyz {
        id: i32,
        name: String,
    }

    let fields = User::fields()
        .iter()
        .map(|v| v.field_name())
        .collect::<Vec<String>>();

    assert_eq!(fields, vec!["id", "name"]);
}

#[test]
pub fn test_custom_field_name() {
    #[derive(Debug, Clone, RelationalModel)]
    struct TableXyz {
        id: i32,
        #[name = "custom_name"]
        complex_field_name: String,
    }

    let fields = User::fields()
        .iter()
        .map(|v| v.field_name())
        .collect::<Vec<String>>();

    assert_eq!(fields, vec!["id", "custom_name"]);
}

#[derive(Serialize, Deserialize, Debug, Clone, RelationalModel, PartialEq, Eq)]
struct User {
    id: i32,
    name: String,
    email: String,
    gender: String,
    // dob: String,
    salary: i32,
}

#[tokio::test]
pub async fn test_simple_select() {
    let db = Postgres::new("host=localhost user=anit.nilay dbname=testing")
        .await
        .unwrap();

    let user: Vec<User> = User::select().fetch_all(&db).await.unwrap();

    assert_eq!(
        user[0],
        User {
            id: 1,
            name: "Waldo Brookzie".to_string(),
            email: "wbrookzie0@nationalgeographic.com".to_string(),
            gender: "Agender".to_string(),
            // dob: "1997-04-01 08:58:44".to_string(),
            salary: 78818,
        }
    )
}

#[tokio::test]
pub async fn test_eq_filters() {
    let db = Postgres::new("host=localhost user=anit.nilay dbname=testing")
        .await
        .unwrap();

    let user: User = User::select()
        .filter(User::id().eq(1))
        .fetch_one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.id, 1,);

    let user: User = User::select()
        .filter(User::email().eq("kfundell2@tiny.cc".into()))
        .fetch_one(&db)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(user.email, "kfundell2@tiny.cc");
}

#[tokio::test]
pub async fn test_like_filters() {
    let db = Postgres::new("host=localhost user=anit.nilay dbname=testing")
        .await
        .unwrap();

    let user: Vec<User> = User::select()
        .filter(User::email().like("%@live.com".to_string()))
        .fetch_all(&db)
        .await
        .unwrap();

    assert_eq!(
        user.iter()
            .map(|f| f.email.clone())
            .filter(|f| !f.contains("@live.com"))
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );

    let user: Vec<User> = User::select()
        .filter(User::email().ilike("%@live.com".to_string()))
        .fetch_all(&db)
        .await
        .unwrap();

    println!("{:#?}", user);

    assert_eq!(
        user.iter()
            .map(|f| f.email.clone())
            .filter(|f| !f.to_uppercase().contains("@LIVE.COM"))
            .collect::<Vec<String>>(),
        Vec::<String>::new()
    );
}
