#![feature(once_cell)]

use std::mem::size_of;
use std::sync::LazyLock;

use dashmap::DashMap;

mod manager;
mod smart;

pub use errors::{Error, Result};

mod errors;
pub use crate::manager::{StringManager, STRING_MANAGER};