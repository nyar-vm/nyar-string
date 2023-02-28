use std::fmt::{Debug, Formatter};

use crate::STRING_MANAGER;

#[repr(C)]
pub struct SmartString {
    // 24 bytes, 192 bits
    // [u64, u64, u64]
    pointer: [u8; 24],
}

impl SmartString {
    pub fn new(s: &str) -> Self {
        todo!()
    }

    pub fn managed(s: &str) -> SmartString {
        let id = STRING_MANAGER.insert(s);
        let mut pointer = [0u8; 24];
        pointer[23] = SmartStringKind::Managed as u8;
        let id = id.to_be_bytes();
        pointer[0..8].copy_from_slice(&id[0..8]);
        Self { pointer }
    }
}

#[test]
fn test() {
    let s1 = SmartString::managed("a");

    let s2 = SmartString::managed("a");
    println!("{:?}", s1);
    println!("{:?}", s2);
}

impl Debug for SmartString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let high = u64::from_be_bytes(self.pointer[0..8].try_into().unwrap());
        let mid = u64::from_be_bytes(self.pointer[8..16].try_into().unwrap());
        let low = u64::from_be_bytes(self.pointer[16..24].try_into().unwrap());
        f.debug_struct("SmartString")
            .field("high", &format!("{:#b}", high))
            .field("mid", &format!("{:#b}", mid))
            .field("low", &format!("{:#b}", low))
            .finish()
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SmartStringKind {
    /// Static Layout
    /// ```js
    /// &'static str
    /// ________ ________ ________ ______00
    /// ```
    Static = 0,
    /// Managed Layout
    /// ```js
    /// u64
    /// ________ ________ ________ ______01
    /// ```
    Managed = 1,
    /// Inlined Layout
    /// ```js
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxx01
    /// ```
    Inlined = 2,
    /// Heap Layout
    /// ```js
    /// box
    /// str
    /// ```
    Heap = 3,
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
    pub unsafe fn as_static(&self) -> Option<&'static str> {
        todo!()
    }
    pub fn as_managed(&self) -> Option<&str> {
        todo!()
    }
}
