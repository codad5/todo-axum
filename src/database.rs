use sqlx::MySqlPool;
pub async fn get_pool() -> Result<MySqlPool, sqlx::Error> {
    MySqlPool::connect("mysql://root:@localhost:3306/test").await
}

