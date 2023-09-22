#![feature(once_cell)]

pub use errors::{Error, Result};

pub use crate::{
    manager::{StringID, StringManager, STRING_MANAGER},
    smart::{
        inlined::{InlineBuffer, LENGTH_MASK, MAX_SIZE},
        SmartString, SmartStringKind,
    },
};

mod manager;
mod smart;

mod errors;
