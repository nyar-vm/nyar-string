use std::fmt::{Debug, Formatter};
use compact_str::CompactString;

pub mod inlined;

use crate::STRING_MANAGER;

/// **Managed**: `Rc<String>`, `Arc<String>`
///
/// ```
/// # use smart_string::SmartStringKind;
/// pub struct SmartString {
///     pointer: u64,
///     length: u64,
///     extra: [u8; 7],
///     kind: SmartStringKind,
/// }
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct SmartString {
    /// `*const ()`
    pointer: usize,
    /// length of the string
    length: usize,
    #[cfg(target_pointer_width = "64")]
    fill32: u32,
    fill16: u16,
    fill8: u8,
    kind: u8,
}

impl Debug for SmartStringKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SmartStringKind")
            .field("kind", &self as &u8)
            .finish()
    }
}

impl Default for SmartString {
    fn default() -> Self {
        Self {
            pointer: 0,
            length: 0,
            fill32: 0,
            fill16: 0,
            fill8: 0,
            kind: SmartStringKind::Inlined as u8,
        }
    }
}


impl SmartString {
    #[inline]
    pub fn new(s: &str) -> Self {
        todo!()
    }
    #[inline]
    pub fn managed(s: &str) -> SmartString {
        let id = STRING_MANAGER.insert(s);
        Self {
            pointer: id,
            length: s.len(),
            fill32: 0,
            fill16: 0,
            fill8: 0,
            kind: SmartStringKind::Managed as u8,
        }
    }
    // 192 bits / (char = 8bits) = 24 chars
    pub fn inlined(s: &str) -> Option<SmartString> {
        if s.as_bytes().contains(&0) {
            return None;
        }
        if s.chars().count() > 24 {
            return None;
        }
        if s.len() > 8 {
            return None;
        }
       CompactString::new_inline()
        let inline = InlineBuffer::new_const(text);
        Repr::from_inline(inline)
    }
    pub fn heap(s: &str) -> SmartString {
        todo!()
    }
}

impl SmartString {
    #[inline(always)]
    pub const fn kind(&self) -> SmartStringKind {
        match self.kind {
            0b00 => SmartStringKind::Inlined,
            0b01 => SmartStringKind::Static,
            0b10 => SmartStringKind::Managed,
            0b11 => SmartStringKind::Heap,
            _ => unreachable!(),
        }
    }
    pub fn len(&self) -> usize {
        match self.kind() {
            SmartStringKind::Inlined => {
                todo!()
            }
            SmartStringKind::Static => {
                todo!()
            }
            SmartStringKind::Managed => {
                todo!()
            }
            SmartStringKind::Heap => {
                todo!()
            }
        }
    }
}

#[test]
fn test() {
    let s1 = SmartString::managed("a");
    let s2 = SmartString::managed("a");
    println!("{:?}", s1);
    println!("{:?}", s2);
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub enum SmartStringKind {
    /// Inlined Layout
    /// ```js
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxx00
    /// ```
    Inlined = 0,
    /// Static Layout
    /// ```js
    /// &'static str
    /// ________ ________ ________ ______01
    /// ```
    Static = 1,
    /// Managed Layout
    /// ```js
    /// u64
    /// ________ ________ ________ ______10
    /// ```
    Managed = 2,
    /// Heap Layout
    /// ```js
    /// box
    /// str
    /// ```
    Heap = 3,
}

impl From<&'static str> for SmartString {
    fn from(s: &'static str) -> Self {
        todo!()
    }
}

impl SmartString {
    pub unsafe fn as_static(&self) -> Option<&'static str> {
        todo!()
    }
    pub fn as_managed(&self) -> Option<&str> {
        todo!()
    }
}
