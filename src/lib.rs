#![feature(try_trait_v2)]
#![feature(trait_alias)]
#[allow(async_fn_in_trait)]
pub mod app;

pub mod prelude {
    pub use super::app::prelude::*;
}
