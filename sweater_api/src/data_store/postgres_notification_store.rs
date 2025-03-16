use chrono::{NaiveDateTime};
use sqlx::PgPool;
use uuid::Uuid;
use crate::data_store::NotificationStore;
use crate::domain::{Notification};

pub struct PostgresNotificationStore {
    pool: PgPool
}

impl PostgresNotificationStore {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl NotificationStore for PostgresNotificationStore {
    async fn create_notification(&mut self, nofification: &Notification) -> Result<(), String> {
        let created_at = NaiveDateTime::parse_from_str(&nofification.created_at, "%Y-%m-%d %H:%M:%S%.f").unwrap();
        sqlx::query!(r#"
            INSERT INTO notifications (id, user_id, text, created_at)
            VALUES ($1, $2, $3, $4)
            "#,
            nofification.id,
            nofification.user_id,
            nofification.text,
            created_at
          )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(())
    }

    async fn get_all_notifications(&self) -> Result<Vec<Notification>, String> {
        let rows = sqlx::query_as!(
            NotificationRow,
            "SELECT * FROM notifications order by created_at desc"
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("{}", e))?;

            let result = rows.into_iter()
                .map(|row| Notification {
                    id: row.id,
                    user_id: row.user_id,
                    text: row.text,
                    created_at: row.created_at.to_string()
                })
                .collect();

        Ok(result)
    }
}

#[derive(sqlx::FromRow)]
struct NotificationRow {
    id: Uuid,
    user_id: Uuid,
    text: String,
    created_at: NaiveDateTime,
}