use crate::orm::{query::BindType, querybuilder::{MySQLCondition, QueryBuilder}, Orm};
use sqlx::Row;

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct User {
    id: i32,
    username: String,
}

impl User {
    pub fn new(id: i32, username: String) -> Self {
        Self { id, username }
    }

    pub async fn create_user(&self) -> Result<Self, sqlx::Error> {
        let orm = Orm::new("users").await;
        let mut query = orm.query_builder();
        query.insert(
            vec!["username".to_string()],
            vec![BindType::String(self.username.clone())],
        );
        let mut query = orm.query(query);
        query.bind(BindType::String(self.username.clone()));
        let result = orm.execute(query).await;
        match result {
            Ok(r) => Ok(Self {
                id: r.last_insert_id() as i32,
                username: self.username.clone(),
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn get_user_by_id(id: u32) -> Result<Self, sqlx::Error> {
        let orm = Orm::new("users").await;
        let mut query = orm.query_builder();
        query.select(vec!["id".to_string(), "username".to_string()]);
        query.where_condition("id", MySQLCondition::Equal(BindType::UInt(id)));
        println!("{}", query.build());
        let mut query = orm.query(query);
        query.bind(BindType::UInt(id));
        println!("{:?}", query.get_bind_values());
        let result = orm.fetch_one(query).await;
        match result {
            Ok(row) => Ok(Self {
                id: row.get("id"),
                username: row.get("username"),
            }),
            Err(e) => Err(e),
        }
    }

    pub async fn get_all_users() -> Result<Vec<Self>, sqlx::Error> {
        let orm = Orm::new("users").await;
        let mut query = orm.query_builder();
        query.select(vec![]);
        println!("{}", query.build());
        let query = orm.query(query);
        let result = orm.fetch_all(query).await;
        match result {
            Ok(rows) => {
                let mut users = Vec::new();
                for row in rows {
                    users.push(Self {
                        id: row.get("id"),
                        username: row.get("username"),
                    });
                }
                Ok(users)
            }
            Err(e) => Err(e),
        }
    }
}
