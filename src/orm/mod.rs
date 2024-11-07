pub mod query;
pub mod querybuilder;

use query::Query;
use sqlx::MySqlPool;

use crate::database;

pub struct Orm {
    pool: MySqlPool,
    table: String,
}



impl Orm {
    pub async  fn new(table: &str) -> Self {
        Self { pool : database::get_pool().await.expect("Failed to connect to database") , table : table.to_string() }   
    }

    pub fn query(&self, query: &str) -> Query {
        Query::new(query.to_string())
    }


    pub async fn execute(&self, query: Query) -> sqlx::Result<sqlx::mysql::MySqlQueryResult> {
        let stmt = query.ready_query();
        return stmt.execute(&self.pool).await;
    }


}