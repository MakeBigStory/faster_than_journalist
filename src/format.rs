use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es30;
use gles::es31;
use gles::es32;

use std::ptr;
use std::fmt;
use std::fmt::Formatter;
use std::error::Error;
use std::mem;
use std::ops::Range;

/// the pixel data outside, only 6 symbolic values are accepted
/// RED,RG,RGB,RGBA
//#[derive(Copy, Clone, Debug)]
/*pub enum TextureFormat {
    //basic format
    DepthComponent,
    DepthStencil,
    RED,
    RG,
    RGB,
    BGR,
    RGBA,
    BGRA,


    ALPHA,
    ALPHA4,
    ALPHA8,
    ALPHA12,
    ALPHA16,
    CompressedAlpha,
    CompressedLuminance,
    CompressedLuminanceAlpha,
    CompressedIntensity,
    DepthComponent16,
    DepthComponent24,
    DepthComponent32,
    LUMINANCE,
    LUMINANCE4,
    LUMINANCE8,
    LUMINANCE12,
    LUMINANCE16,
    LuminanceAlpha,
    Luminance4Alpha4,
    Luminance6Alpha2,
    Luminance8Alpha8,
    Luminance12Alpha4,
    Luminance12Alpha12,
    Luminance16Alpha16,
    INTENSITY,
    INTENSITY4,
    INTENSITY8,
    INTENSITY12,
    INTENSITY16,
    RGB16,
    SLUMINANCE,
    SLUMINANCE8,
    SluminanceAlpha,
    Sluminance8Alpha8,
    SRGB,
    SrgbAlpha,

    //sized internal formats
    R8,
    R8Snorm,
    R16,
    R16Snorm,
    RG8,
    Rg8Snorm,
    RG16,
    Rg16Snorm,
    R3G3B2,
    RGB4,
    RGB5,
    RGB8,
    Rgb8Snorm,
    RGB10,
    RGB12,
    Rgb16Snorm,
    RGBA2,
    RGBA4,
    Rgb5A1,
    RGBA8,
    Rgba8Snorm,
    Rgb10A2,
    Rgb10A2ui,
    RGBA12,
    RGBA16,
    SRGB8,
    Srgb8Alpha8,
    R16F,
    RG16F,
    RGB16F,
    RGBA16F,
    R32F,
    RG32F,
    RGB32F,
    R11fG11fB10f,
    Rgb9E5,
    R8I,
    R8UI,
    R16I,
    R16UI,
    R32I,
    R32UI,
    RG8I,
    RG8UI,
    RG16I,
    RG16UI,
    RG32I,
    RG32UI,
    RGB8I,
    RGB8UI,
    RGB16I,
    RGB16UI,
    RGB32I,
    RGB32UI,
    RGBA8I,
    RGBA8UI,
    RGBA16I,
    RGBA16UI,
    RGBA32I,
    RGBA32UI,

    //compressed internal formats
    CompressedRed,
    CompressedRg,
    CompressedRgb,
    CompressedRgba,
    CompressedSrgb,
    CompressedSrgbAlpha,
    CompressedRedRgtc1,
    CompressedSignedRedRgtc1,
    CompressedRgRgtc2,
    CompressedSignedRgRgtc2,
    CompressedRgbaBptcUnorm,
    CompressedSrgbAlphaBptcUnorm,
    CompressedRgbBptcSignedFloat,
    CompressedRgbBptcUnsignedFloat,
}
*/
#[derive(Copy, Clone, Debug)]
pub enum TextureFormat {
    //basic format
    DepthComponent = es20d::GL_DEPTH_COMPONENT as isize,
    RGB = es20d::GL_RGB as isize,
    RGBA = es20d::GL_RGBA as isize,
}

// out format, Like glTexImage2D func used
#[derive(Copy, Clone, Debug)]
pub enum Format {
    Byte = es20d::GL_BYTE as isize,
    UnsignedByte = es20d::GL_UNSIGNED_BYTE as isize,
    Short = es20d::GL_SHORT as isize,
    UnsignedShort = es20d::GL_UNSIGNED_SHORT as isize,
    Int = es20d::GL_INT as isize,
    UnsignedInt = es20d::GL_UNSIGNED_INT as isize,
    Float = es20d::GL_FLOAT as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum BlendFactor {
    //todo: 只包含了20 blend
    Zero = es20d::GL_ZERO as isize,
    One = es20d::GL_ONE as isize,
    SrcColor = es20d::GL_SRC_COLOR as isize,
    OneMinusSrcColor = es20d::GL_ONE_MINUS_SRC_COLOR as isize,
    DstColor = es20d::GL_DST_COLOR as isize,
    OneMinusDstColor = es20d::GL_ONE_MINUS_DST_COLOR as isize,
    SrcAlpha = es20d::GL_SRC_ALPHA as isize,
    OneMinusSrcAlpha = es20d::GL_ONE_MINUS_SRC_ALPHA as isize,
    DstAlpha = es20d::GL_DST_ALPHA as isize,
    OneMinusDstAlpha = es20d::GL_ONE_MINUS_DST_ALPHA as isize,
    ConstColor = es20d::GL_CONSTANT_COLOR as isize,
    OneMinusConstColor = es20d::GL_ONE_MINUS_CONSTANT_COLOR as isize,
    ConstAlpha = es20d::GL_CONSTANT_ALPHA as isize,
    OneMinusConstAlpha = es20d::GL_ONE_MINUS_CONSTANT_ALPHA as isize,
    SrcAlphaSaturate = es20d::GL_SRC_ALPHA_SATURATE as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum BlendMod {
    Add,
    Sub,
    RevSub,
    Min,
    Max,
}

//todo: separate fun要不要启用？
#[derive(Copy, Clone, Debug)]
pub enum Comparison {
    Always = es20d::GL_ALWAYS as isize,
    Never = es20d::GL_NEVER as isize,
    Less = es20d::GL_LESS as isize,
    Equal = es20d::GL_EQUAL as isize,
    LeEqual = es20d::GL_LEQUAL as isize,
    Greater = es20d::GL_GREATER as isize,
    NotEqual = es20d::GL_NOTEQUAL as isize,
    GeEqual = es20d::GL_GEQUAL as isize,
}

//todo: separate 函数要不要使用
#[derive(Copy, Clone, Debug)]
pub enum StencilOp {
    Keep = es20d::GL_KEEP as isize,
    Zero = es20d::GL_ZERO as isize,
    Replace = es20d::GL_REPLACE as isize,
    Incr = es20d::GL_INCR as isize,
    IncrWrap = es20d::GL_INCR_WRAP as isize,
    Decr = es20d::GL_DECR as isize,
    DecrWrap = es20d::GL_DECR_WRAP as isize,
    Inver = es20d::GL_INVERT as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum CullMode {
    Front = es20d::GL_FRONT as isize,
    Back = es20d::GL_BACK as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum FrontFaceMode {
    CW = es20d::GL_CW as isize,
    CCW = es20d::GL_CCW as isize,
}

/*
//opengles don't support
#[derive(Copy, Clone, Debug)]
pub enum PolygonMode{
    Point,
    Line,
    Fill,
}
*/