use crate::orm::{query::BindType, querybuilder::{MySQLJoin, QueryBuilder}};

//  to test basic select
#[test]
fn test_basic_select_query_builder() {
    let mut query_builder = QueryBuilder::new("users");
    let query = query_builder.select(vec!["id".to_string(), "username".to_string()]).build();
    println!("{}", query);
    assert_eq!(query, "SELECT id, username FROM users");
}
// to test insert query
#[test]
fn test_insert_query_builder() {
    let mut query_builder = QueryBuilder::new("users");
    query_builder.prepared_stmt(false);
    let query = query_builder.insert(vec![ "id".to_string(), "username".to_string()], vec![BindType::Int(1), BindType::String("test".to_string())]).build();
    println!("{}", query);
    assert_eq!(query, "INSERT INTO users (id, username) VALUES (1, 'test')");
}

#[test]
fn test_select_query_builder_with_join() {
    let mut query_builder = QueryBuilder::new("users");
    let query = query_builder.select(vec!["id".to_string(), "username".to_string()])
        .join("roles".to_string(), MySQLJoin::Inner, "users.id = roles.user_id".to_string())
        .build();
    println!("{}", query);
    assert_eq!(query, "SELECT id, username FROM users INNER JOIN roles ON users.id = roles.user_id");
}
