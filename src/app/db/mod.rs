// ********************* mod ********************* //
pub mod entity;
pub mod link;

pub mod prelude {
    pub use super::entity::prelude::*;
    pub use super::link::prelude::*;
    pub use super::{create_db_conn, DBConfig, DBConnProvider, DatabaseConnection};
}

// ********************* import ********************* //
use std::time::Duration;

use sea_orm::{ConnectOptions, Database};
use serde::Deserialize;

use crate::app::common::prelude::*;

// ********************* content ********************* //
pub type DatabaseConnection = sea_orm::DatabaseConnection;

pub trait DBConnProvider {
    fn db_conn(&self) -> &DatabaseConnection;
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct DBConfig {
    // Connection Details
    pub db_backend: String, // mysql sqlite postgre
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
    // Connection Pool Configuration
    pub max_conns: u32,
    pub conn_timeout: u64,
}

impl Default for DBConfig {
    fn default() -> Self {
        Self {
            db_backend: "mysql".into(),
            user: "root".into(),
            password: "".into(),
            host: "localhost".into(),
            port: 3306,
            db_name: "space_backend".into(),
            max_conns: 10,
            conn_timeout: 8,
        }
    }
}

pub async fn create_db_conn(cfg: &DBConfig) -> AppResult<DatabaseConnection> {
    let db_url = format!(
        "{}://{}:{}@{}:{}/{}",
        cfg.db_backend, cfg.user, cfg.password, cfg.host, cfg.port, cfg.db_name
    );
    let mut opt = ConnectOptions::new(&db_url);
    opt.max_connections(cfg.max_conns)
        .connect_timeout(Duration::from_secs(cfg.conn_timeout));

    // Set the timezone to system timezone
    use sea_orm::{SqlxMySqlConnector, SqlxPostgresConnector};
    use sqlx::{Executor, MySql, Postgres};

    match cfg.db_backend.as_str() {
        "mysql" => opt
            .pool_options::<MySql>()
            .after_connect(move |conn, _| {
                Box::pin(async move {
                    conn.execute("SET time_zone = 'system';").await?;
                    Ok(())
                })
            })
            .connect(&db_url)
            .await
            .map(SqlxMySqlConnector::from_sqlx_mysql_pool)
            .wrap_with(
                || format!("Failed to connect to the database\ndb_config: {:#?}", &cfg),
                AppErrorKind::DBOperationError,
            ),
        "postgres" => opt
            .pool_options::<Postgres>()
            .after_connect(move |conn, _| {
                Box::pin(async move {
                    conn.execute("SET TIME ZONE 'system';").await?;
                    Ok(())
                })
            })
            .connect(&db_url)
            .await
            .map(SqlxPostgresConnector::from_sqlx_postgres_pool)
            .wrap_with(
                || format!("Failed to connect to the database\ndb_config: {:#?}", &cfg),
                AppErrorKind::DBOperationError,
            ),
        _ => Database::connect(opt).await.wrap_with(
            || format!("Failed to connect to the database\ndb_config: {:#?}", &cfg),
            AppErrorKind::DBOperationError,
        ),
    }
}
