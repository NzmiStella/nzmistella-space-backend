use sea_orm::entity::prelude::*;

use super::super::entity::{article, article_tag, tag, user};

pub struct CreateUserLink;
impl Linked for CreateUserLink {
    type FromEntity = tag::Entity;
    type ToEntity = user::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![tag::Relation::CreateUser.def()]
    }
}

pub struct UpdateUserLink;
impl Linked for UpdateUserLink {
    type FromEntity = tag::Entity;
    type ToEntity = user::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![tag::Relation::UpdateUser.def()]
    }
}

pub struct ArticleLink;
impl Linked for ArticleLink {
    type FromEntity = tag::Entity;
    type ToEntity = article::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![
            article_tag::Relation::Tag.def().rev(),
            article_tag::Relation::Article.def(),
        ]
    }
}

pub struct ChildTagLink;
impl Linked for ChildTagLink {
    type FromEntity = tag::Entity;
    type ToEntity = tag::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![tag::Relation::ParentTag.def().rev()]
    }
}

pub struct ParentTagLink;
impl Linked for ParentTagLink {
    type FromEntity = tag::Entity;
    type ToEntity = tag::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![tag::Relation::ParentTag.def()]
    }
}
