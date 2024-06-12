// ********************* mod ********************* //
pub mod common;
pub mod config;
pub mod controller;
pub mod dao;
pub mod db;
pub mod middleware;
pub mod service;
pub mod utils;

pub mod prelude {
    pub use super::common::prelude::*;
    pub use super::config::AppConfig;
    pub use super::controller::prelude::*;
    pub use super::dao::prelude::*;
    pub use super::db::prelude::*;
    pub use super::middleware::prelude::*;
    pub use super::service::prelude::*;
    pub use super::utils::prelude::*;
    pub use super::App;
}

// ********************* import ********************* //
use std::{ops::Deref, sync::Arc};

use axum::{Extension, Router};

use prelude::{
    create_db_conn, init_logging, user_admin_router, user_public_router, AppConfig, AppErrorKind,
    AppResult, IntoAppResult, JwtTokenUtils, Pbkdf2CryptoUtils, RedisCacheUtils, UserDAO,
    UserService,
};

// ********************* content ********************* //
pub struct App;

impl App {
    pub async fn run() -> AppResult<()> {
        // config
        let cfg = AppConfig::init("config/config_test.toml")?;

        // log
        init_logging(&cfg.log).with_err_kind(AppErrorKind::default())?;

        // utils
        let cache_utils = Arc::new(RedisCacheUtils::new(&cfg.cache).await?);
        let crypto_utils = Arc::new(Pbkdf2CryptoUtils::default());
        let token_utils = Arc::new(JwtTokenUtils::new(cache_utils).await?);

        // db
        let db_conn = Arc::new(create_db_conn(&cfg.db).await?);

        // dao
        let user_dao = Arc::new(UserDAO::new(db_conn));

        // service
        let user_service = Arc::new(UserService::new(user_dao, crypto_utils, token_utils));

        // router
        let app = Router::new().nest(
            "/api/v1",
            Router::new()
                .nest(
                    "/public/",
                    Router::new().nest("/user", user_public_router(user_service.deref())),
                )
                .nest(
                    "/admin",
                    Router::new().nest("/user", user_admin_router(user_service.deref())),
                )
                .layer(Extension(user_service)),
        );

        // app server
        let addr = format!("{}:{}", cfg.service.host, cfg.service.port);
        println!("runing on {}", &addr);
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .with_err_kind(AppErrorKind::default())?;
        axum::serve(listener, app)
            .await
            .with_err_kind(AppErrorKind::default())?;
        Ok(())
    }
}
