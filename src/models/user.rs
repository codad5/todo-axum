use crate::orm::{query::BindType, Orm, querybuilder::QueryBuilder};

pub struct User {
    id: u32,
    username: String,
}

impl User {
    pub fn new(id: u32, username: String) -> Self {
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
                id: r.last_insert_id() as u32,
                username: self.username.clone(),
            }),
            Err(e) => Err(e),
        }
    }

    // pub async fn update_user(&self) -> Result<Self, sqlx::Error> {
    //     // let orm = Orm::new("users").await;
    //     // let mut query = orm.query_builder();
    //     // query.update(
    //     //     vec!["username".to_string()],
    //     //     vec![BindType::String(self.username.clone())],
    //     // );
    //     // query.where_clause("id".to_string(), BindType::Int(self.id));
    //     // let mut query = orm.query(query);
    //     // query.bind(BindType::String(self.username.clone()));
    //     // query.bind(BindType::Int(self.id));
    //     // let result = orm.execute(query).await;
    //     // match result {
    //     //     Ok(_) => Ok(Self {
    //     //         id: self.id,
    //     //         username: self.username.clone(),
    //     //     }),
    //     //     Err(e) => Err(e),
    //     // }
    // }
}
