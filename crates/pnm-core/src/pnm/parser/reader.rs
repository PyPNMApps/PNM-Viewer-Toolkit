/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use crate::diagnostic::{ErrorCode, ParseError};

/// Cursor over borrowed bytes with checked, transactional reads.
/// Failed reads leave the cursor unchanged.
pub struct Reader<'input> {
    bytes: &'input [u8],
    offset: usize,
}
impl<'input> Reader<'input> {
    /// Starts a reader at byte offset zero.
    ///
    /// # Arguments
    /// * `bytes` - Input retained for the reader lifetime.
    ///
    /// # Returns
    /// A cursor borrowing the input without copying it.
    pub fn new(bytes: &'input [u8]) -> Self {
        Self { bytes, offset: 0 }
    }
    /// Copies a fixed-size field and advances only on success.
    ///
    /// # Arguments
    /// * `BYTE_COUNT` - Number of bytes to read.
    /// * `field` - Static field label used in failure diagnostics.
    ///
    /// # Returns
    /// Exactly BYTE_COUNT bytes in source order; zero-length reads are permitted.
    ///
    /// # Errors
    /// Returns Truncated at the original cursor if bounds or addition fail.
    pub fn take<const BYTE_COUNT: usize>(
        &mut self,
        field: &'static str,
    ) -> Result<[u8; BYTE_COUNT], ParseError> {
        let error = ParseError {
            code: ErrorCode::Truncated,
            offset: self.offset,
            field,
        };
        let end_exclusive = self
            .offset
            .checked_add(BYTE_COUNT)
            .ok_or_else(|| error.clone())?;
        let slice = self.bytes.get(self.offset..end_exclusive).ok_or(error)?;
        let mut result = [0; BYTE_COUNT];
        result.copy_from_slice(slice);
        self.offset = end_exclusive;
        Ok(result)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn truncated_read_does_not_advance() {
        let mut reader = Reader::new(&[1, 2, 3]);
        assert_eq!(reader.take::<2>("first").unwrap(), [1, 2]);
        assert_eq!(reader.take::<2>("second").unwrap_err().offset, 2);
        assert_eq!(reader.take::<1>("last").unwrap(), [3]);
    }
}
