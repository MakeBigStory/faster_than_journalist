use gles::es20;

enum PixelStorage {}

enum CompressedPixelStorage {}

#[derive(Debug)]
pub(crate) struct Device {}

impl Device {
//    fn query_features() {}
//    fn query_feature() {}

    pub fn prepare() -> Result<u32, &str> {
        Ok(0)
    }

    pub fn flush() -> Result<u32, &str> {
        es20::wrapper::

        Ok(0)
    }

    pub fn finish() -> Result<u32, &str> {
        Ok(0)
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