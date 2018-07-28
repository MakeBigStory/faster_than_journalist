use std::result::Result::Ok;
use gles::es20::wrapper::{flush, finish};

enum PixelStorage {}

enum CompressedPixelStorage {}

#[derive(Debug)]
pub(crate) struct Device {}

impl Device {
//    fn query_features() {}
//    fn query_feature() {}

    pub fn prepare(&mut self) {
    }

    pub fn flush(&mut self) {
        flush();
    }

    pub fn finish(&mut self) {
        finish();
    }
}

/*
trait ES20 {
    fn apply_pixel_storage_pack(mut self, storage: PixelStorage) -> Self;
    fn apply_pixel_storage_unpack(mut self, storage: PixelStorage) -> Self;

    fn apply_compressed_pixel_storage_pack(mut self, storage: CompressedPixelStorage) -> Self;

    fn apply_compressed_pixel_storage_unpack(mut self, storage: CompressedPixelStorage) -> Self;
}

impl ES20 for Device {
    fn apply_pixel_storage_pack(mut self, storage: PixelStorage) -> Self {
        self
    }
    fn apply_pixel_storage_unpack(mut self, storage: PixelStorage) -> Self {
        self
    }

    fn apply_compressed_pixel_storage_pack(mut self, storage: CompressedPixelStorage) -> Self {
        self
    }

    fn apply_compressed_pixel_storage_unpack(mut self, storage: CompressedPixelStorage) -> Self {
        self
    }
}

trait ES30 {}

impl Device {
    fn query_features() {}
    fn query_feature() {}

    pub fn flush() {}
    pub fn finish() {}
}
*/