use chrono::{NaiveDateTime};
use sqlx::PgPool;
use uuid::Uuid;
use crate::data_store::NotificationStore;
use crate::domain::{Notification, NotificationWithUserInfo};

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

    async fn get_all_notifications(&self) -> Result<Vec<NotificationWithUserInfo>, String> {
        let rows = sqlx::query_as!(
            NotificationRow,
            "SELECT
                n.id as id,
                n.text as text,
                n.created_at as created_at,
                u.id as user_id,
                u.username as username
            FROM notifications as n
            inner join public.users u on n.user_id = u.id
            order by created_at desc"
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("{}", e))?;

            let result = rows.into_iter()
                .map(|row| NotificationWithUserInfo {
                    id: row.id,
                    text: row.text,
                    created_at: row.created_at.format("%Y-%m-%d %H:%M:%S").to_string(),
                    user_id: row.user_id,
                    username: row.username,
                })
                .collect();

        Ok(result)
    }
}

#[derive(sqlx::FromRow)]
struct NotificationRow {
    id: Uuid,
    text: String,
    created_at: NaiveDateTime,
    user_id: Uuid,
    username: String,
}