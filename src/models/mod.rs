pub mod division_model;
pub mod officier_model;
pub mod onchain_officer_model;

use derive_more::{Display, Error as DeriveError};

#[derive(Debug, Display, DeriveError)]
pub enum ModelError {
    ValidationError,
    InternalError,
}

impl ModelError {
    fn new(err: Self, msg: &str) -> Self {
        log::error!("{}", msg);
        err
    }
}
