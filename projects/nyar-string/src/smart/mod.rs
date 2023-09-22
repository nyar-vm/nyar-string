use std::{
    fmt::{Debug, Formatter, UpperHex},
    mem::transmute,
    str::from_utf8_unchecked,
};

use compact_str::CompactString;

use crate::{InlineBuffer, StringManager, MAX_SIZE, STRING_MANAGER};

pub mod inlined;
pub mod on_heap;

/// **Managed**: `Rc<String>`, `Arc<String>`
///
/// ```
/// # use smart_string::SmartStringKind;
/// pub struct SmartString {
///     pointer: u64,
///     length: u64,
///     fill: [u8; 7],
///     kind: SmartStringKind,
/// }
/// ```
#[repr(C)]
#[derive(Copy, Clone, Debug)]
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

impl UpperHex for SmartString {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for byte in self.as_bytes() {
            write!(f, "{:02X}", byte)?;
        }
        Ok(())
    }
}

// impl Debug for SmartString {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("SmartStringKind").field("kind", &self.kind).finish()
//     }
// }

impl Default for SmartString {
    fn default() -> Self {
        Self { pointer: 0, length: 0, fill32: 0, fill16: 0, fill8: 0, kind: SmartStringKind::Inlined as u8 }
    }
}

impl SmartString {
    #[inline]
    pub fn new(str: &str) -> SmartString {
        match Self::try_inline(str) {
            Some(s) => s,
            None => Self::try_managed(str.to_string()),
        }
    }
    /// Create a new managed string
    #[inline]
    pub fn try_managed(string: String) -> SmartString {
        let length = string.len();
        let pointer = STRING_MANAGER.insert(string);
        Self { pointer, length, fill32: 0, fill16: 0, fill8: 0, kind: SmartStringKind::Managed as u8 }
    }
    /// Create a intern string without checking if it already exists
    #[inline]
    pub unsafe fn managed(string: &str) -> SmartString {
        Self {
            pointer: StringManager::get_hash_key(&string),
            length: string.len(),
            fill32: 0,
            fill16: 0,
            fill8: 0,
            kind: SmartStringKind::Managed as u8,
        }
    }
    /// Create a new static string, remove the lifetime
    #[inline]
    pub unsafe fn static_str(s: &'static str) -> SmartString {
        let pointer = s.as_ptr() as usize;
        Self { pointer, length: s.len(), fill32: 0, fill16: 0, fill8: 0, kind: SmartStringKind::Static as u8 }
    }
    #[inline]
    pub fn try_inline(s: &str) -> Option<SmartString> {
        if s.chars().count() > 24 {
            return None;
        }
        unsafe { Some(Self::inlined(s)) }
    }
    /// Create a new inlined string
    /// 192 bits / (char = 8bits) = 24 chars
    #[inline]
    pub unsafe fn inlined(s: &str) -> SmartString {
        unsafe { InlineBuffer::new_const(s).as_smart_string() }
    }
    /// Create a new string on the heap
    #[inline]
    pub fn heap(s: &str) -> SmartString {
        let pointer = Box::new(s).as_ptr() as usize;
        Self { pointer, length: s.len(), fill32: 0, fill16: 0, fill8: 0, kind: 0 }
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
            SmartStringKind::Inlined => (self.kind & 0b1111_1111) as usize,
            SmartStringKind::Static => self.length,
            SmartStringKind::Managed => self.length,
            SmartStringKind::Heap => self.length,
        }
    }
    pub fn as_str(&self) -> &str {
        let s = CompactString::new("");
        match self.kind() {
            SmartStringKind::Inlined => todo!(),
            SmartStringKind::Static => {
                todo!()
            }
            SmartStringKind::Managed => match STRING_MANAGER.get(self.pointer) {
                Some(s) => s.as_ref(),
                None => "",
            },
            SmartStringKind::Heap => {
                todo!()
            }
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Hash)]
pub enum SmartStringKind {
    /// Inlined Layout
    /// ```js
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx xxxxxxxx
    /// xxxxxxxx xxxxxxxx xxxxxxxx len_mask + 00
    /// ```
    Inlined = 0,
    /// Static Layout
    /// ```js
    /// &'static str
    /// len: usize
    /// ________ ________ ________ ______01
    /// ```
    Static = 1,
    /// Managed Layout
    /// ```js
    /// u64
    /// usize
    /// ________ ________ ________ ______10
    /// ```
    Managed = 2,
    /// Heap Layout
    /// ```js
    /// str
    /// usize
    /// box ________ ________ ______11
    /// ```
    Heap = 3,
}

impl From<&'static str> for SmartString {
    fn from(s: &'static str) -> Self {
        unsafe { SmartString::static_str(s) }
    }
}

impl SmartString {
    pub fn as_static(&self) -> Option<&'static str> {
        match self.kind() {
            SmartStringKind::Static => {
                todo!()
            }
            _ => None,
        }
    }
    pub fn as_managed(&self) -> Option<&str> {
        todo!()
    }
    pub fn as_bytes(self) -> [u8; MAX_SIZE] {
        unsafe { transmute(self) }
    }
}
