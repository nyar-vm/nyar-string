use core::mem::size_of;
use std::mem::transmute;

use crate::SmartString;

pub const MAX_SIZE: usize = size_of::<String>();

pub const LENGTH_MASK: u8 = 0b11000000;

/// A buffer stored on the stack whose size is equal to the stack size of `String`
#[repr(transparent)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InlineBuffer([u8; MAX_SIZE]);

impl InlineBuffer {
    /// Construct a new [`InlineString`]. A string that lives in a small buffer on the stack
    ///
    /// SAFETY:
    /// * The caller must guarantee that the length of `text` is less than [`MAX_SIZE`]
    #[inline(always)]
    pub unsafe fn new(buffer: [u8; MAX_SIZE]) -> Self {
        InlineBuffer(buffer)
    }

    #[inline]
    pub const fn new_const(text: &str) -> Self {
        if text.len() > MAX_SIZE {
            panic!("Provided string has a length greater than our MAX_SIZE");
        }

        let len = text.len();
        let mut buffer = [0u8; MAX_SIZE];

        // set the length
        buffer[MAX_SIZE - 1] = len as u8 | LENGTH_MASK;

        // Note: for loops aren't allowed in `const fn`, hence the while.
        // Note: Iterating forward results in badly optimized code, because the compiler tries to
        //       unroll the loop.
        let text = text.as_bytes();
        let mut i = len;
        while i > 0 {
            buffer[i - 1] = text[i - 1];
            i -= 1;
        }

        InlineBuffer(buffer)
    }

    /// Returns an empty [`InlineBuffer`]
    #[inline(always)]
    pub const fn empty() -> Self {
        Self::new_const("")
    }

    /// Set's the length of the content for this [`InlineBuffer`]
    ///
    /// # SAFETY:
    /// * The caller must guarantee that `len` bytes in the buffer are valid UTF-8
    #[inline]
    pub unsafe fn set_len(&mut self, len: usize) {
        debug_assert!(len <= MAX_SIZE);

        // If `length` == MAX_SIZE, then we infer the length to be the capacity of the buffer. We
        // can infer this because the way we encode length doesn't overlap with any valid UTF-8
        // bytes
        if len < MAX_SIZE {
            self.0[MAX_SIZE - 1] = len as u8 | LENGTH_MASK;
        }
    }
    pub fn get_len(&self) -> usize {
        let len = self.0[MAX_SIZE - 1] & !LENGTH_MASK;
        if len == MAX_SIZE as u8 { MAX_SIZE } else { len as usize }
    }

    #[inline(always)]
    pub fn copy(&self) -> Self {
        InlineBuffer(self.0)
    }

    pub unsafe fn as_smart_string(&self) -> SmartString {
        transmute::<Self, SmartString>(*self)
    }
    pub unsafe fn as_str(&self) -> &str {
        let len = self.get_len();
        let ptr = self.0.as_ptr() as *const u8;
        std::str::from_utf8_unchecked(std::slice::from_raw_parts(ptr, len))
    }
}
