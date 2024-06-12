pub mod article;
pub mod tag;
pub mod user;

pub mod prelude {
    pub use super::article::{
        CreateUserLink as ArticleToCreateUserLink, TagLink as ArticleToTagLink,
        UpdateUserLink as ArticleToUpdateUserLink,
    };
    pub use super::tag::{
        ArticleLink as TagToArticleLink, ChildTagLink as TagToChildTagLink,
        CreateUserLink as TagToCreateUserLink, ParentTagLink as TagToParentTagLink,
        UpdateUserLink as TagToUpdateUserLink,
    };
    pub use super::user::{
        CreatedArticleLink as UserToCreatedArticleLink, CreatedTagLink as UserToCreatedTagLink,
        UpdatedArticleLink as UserToUpdatedArticleLink, UpdatedTagLink as UserToUpdatedTagLink,
    };
}
