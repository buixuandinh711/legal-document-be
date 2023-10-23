pub mod document_model;
pub mod document_type_model;
pub mod draft_model;
pub mod officier_model;
pub mod signature_model;

use derive_more::{Display, Error as DeriveError};
use std::fmt::Display;

#[derive(Debug, Display, DeriveError)]
pub enum ModelError {
    ValidationError,
    InternalError,
    AuthError,
    NotFoundError,
}

impl ModelError {
    fn new(err: Self, msg: &str, err_content: &impl Display) -> Self {
        log::error!("{} -> {}", msg, err_content.to_string());
        err
    }
}
