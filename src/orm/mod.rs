pub mod query;
pub mod querybuilder;

use query::Query;
use querybuilder::QueryBuilder;
use sqlx::MySqlPool;

use crate::database;

pub struct Orm {
    pool: MySqlPool,
    table: String,
}

impl Orm {
    pub async fn new(table: &str) -> Self {
        Self {
            pool: database::get_pool()
                .await
                .expect("Failed to connect to database"),
            table: table.to_string(),
        }
    }

    pub async fn query_statement(
        &self,
        query: &str,
    ) -> sqlx::Result<sqlx::mysql::MySqlQueryResult> {
        let q = Query::new(QueryBuilder::custom_query(query));
        return q.execute(&self.pool).await;
    }

    pub fn query_builder(&self) -> QueryBuilder {
        QueryBuilder::new(&self.table)
    }

    pub fn query(&self, query: QueryBuilder) -> Query {
        Query::new(query)
    }

    
    pub async fn fetch_all(&self, query: Query) -> sqlx::Result<Vec<sqlx::mysql::MySqlRow>> {
        query.fetch_all(&self.pool).await
    }

    pub async fn fetch_one(&self, query: Query) -> sqlx::Result<sqlx::mysql::MySqlRow> {
        query.fetch_one(&self.pool).await
    }

    pub async fn execute(&self, query: Query) -> sqlx::Result<sqlx::mysql::MySqlQueryResult> {
        query.execute(&self.pool).await
    }
}
