use super::InternedStr;
#[cfg(not(feature = "std"))]
use alloc::string::String;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct FixedString {
    contents: Vec<u8>,
}

impl FixedString {
    /// Creates a new fixed string with the given fixed capacity.
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            contents: Vec::with_capacity(cap),
        }
    }

    /// Returns the underlying [`Box<str>`].
    ///
    /// Guarantees not to perform any reallocations in this process.
    #[inline]
    pub fn finish(self) -> Vec<u8> {
        self.contents
    }

    /// Returns the capacity in bytes of the fixed string.
    #[inline]
    pub fn capacity(&self) -> usize {
        self.contents.capacity()
    }

    /// Returns the length in bytes of the fixed string.
    #[inline]
    pub fn len(&self) -> usize {
        self.contents.len()
    }

    /// Pushes the given string into the fixed string if there is enough capacity.
    ///
    /// Returns a reference to the pushed string if there was enough capacity to
    /// perform the operation. Otherwise returns `None`.
    #[inline]
    pub fn push_str(&mut self, string: &[u8]) -> Option<InternedStr> {
        let len = self.len();
        if self.capacity() < len + string.len() {
            return None;
        }
        self.contents.extend_from_slice(string);
        debug_assert_eq!(self.contents.len(), len + string.len());
        Some(InternedStr::new(&self.contents[len..len + string.len()]))
    }
}
