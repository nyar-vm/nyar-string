use std::fmt::{Debug, Formatter};
use std::mem::size_of;

#[repr(C)]
pub struct SmartString {
    // 24 bytes, 192 bits
    // [u64, u64, u64]
    pointer: [u8; 24],
}

impl SmartString {
    pub fn new() -> Self {
        Self {
            pointer: [0u8; 24],
        }
    }
    pub fn managed(s: &str) -> SmartString {

    }
}

#[test]
fn test() {
    let s = SmartString::new();
    println!("{:?}", s);
}

impl Debug for SmartString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let high = u64::from_be_bytes(self.pointer[0..8].try_into().unwrap());
        let mid = u64::from_be_bytes(self.pointer[8..16].try_into().unwrap());
        let low = u64::from_be_bytes(self.pointer[16..24].try_into().unwrap());
        f.debug_struct("SmartString")
            .field("high", &format!("{:#x}", high))
            .field("mid", &format!("{:#x}", mid))
            .field("low", &format!("{:#x}", low))
            .finish()
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SmartStringKind {
    /// Static Layout
    /// ```js
    /// 00______ ________ ________ ________
    /// &'static str
    /// ```
    Static = 0,
    /// Managed Layout
    /// ```js
    /// 01______ ________ ________ ________
    /// ________ ________ ________ ________
    /// u64
    /// ```
    Managed = 1,
    /// Inlined Layout
    /// ```js
    /// 01xxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
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

#[test]
fn keep_size_of() {
    assert_eq!(size_of::<SmartString>(), size_of::<String>())
}