use sea_orm::entity::prelude::*;

use super::super::entity::{article, article_tag, tag, user};

pub struct CreateUserLink;
impl Linked for CreateUserLink {
    type FromEntity = article::Entity;
    type ToEntity = user::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![article::Relation::CreateUser.def()]
    }
}

pub struct UpdateUserLink;
impl Linked for UpdateUserLink {
    type FromEntity = article::Entity;
    type ToEntity = user::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![article::Relation::UpdateUser.def()]
    }
}

pub struct TagLink;
impl Linked for TagLink {
    type FromEntity = article::Entity;
    type ToEntity = tag::Entity;
    fn link(&self) -> Vec<RelationDef> {
        vec![
            article_tag::Relation::Article.def().rev(),
            article_tag::Relation::Tag.def(),
        ]
    }
}
