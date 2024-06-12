pub mod article;
pub mod article_tag;
pub mod tag;
pub mod user;

pub mod prelude {
    pub use super::article::{
        ActiveModel as ArticleActiveModel, Column as ArticleColumn, Entity as ArticleEntity,
        Model as ArticleModel,
    };
    pub use super::tag::{
        ActiveModel as TagActiveModel, Column as TagColumn, Entity as TagEntity, Model as TagModel,
    };
    pub use super::user::{
        ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity,
        Model as UserModel,
    };
}
