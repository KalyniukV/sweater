use sqlx::PgPool;
use uuid::Uuid;
use crate::data_store::AuthenticationStore;
use crate::domain::UserAuthentication;

pub struct PostgresAuthenticationStore {
    pool: PgPool
}

impl PostgresAuthenticationStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl AuthenticationStore for PostgresAuthenticationStore {
    async fn create_authentication(&mut self, user_id: &Uuid, password: &str) -> Result<(), String> {
        sqlx::query!(r#"
            INSERT INTO user_authentication (user_id, password)
            VALUES ($1, $2)"#,
            user_id,
            password
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_authentication(&self, user_id: &Uuid) -> Result<UserAuthentication, String> {
        sqlx::query_as!(
            UserAuthentication,
            "SELECT * FROM user_authentication WHERE user_id = $1",
            user_id
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}