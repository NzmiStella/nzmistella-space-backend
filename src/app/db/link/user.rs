use sea_orm::entity::prelude::*;

use super::super::entity::{article, tag, user};

pub struct CreatedTagLink;
impl Linked for CreatedTagLink {
    type FromEntity = user::Entity;
    type ToEntity = tag::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![tag::Relation::CreateUser.def().rev()]
    }
}

pub struct UpdatedTagLink;
impl Linked for UpdatedTagLink {
    type FromEntity = user::Entity;
    type ToEntity = tag::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![tag::Relation::UpdateUser.def().rev()]
    }
}

pub struct CreatedArticleLink;
impl Linked for CreatedArticleLink {
    type FromEntity = user::Entity;
    type ToEntity = article::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![article::Relation::CreateUser.def().rev()]
    }
}

pub struct UpdatedArticleLink;
impl Linked for UpdatedArticleLink {
    type FromEntity = user::Entity;
    type ToEntity = article::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![article::Relation::UpdateUser.def().rev()]
    }
}
