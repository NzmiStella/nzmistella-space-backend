// ********************* interface ********************* //
use std::fmt::Debug;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::app::common::prelude::AppResult;

#[async_trait]
pub trait CacheUtilsTrait {
    // basic
    async fn del(&self, key: &str) -> AppResult<()>;
    async fn exists(&self, key: &str) -> AppResult<bool>;
    async fn expire(&self, key: &str, expire_sec: u64) -> AppResult<()>;
    // string
    async fn get<T: for<'a> Deserialize<'a>>(&self, key: &str) -> AppResult<Option<T>>;
    async fn set<T: Serialize + Send>(
        &self,
        key: &str,
        value: T,
        expire_sec: Option<u64>,
    ) -> AppResult<()>;
}

pub trait CacheUtilsProvider {
    type CacheUtils: CacheUtilsTrait;
    fn cache_utils(&self) -> &Self::CacheUtils;
}

#[derive(Debug, Deserialize)]
#[serde(default)]
pub struct CacheConfig {
    // Connection Details
    pub cache_backend: String, // redis memcache
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            cache_backend: "redis".into(),
            user: "default".into(),
            password: "".into(),
            host: "localhost".into(),
            port: 6379,
            db_name: "0".into(),
        }
    }
}

// ********************* implementation ********************* //
use deadpool_redis::{Config, Connection, Pool, Runtime};
use redis::AsyncCommands;

use crate::app::common::prelude::{AppErrorKind, WrapToAppResult};

pub struct RedisCacheUtils {
    cache_pool: Pool,
}

impl RedisCacheUtils {
    pub async fn new(cfg: &CacheConfig) -> AppResult<Self> {
        let cache_url = format!(
            "redis://{}:{}@{}:{}/{}",
            cfg.user, cfg.password, cfg.host, cfg.port, cfg.db_name
        );
        let cache_pool = Config::from_url(cache_url)
            .create_pool(Some(Runtime::Tokio1))
            .wrap_with(
                || {
                    format!(
                        "Failed to create cache connection pool, cache_config: {:#?}",
                        &cfg
                    )
                },
                AppErrorKind::CacheOperationError,
            )?;
        cache_pool.get().await.wrap(
            "Failed to connect to cache server",
            AppErrorKind::CacheOperationError,
        )?;
        Ok(Self { cache_pool })
    }
    async fn get_conn(&self) -> AppResult<Connection> {
        self.cache_pool.get().await.wrap(
            "Failed to get cache connection",
            AppErrorKind::CacheOperationError,
        )
    }
}

#[async_trait]
impl CacheUtilsTrait for RedisCacheUtils {
    async fn del(&self, key: &str) -> AppResult<()> {
        self.get_conn().await?.del(key).await.wrap_with(
            || format!("Failed to delete key: {}", key),
            AppErrorKind::CacheOperationError,
        )
    }

    async fn exists(&self, key: &str) -> AppResult<bool> {
        self.get_conn().await?.exists(key).await.wrap_with(
            || format!("Failed to check key: {}", key),
            AppErrorKind::CacheOperationError,
        )
    }

    async fn expire(&self, key: &str, expire_sec: u64) -> AppResult<()> {
        self.get_conn()
            .await?
            .expire(key, expire_sec as i64)
            .await
            .wrap_with(
                || format!("Failed to set expire time for key: {}", key),
                AppErrorKind::CacheOperationError,
            )
    }

    async fn get<T: for<'a> Deserialize<'a>>(&self, key: &str) -> AppResult<Option<T>> {
        let value_opt: Option<String> = self.get_conn().await?.get(key).await.wrap_with(
            || format!("Failed to get value by key: {}", key),
            AppErrorKind::CacheOperationError,
        )?;
        match value_opt {
            Some(value_str) => {
                let value: T = serde_json::from_str(&value_str).wrap(
                    "Failed to deserialize value",
                    AppErrorKind::CacheOperationError,
                )?;
                Ok(Some(value))
            }
            None => Ok(None),
        }
    }

    async fn set<T: Serialize + Send>(
        &self,
        key: &str,
        value: T,
        expire_sec: Option<u64>,
    ) -> AppResult<()> {
        let value_str = serde_json::to_string(&value).wrap(
            "Failed to serialize value",
            AppErrorKind::CacheOperationError,
        )?;
        let mut cache_conn = self.get_conn().await?;
        if let Some(expire_sec) = expire_sec {
            cache_conn.set_ex(key, &value_str, expire_sec)
        } else {
            cache_conn.set(key, &value_str)
        }
        .await
        .wrap_with(
            || format!("Failed to set key: {} value: {}", key, value_str),
            AppErrorKind::CacheOperationError,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::config::AppConfig;
    use serde::{Deserialize, Serialize};
    use tokio::time::{sleep, Duration};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
    struct TestStruct {
        field1: String,
        field2: i32,
    }

    #[tokio::test]
    async fn test_redis_cache_utils() {
        let cfg = AppConfig::init("config/config_test.toml").unwrap();

        // 初始化
        let cache = RedisCacheUtils::new(&cfg.cache)
            .await
            .expect("Failed to create RedisCacheUtils");

        // 测试 set字符串、get存在
        cache
            .set("test_key", "test_value", None)
            .await
            .expect("Failed to set key");

        let value: Option<String> = cache.get("test_key").await.expect("Failed to get key");
        assert_eq!(value, Some("test_value".into()));

        // 测试 del、get不存在
        cache.del("test_key").await.expect("Failed to delete key");
        let deleted_value: Option<String> = cache.get("test_key").await.expect("Failed to get key");
        assert_eq!(deleted_value, None);

        // 测试 exists存在
        let exists = cache
            .exists("test_key")
            .await
            .expect("Failed to check key existence");
        assert!(!exists);

        // 测试 exists不存在
        cache
            .set("test_key", "test_value", None)
            .await
            .expect("Failed to set key");
        let exists = cache
            .exists("test_key")
            .await
            .expect("Failed to check key existence");
        assert!(exists);

        // 测试 expire
        cache
            .expire("test_key", 1)
            .await
            .expect("Failed to set expire time");
        let value_before_expire: Option<String> =
            cache.get("test_key").await.expect("Failed to get key");
        assert_eq!(value_before_expire, Some("test_value".into()));

        sleep(Duration::from_secs(2)).await;
        let value_after_expire: Option<String> = cache
            .get("test_key")
            .await
            .expect("Failed to get key after expire");
        assert_eq!(value_after_expire, None);

        // 测试 set、get序列化对象成功
        let test_struct = TestStruct {
            field1: "value1".into(),
            field2: 42,
        };
        cache
            .set("test_struct_key", &test_struct, None)
            .await
            .expect("Failed to set struct key");

        let retrieved_struct: Option<TestStruct> = cache
            .get("test_struct_key")
            .await
            .expect("Failed to get struct key");
        assert_eq!(retrieved_struct, Some(test_struct));

        // 测试set、get序列化对象失败
        cache
            .set("invalid_struct_key", "invalid_json", None)
            .await
            .expect("Failed to set invalid struct key");

        let invalid_struct: Result<Option<TestStruct>, _> = cache.get("invalid_struct_key").await;
        assert!(invalid_struct.is_err());

        // 清理
        let _: () = cache.del("test_key").await.expect("Failed to clean up");
        let _: () = cache
            .del("test_struct_key")
            .await
            .expect("Failed to clean up");
        let _: () = cache
            .del("invalid_struct_key")
            .await
            .expect("Failed to clean up");
    }
}
