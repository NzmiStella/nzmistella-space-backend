// ********************* interface ********************* //
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{app::common::prelude::AppResult, prelude::AppError};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    /// 用户ID
    pub user_id: i32,
    /// 用户组
    aud: String,
    /// 过期时间戳
    exp: u64,
    /// 生效时间戳
    nbf: u64,
    /// token版本
    version: String,
}

#[async_trait]
pub trait TokenUtilsTrait {
    async fn generate_token(
        &self,
        user_id: i32,
        group_type: i32,
        exp_sec: u64,
    ) -> AppResult<String>;
    async fn verify_token<T: ToString + Sync>(
        &self,
        token: &str,
        allow_groups: &[T],
    ) -> AppResult<Claims>;
    async fn invalidate_token(&self, user_id: i32, exp_src: u64) -> AppResult<()>;
}

pub trait TokenUtilsProvider {
    type TokenUtils: TokenUtilsTrait;
    fn token_utils(&self) -> &Self::TokenUtils;
}

// ********************* implementation ********************* //
use std::sync::Arc;

use jsonwebtoken::{
    self, decode, encode, errors::ErrorKind as JWTError, get_current_timestamp, DecodingKey,
    EncodingKey, Header, Validation,
};
use rand::{distributions::Alphanumeric, thread_rng, Rng};
use ring::rand::{SecureRandom, SystemRandom};

use super::prelude::CacheUtilsTrait;
use crate::app::common::prelude::{AppErrorKind, IntoAppError, WrapToAppResult};

pub struct JwtTokenUtils<C: CacheUtilsTrait> {
    secret_key: [u8; 32],
    header: Header,
    validation: Validation,
    cache_utils: Arc<C>,
}
impl<C: CacheUtilsTrait> JwtTokenUtils<C> {
    pub async fn new(cache_utils: Arc<C>) -> AppResult<Self> {
        let secret_key = cache_utils.get("token_utils:secret_key").await?;
        let secret_key = match secret_key {
            Some(key) => key,
            None => {
                let mut key = [0u8; 32];
                SystemRandom::new()
                    .fill(&mut key)
                    .wrap("Failed to generate a secret key", AppErrorKind::default())?;
                cache_utils.set("token_utils:secret_key", key, None).await?;
                key
            }
        };

        let header = Header::default();
        let mut validation = Validation::default();
        validation.validate_aud = true;
        validation.validate_exp = true;
        validation.validate_nbf = true;
        validation.leeway = 5;
        validation.set_required_spec_claims(&["aud", "exp", "nbf"]);

        Ok(Self {
            secret_key,
            header,
            validation,
            cache_utils,
        })
    }
    async fn get_token_version(&self, user_id: i32) -> AppResult<Option<String>> {
        self.cache_utils
            .get(&format!("user:{}:token_version", user_id))
            .await
    }
    async fn set_token_version(&self, user_id: i32, version: &str, exp_src: u64) -> AppResult<()> {
        self.cache_utils
            .set(
                &format!("user:{}:token_version", user_id),
                version,
                Some(exp_src),
            )
            .await
    }
}

#[async_trait]
impl<C: CacheUtilsTrait + Send + Sync> TokenUtilsTrait for JwtTokenUtils<C> {
    async fn generate_token(
        &self,
        user_id: i32,
        group_type: i32,
        exp_sec: u64,
    ) -> AppResult<String> {
        // 如果用户的token_version已在缓存中，说明只有最新版本的token可用，需要用该版本生成新token
        let version = self.get_token_version(user_id).await?.unwrap_or_default();
        let claims = Claims {
            user_id,
            aud: group_type.to_string(),
            exp: get_current_timestamp().saturating_add(exp_sec),
            nbf: get_current_timestamp(),
            version,
        };

        encode(
            &self.header,
            &claims,
            &EncodingKey::from_secret(&self.secret_key),
        )
        .wrap("Token encoding failed", AppErrorKind::default())
    }

    async fn verify_token<T: ToString + Sync>(
        &self,
        token: &str,
        allow_groups: &[T],
    ) -> AppResult<Claims> {
        // 如果用户的token_version已在缓存中，说明只有最新版本的token可用，需要验证token版本
        let mut validation = self.validation.clone();
        validation.set_audience(allow_groups);

        let claims = decode::<Claims>(
            token,
            &DecodingKey::from_secret(&self.secret_key),
            &validation,
        )
        .map(|data| data.claims)
        .map_err(|e| match e.kind() {
            JWTError::InvalidAudience => e.with_err_kind(AppErrorKind::PermissionDenied),
            JWTError::ExpiredSignature | JWTError::ImmatureSignature => {
                e.with_err_kind(AppErrorKind::InvalidCredential)
            }
            _ => e.with_err_kind(AppErrorKind::MalformedCredential),
        })?;
        if let Some(version) = self.get_token_version(claims.user_id).await? {
            if version != claims.version {
                return Err(AppError::new(
                    "Token version mismatch",
                    AppErrorKind::InvalidCredential,
                ));
            }
        }
        Ok(claims)
    }
    async fn invalidate_token(&self, user_id: i32, exp_src: u64) -> AppResult<()> {
        // 令已授权的token失效，只需更新token版本
        let version: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(8)
            .map(char::from)
            .collect();
        self.set_token_version(user_id, &version, exp_src).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::config::AppConfig;
    use crate::app::utils::cache::RedisCacheUtils;

    #[tokio::test]
    async fn test_generate_and_verify_token() {
        // 初始化
        let cfg = AppConfig::init("config/config_test.toml").unwrap();
        let cache_utils = Arc::new(
            RedisCacheUtils::new(&cfg.cache)
                .await
                .expect("Failed to create RedisCacheUtils"),
        );
        let token_utils = JwtTokenUtils::new(cache_utils).await.unwrap();

        let user_id = 1;
        let group_type = 1;
        let exp_sec = 3600;

        // 测试生成 token
        let token_result = token_utils
            .generate_token(user_id, group_type, exp_sec)
            .await;
        assert!(token_result.is_ok());
        let token = token_result.unwrap();

        // 测试验证 token，假设只允许特定组访问
        let allowed_groups = vec![group_type];
        let verify_res = token_utils.verify_token(&token, &allowed_groups).await;
        assert!(verify_res.is_ok());

        let claims = verify_res.unwrap();
        assert_eq!(claims.user_id, user_id);
        assert_eq!(claims.aud, group_type.to_string());
    }

    #[tokio::test]
    async fn test_token_expiration() {
        // 初始化
        let cfg = AppConfig::init("config/config_test.toml").unwrap();
        let cache_utils = Arc::new(
            RedisCacheUtils::new(&cfg.cache)
                .await
                .expect("Failed to create RedisCacheUtils"),
        );
        let token_utils = JwtTokenUtils::new(cache_utils).await.unwrap();

        let user_id = 1;
        let group_type = 1;
        let exp_sec = 0;

        // 生成 token
        let token = token_utils
            .generate_token(user_id, group_type, exp_sec)
            .await
            .unwrap();

        // 等待 token 过期
        std::thread::sleep(std::time::Duration::from_secs(exp_sec + 5 + 1));

        // 验证 token
        let allowed_groups = vec![group_type];
        let verify_res = token_utils.verify_token(&token, &allowed_groups).await;
        assert!(verify_res.is_err());

        // 检查错误类型
        let error = verify_res.unwrap_err();
        match error.kind {
            AppErrorKind::InvalidCredential => (),
            _ => panic!("Expected TokenExpired error, got {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_invalid_group_access() {
        // 初始化
        let cfg = AppConfig::init("config/config_test.toml").unwrap();
        let cache_utils = Arc::new(
            RedisCacheUtils::new(&cfg.cache)
                .await
                .expect("Failed to create RedisCacheUtils"),
        );
        let token_utils = JwtTokenUtils::new(cache_utils).await.unwrap();

        let user_id = 1;
        let group_type = 1;
        let exp_sec = 3600;

        // 生成 token
        let token = token_utils
            .generate_token(user_id, group_type, exp_sec)
            .await
            .unwrap();

        // 使用不允许的组尝试验证 token
        let not_allowed_groups = vec![0];
        let verify_res = token_utils.verify_token(&token, &not_allowed_groups).await;
        assert!(verify_res.is_err());

        // 检查错误类型是否为 Unauthorized
        let error = verify_res.unwrap_err();
        match error.kind {
            AppErrorKind::PermissionDenied => (),
            _ => panic!("Expected Unauthorized error, got {:?}", error),
        }
    }

    #[tokio::test]
    async fn test_invalidate_token() {
        // 初始化
        let cfg = AppConfig::init("config/config_test.toml").unwrap();
        let cache_utils = Arc::new(
            RedisCacheUtils::new(&cfg.cache)
                .await
                .expect("Failed to create RedisCacheUtils"),
        );
        let token_utils = JwtTokenUtils::new(cache_utils).await.unwrap();

        let user_id = 1;
        let group_type = 1;
        let exp_sec = 3600;

        let token = token_utils
            .generate_token(user_id, group_type, exp_sec)
            .await
            .unwrap();
        let verify_before = token_utils.verify_token(&token, &["1".to_string()]).await;
        assert!(verify_before.is_ok());

        token_utils
            .invalidate_token(user_id, exp_sec)
            .await
            .unwrap();
        let verify_after = token_utils.verify_token(&token, &["1".to_string()]).await;
        assert!(verify_after.is_err());
    }
}
