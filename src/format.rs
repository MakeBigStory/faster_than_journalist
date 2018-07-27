///Copyright reserve@feiper

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
//#[derive(Clone, Debug)]
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
#[derive(Clone, Debug)]
pub enum TextureFormat {
    //basic format
    DepthComponent = es20d::GL_DEPTH_COMPONENT as isize,
    RGB = es20d::GL_RGB as isize,
    RGBA = es20d::GL_RGBA as isize,
}

// out format, Like glTexImage2D func used
#[derive(Clone, Debug)]
pub enum Format {
    Byte = es20d::GL_BYTE as isize,
    UnsignedByte = es20d::GL_UNSIGNED_BYTE as isize,
    Short = es20d::GL_SHORT as isize,
    UnsignedShort = es20d::GL_UNSIGNED_SHORT as isize,
    Int = es20d::GL_INT as isize,
    UnsignedInt = es20d::GL_UNSIGNED_INT as isize,
    Float = es20d::GL_FLOAT as isize,
}
