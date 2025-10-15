pub mod error;

pub use self::{error::Error, result::Result};

pub mod result {
    pub type Result<T> = std::result::Result<T, crate::error::Error>;
}
