#![feature(once_cell)]

use std::mem::size_of;
use std::sync::LazyLock;

use dashmap::DashMap;



pub use errors::{Error, Result};

mod errors;

pub struct SmartString {
    // 24 bytes, 192 bits
    pointer: [u8; 24],
}

#[repr(u8)]
pub enum SmartStringKind {
    /// Static Layout
    /// ```js
    /// 00______ ________ ________ ________
    /// &'static str
    /// ```
    Static,
    /// Managed Layout
    /// ```js
    /// 01______ ________ ________ ________
    /// ________ ________ ________ ________
    /// u64
    /// ```
    Managed,
    /// Inlined Layout
    /// ```js
    /// 01xxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// ```
    Inlined,
    /// Heap Layout
    /// ```js
    /// box
    /// str
    /// ```
    Heap,
}

impl From<&'static str> for SmartString {
    fn from(s: &'static str) -> Self {
        let mut pointer = [0u8; 24];
        pointer[0] = SmartStringKind::Static as u8;
        pointer[1..].copy_from_slice(s.as_bytes());
        Self { pointer }
    }
}

impl SmartString {
    pub fn kind(&self) -> SmartStringKind {
        match self.pointer[0] >> 6 {
            0b00 => SmartStringKind::Static,
            0b01 => SmartStringKind::Managed,
            0b10 => SmartStringKind::Inlined,
            0b11 => SmartStringKind::Heap,
            _ => unreachable!(),
        }
    }
    pub fn as_static(&self) -> Option<&'static str> {
        if self.kind() == SmartStringKind::Static as u8 {
            let len = self.pointer[1..].iter().position(|&x| x == 0).unwrap();
            let s = unsafe { std::str::from_utf8_unchecked(&self.pointer[1..1 + len]) };
            Some(s)
        } else {
            None
        }
    }
}

#[test]
fn test_size_of() {
    assert_eq!(size_of::<SmartString>(), size_of::<String>())
}