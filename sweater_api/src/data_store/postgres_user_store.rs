use sqlx::PgPool;
use crate::data_store::UserStore;
use crate::domain::User;

pub struct PostgresUserStore {
    pool: PgPool
}

impl PostgresUserStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserStore<> for PostgresUserStore {
    async fn create_user(&mut self, user: &User) -> Result<(), String> {
        let user_from_db = self.get_user(&user.email).await;
        if user_from_db.is_ok() {
            return Err(format!("User already exists: {}", user.email));
        }

        sqlx::query!(r#"
            INSERT INTO users (id, username, email)
            VALUES ($1, $2, $3)
            "#,
            &user.id,
            &user.username,
            &user.email
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    }

    async fn get_user(&self, email: &str) -> Result<User, String> {
        sqlx::query_as!(
            User,
            "SELECT * FROM users WHERE email = $1",
            email
        )
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}