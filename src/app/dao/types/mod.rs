// ********************* mod ********************* //
pub mod user;

pub mod prelude {
    pub use super::{user::prelude::*, OrderParam, PaginateParam};
}

// ********************* content ********************* //
#[derive(Clone, Debug)]
pub struct PaginateParam {
    pub page_num: u64,
    pub page_size: u64,
}

impl Default for PaginateParam {
    fn default() -> Self {
        Self {
            page_num: 1,
            page_size: 10,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct OrderParam<T> {
    pub by: T,
    pub ascending: bool,
}
