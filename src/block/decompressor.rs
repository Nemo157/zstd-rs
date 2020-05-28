use crate::map_error_code;
use crate::dict::DecoderDictionary;

use std::io;
use zstd_safe;

/// Allows to decompress independently multiple blocks of data.
///
/// This reduces memory usage compared to calling `decompress` multiple times.
pub struct Decompressor<'a> {
    context: zstd_safe::DCtx<'static>,
    dict: DecoderDictionary<'a>,
}

impl<'a> Decompressor<'a> {
    /// Creates a new zstd decompressor.
    pub fn new() -> Self {
        Decompressor::with_dict(&[])
    }

    /// Creates a new zstd decompressor, using the given dictionary.
    pub fn with_dict(dict: &'a [u8]) -> Self {
        Decompressor {
            context: zstd_safe::create_dctx(),
            dict: DecoderDictionary::new(dict),
        }
    }

    /// Deompress a single block of data to the given destination buffer.
    ///
    /// Returns the number of bytes written, or an error if something happened
    /// (for instance if the destination buffer was too small).
    pub fn decompress_to_buffer(
        &mut self,
        source: &[u8],
        destination: &mut [u8],
    ) -> io::Result<usize> {
        zstd_safe::decompress_using_ddict(
            &mut self.context,
            destination,
            source,
            self.dict.as_ddict(),
        )
        .map_err(map_error_code)
    }

    /// Decompress a block of data, and return the result in a `Vec<u8>`.
    ///
    /// The decompressed data should be less than `capacity` bytes,
    /// or an error will be returned.
    pub fn decompress(
        &mut self,
        data: &[u8],
        capacity: usize,
    ) -> io::Result<Vec<u8>> {
        let mut buffer = Vec::with_capacity(capacity);
        unsafe {
            buffer.set_len(capacity);
            let len = self.decompress_to_buffer(data, &mut buffer[..])?;
            buffer.set_len(len);
        }
        Ok(buffer)
    }
}

fn _assert_traits() {
    fn _assert_send<T: Send>(_: T) {}

    _assert_send(Decompressor::new());
}
