use crate::orm::{query::BindType, Orm};

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
        let mut query = orm.query("INSERT INTO users (id, username) VALUES (?, ?)");
        query.bind(BindType::Int(self.id as i32));
        query.bind(BindType::String(self.username.clone()));
        let result = orm.execute(query).await;
        match result {
            Ok(_) => Ok(Self { id: self.id, username: self.username.clone() }),
            Err(e) => Err(e),
            
        }
    }

    pub async fn update_user(&self) -> Result<Self, sqlx::Error> {
        let orm = Orm::new("users").await;
        let mut query = orm.query("UPDATE users SET username = ? WHERE id = ?");
        query.bind(BindType::String(self.username.clone()));
        query.bind(BindType::Int(self.id as i32));
        let resul = orm.execute(query).await;
        match resul {
            Ok(_) => Ok(Self { id: self.id, username: self.username.clone() }),
            Err(e) => Err(e),
        }


    }
    
}