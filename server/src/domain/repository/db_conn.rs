use async_trait::async_trait;
use sqlx::{
	Pool, 
	Transaction, 
	Error,
	mysql::{
		MySql,
		MySqlRow,
	},
};
use std::sync::Arc;

pub type ArcDBConn = Arc<dyn DBConn + Send + Sync>;

#[async_trait]
pub trait DBConn: Send + Sync {
    async fn query(&self, query: &str) -> Result<Vec<MySqlRow>, Error>;
    async fn query_one(&self, query: &str) -> Result<MySqlRow, Error>;
    async fn execute(&self, query: &str) -> Result<u32, Error>;

	async fn query_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<Vec<MySqlRow>, Error>;
	async fn query_one_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<MySqlRow, Error>;
    async fn execute_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<u32, Error>;
}

#[async_trait]
impl<'a> DBConn for Pool<MySql> {
    async fn query(&self, query: &str) -> Result<Vec<MySqlRow>, Error> {
        let mut rows = sqlx::query(query).fetch_all(self).await?;
        Ok(rows)
    }

    async fn query_one(&self, query: &str) -> Result<MySqlRow, Error> {
        let row = sqlx::query(query).fetch_one(self).await?;
        Ok(row)
    }

	async fn execute(&self, query: &str) -> Result<u32, Error> {
        let rows_affected = sqlx::query(query).execute(self).await?.rows_affected();
        Ok(rows_affected as u32)
    }

	async fn query_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<Vec<MySqlRow>, Error> {
        let mut query_builder = sqlx::query(query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        let mut rows = query_builder.fetch_all(self).await?;
        Ok(rows)
    }

	async fn query_one_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<MySqlRow, Error> {
        let mut query_builder = sqlx::query(query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        let mut row = query_builder.fetch_one(self).await?;
        Ok(row)
    }

    async fn execute_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<u32, Error> {
        let mut query_builder = sqlx::query(query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        let rows_affected = query_builder.execute(self).await?.rows_affected();
        Ok(rows_affected as u32)
    }
}

#[async_trait]
impl<'a> DBConn for Transaction<'_, MySql> {
    async fn query(&self, query: &str) -> Result<Vec<MySqlRow>, Error> {
        let mut rows = sqlx::query(query).fetch_all(&self).await?;
        Ok(rows)
    }

    async fn query_one(&self, query: &str) -> Result<MySqlRow, Error> {
        let row = sqlx::query(query).fetch_one(self).await?;
        Ok(row)
    }

	async fn execute(&self, query: &str) -> Result<u32, Error> {
        let rows_affected = sqlx::query(query).execute(self).await?.rows_affected();
        Ok(rows_affected as u32)
    }

	async fn query_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<Vec<MySqlRow>, Error> {
        let mut query_builder = sqlx::query(query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        let mut rows = query_builder.fetch_all(self).await?;
        Ok(rows)
    }

	async fn query_one_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<MySqlRow, Error> {
        let mut query_builder = sqlx::query(query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        let mut row = query_builder.fetch_one(self).await?;
        Ok(row)
    }

    async fn execute_with_params(&self, query: &str, params: &[&(dyn sqlx::Encode<MySql> + std::marker::Sync)]) -> Result<u32, Error> {
        let mut query_builder = sqlx::query(query);
        for param in params {
            query_builder = query_builder.bind(param);
        }
        let rows_affected = query_builder.execute(self).await?.rows_affected();
        Ok(rows_affected as u32)
    }
}

impl std::fmt::Debug for dyn DBConn {
    fn fmt(&self, f: &std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "custom debug implementation DBConn")
    }
}
