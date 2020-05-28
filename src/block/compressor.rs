use crate::map_error_code;
use crate::dict::EncoderDictionary;

use std::io;
use zstd_safe;

/// Allows to compress independently multiple blocks of data.
///
/// This reduces memory usage compared to calling `compress` multiple times.
/// The compressed blocks are still completely independent.
pub struct Compressor<'a> {
    context: zstd_safe::CCtx<'static>,
    dict: EncoderDictionary<'a>,
}

impl<'a> Compressor<'a> {
    /// Creates a new zstd compressor
    pub fn new(level: i32) -> Self {
        Compressor::with_dict(&[], level)
    }

    /// Creates a new zstd compressor, using the given dictionary.
    pub fn with_dict(dict: &'a [u8], level: i32) -> Self {
        Compressor {
            context: zstd_safe::create_cctx(),
            dict: EncoderDictionary::new(dict, level),
        }
    }

    pub fn compress_to_buffer(
        &mut self,
        source: &[u8],
        destination: &mut [u8],
    ) -> io::Result<usize> {
        zstd_safe::compress_using_cdict(
            &mut self.context,
            destination,
            source,
            self.dict.as_cdict(),
        )
        .map_err(map_error_code)
    }

    pub fn compress(
        &mut self,
        data: &[u8],
    ) -> io::Result<Vec<u8>> {
        // We allocate a big buffer, slightly larger than the input data.
        let buffer_len = zstd_safe::compress_bound(data.len());
        let mut buffer = Vec::with_capacity(buffer_len);
        unsafe {
            // Use all capacity.
            // Memory may not be initialized, but we won't read it.
            buffer.set_len(buffer_len);
            let len = self.compress_to_buffer(data, &mut buffer[..])?;
            buffer.set_len(len);
        }

        // Should we shrink the vec? Meh, let the user do it if he wants.
        Ok(buffer)
    }
}

fn _assert_traits() {
    fn _assert_send<T: Send>(_: T) {}

    _assert_send(Compressor::new(0));
}
