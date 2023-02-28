#![feature(once_cell)]


pub use errors::{Error, Result};

pub use crate::manager::{STRING_MANAGER, StringID, StringManager};
pub use crate::smart::{SmartString, SmartStringKind};

mod manager;
mod smart;

mod errors;
