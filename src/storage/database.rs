use crate::domain::{Transaction, TransactionStatus, User};
use crate::error::Result;
use sqlx::{postgres::PgPoolOptions, PgPool, Postgres};
use tracing::info;
use uuid::Uuid;

/// Database connection pool and operations
#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new(database_url: &str) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(20)
            .connect(database_url)
            .await?;

        info!("Database connection pool established");

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    // User operations
    pub async fn create_user(&self, user: &User) -> Result<User> {
        let user = sqlx::query_as::<Postgres, User>(
            r#"
            INSERT INTO users (id, phone_number, name, email, language, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(&user.id)
        .bind(&user.phone_number)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.language)
        .bind(&user.status)
        .bind(&user.created_at)
        .bind(&user.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = sqlx::query_as::<Postgres, User>(
            r#"
            SELECT * FROM users WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_user_by_phone(&self, phone_number: &str) -> Result<Option<User>> {
        let user = sqlx::query_as::<Postgres, User>(
            r#"
            SELECT * FROM users WHERE phone_number = $1
            "#,
        )
        .bind(phone_number)
        .fetch_optional(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn update_user(&self, user: &User) -> Result<User> {
        let user = sqlx::query_as::<Postgres, User>(
            r#"
            UPDATE users
            SET name = $2, email = $3, language = $4, status = $5, updated_at = $6
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(&user.id)
        .bind(&user.name)
        .bind(&user.email)
        .bind(&user.language)
        .bind(&user.status)
        .bind(&user.updated_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(user)
    }

    pub async fn get_or_create_user(&self, phone_number: &str) -> Result<User> {
        if let Some(user) = self.get_user_by_phone(phone_number).await? {
            Ok(user)
        } else {
            let user = User::new(phone_number.to_string());
            self.create_user(&user).await
        }
    }

    // Transaction operations
    pub async fn create_transaction(&self, transaction: &Transaction) -> Result<Transaction> {
        let tx = sqlx::query_as::<Postgres, Transaction>(
            r#"
            INSERT INTO transactions (id, user_id, session_id, transaction_type, amount, status, metadata, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING *
            "#,
        )
        .bind(&transaction.id)
        .bind(&transaction.user_id)
        .bind(&transaction.session_id)
        .bind(&transaction.transaction_type)
        .bind(&transaction.amount)
        .bind(&transaction.status)
        .bind(&transaction.metadata)
        .bind(&transaction.created_at)
        .fetch_one(&self.pool)
        .await?;

        Ok(tx)
    }

    pub async fn get_transaction(&self, id: Uuid) -> Result<Option<Transaction>> {
        let tx = sqlx::query_as::<Postgres, Transaction>(
            r#"
            SELECT * FROM transactions WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(tx)
    }

    pub async fn get_user_transactions(
        &self,
        user_id: Uuid,
        limit: i64,
    ) -> Result<Vec<Transaction>> {
        let transactions = sqlx::query_as::<Postgres, Transaction>(
            r#"
            SELECT * FROM transactions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(transactions)
    }

    pub async fn update_transaction_status(
        &self,
        id: Uuid,
        status: TransactionStatus,
    ) -> Result<Transaction> {
        let tx = sqlx::query_as::<Postgres, Transaction>(
            r#"
            UPDATE transactions
            SET status = $2
            WHERE id = $1
            RETURNING *
            "#,
        )
        .bind(id)
        .bind(status)
        .fetch_one(&self.pool)
        .await?;

        Ok(tx)
    }

    // Analytics
    pub async fn get_session_count_by_date(&self, days: i32) -> Result<Vec<(String, i64)>> {
        let results = sqlx::query_as::<Postgres, (String, i64)>(
            r#"
            SELECT
                date_trunc('day', created_at)::date::text as date,
                COUNT(*) as count
            FROM transactions
            WHERE created_at >= NOW() - INTERVAL '1 day' * $1
            GROUP BY date_trunc('day', created_at)
            ORDER BY date_trunc('day', created_at) DESC
            "#,
        )
        .bind(days)
        .fetch_all(&self.pool)
        .await?;

        Ok(results)
    }

    pub async fn close(&self) {
        self.pool.close().await;
        info!("Database connection pool closed");
    }
}
