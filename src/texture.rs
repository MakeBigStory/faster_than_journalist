struct Texture {}

enum TextureFormat {
    //Red = GL_RED,
//Red = GL_RED_EXT,

//R8 = GL_R8,
//R8 = GL_R8_EXT,

//RG = GL_RG,
//RG = GL_RG_EXT,

//RG8 = GL_RG8,
//RG8 = GL_RG8_EXT,

//RGB = GL_RGB,


//RGB8 = GL_RGB8,
//RGB8 = GL_RGB8_OES,


//RGBA = GL_RGBA,
//RGBA8 = GL_RGBA8,
//RGBA8 = GL_RGBA8_OES,

//    R8Snorm = GL_R8_SNORM,

//RG8Snorm = GL_RG8_SNORM,

//RGB8Snorm = GL_RGB8_SNORM,

//RGBA8Snorm = GL_RGBA8_SNORM,


//R16 = GL_R16,

//RG16 = GL_RG16,

//RGB16 = GL_RGB16,

//RGBA16 = GL_RGBA16,

//R16Snorm = GL_R16_SNORM,

//RG16Snorm = GL_RG16_SNORM,

//RGB16Snorm = GL_RGB16_SNORM,

//RGBA16Snorm = GL_RGBA16_SNORM,
}

impl Texture {
    fn set_minification_filter() -> Self {}
    fn set_magnification_filter() -> Self {}
    fn set_min_lod() -> Self {}
    fn set_max_lod() -> Self {}
    fn set_lod_bias() -> Self {}
    fn set_wrapping() -> Self {}
    fn set_max_anisotropy() -> Self {}
    fn set_image() -> Self {}
    fn set_sub_image() -> Self {}
    fn set_compressed_image() -> Self {}
    fn set_compressed_sub_image() -> Self {}
    fn generate_mipmap() -> Self {}
}
