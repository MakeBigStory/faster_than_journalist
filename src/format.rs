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

//todo : internal format 和 out format 要不要区分？
#[derive(Copy, Clone, Debug)]
pub enum DataFormat {
    //basic format
    DepthComponent = es20d::GL_DEPTH_COMPONENT as isize,
    RGB = es20d::GL_RGB as isize,
    RGBA = es20d::GL_RGBA as isize,
}

// todo: Data Kind是否需要分开？使用方有 Index Kind, Attribute Kind, Out Format Kind
#[derive(Copy, Clone, Debug, Hash)]
pub enum DataKind {
    Byte = es20d::GL_BYTE as isize,
    UnsignedByte = es20d::GL_UNSIGNED_BYTE as isize,
    Short = es20d::GL_SHORT as isize,
    UnsignedShort = es20d::GL_UNSIGNED_SHORT as isize,
    Int = es20d::GL_INT as isize,
    UnsignedInt = es20d::GL_UNSIGNED_INT as isize,
    Float = es20d::GL_FLOAT as isize,
    Bool = es20d::GL_BOOL as isize,
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
    FrontAndBack = es20d::GL_FRONT_AND_BACK as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum FrontFaceMode {
    CW = es20d::GL_CW as isize,
    CCW = es20d::GL_CCW as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum TextureType {
    Texture2d = es20d::GL_TEXTURE_2D as isize,
    Texture2dArray = es30d::GL_TEXTURE_2D_ARRAY as isize,
    Texture3d = es30d::GL_TEXTURE_3D as isize,
    TextureCube = es20d::GL_TEXTURE_CUBE_MAP as isize,
    TextureCubeArray = es32d::GL_TEXTURE_CUBE_MAP_ARRAY as isize,

    //todo: compress texture??
}

#[derive(Copy, Clone, Debug)]
pub enum SwizzleMode {
    SwizzleR = es30d::GL_RED as isize,
    SwizzleG = es30d::GL_GREEN as isize,
    SwizzleB = es30d::GL_BLUE as isize,
    SwizzleA = es20d::GL_ALPHA as isize,
    SwizzleOne = es20d::GL_ONE as isize,
    SwizzleZero = es20d::GL_ZERO as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum DrawMode{
    Points = es20d::GL_POINTS as isize,
    LineStrip = es20d::GL_LINE_STRIP as isize,
    LineLoop  = es20d::GL_LINE_LOOP as isize,
    Lines = es20d::GL_LINES as isize,
    //    LineStripAdjacency,
//    LineAdjacency,
    TriangleStrip = es20d::GL_TRIANGLE_STRIP as isize,
    TriangleFan = es20d::GL_TRIANGLE_FAN as isize,
    Triangles = es20d::GL_TRIANGLES as isize,
//    TriangleStripAdjacency,
//    TrianglesAdjacency,
//    Patches,
}

pub enum ErrorType {
    NoError = es20d::GL_NO_ERROR as isize,
    InvalidEnum = es20d::GL_INVALID_ENUM as isize,
    InvalidValue = es20d::GL_INVALID_VALUE as isize,
    InvalidOperation = es20d::GL_INVALID_OPERATION as isize,
    InvalidFramebufferOperation = es20d::GL_INVALID_FRAMEBUFFER_OPERATION as isize,
    //    StackUnderflow,
//    StackOverflow,
    OutOfMemory = es20d::GL_OUT_OF_MEMORY as isize,
}

// TODO: 为什么不能直接 as u32?
#[derive(Copy, Debug, Clone, Hash)]
pub enum BufferType {
    ArrayBuffer = es20d::GL_ARRAY_BUFFER as isize,
    ElementArrayBuffer = es20d::GL_ELEMENT_ARRAY_BUFFER as isize,

    PixelPackBuffer = es30d::GL_PIXEL_PACK_BUFFER as isize,
    PixelUnpackBuffer = es30d::GL_PIXEL_UNPACK_BUFFER as isize,
    UniformBuffer = es30d::GL_UNIFORM_BUFFER as isize,

    TransformFeedbackBuffer = es30d::GL_TRANSFORM_FEEDBACK_BUFFER as isize,
    DrawIndirectBuffer = es31d::GL_DRAW_INDIRECT_BUFFER as isize,

    //feiper: for copy buffer to another
    //COPY_READ_BUFFER,
    //COPY_WRITE_BUFFER,

}

#[derive(Copy, Debug, Clone, Hash)]
pub enum BufferUsage {
    /// Set once by the application and used infrequently for drawing.
    StreamDraw = es20d::GL_STREAM_DRAW as isize,
    /// Set once as output from an OpenGL command and used infequently for drawing.
    StreamRead = es30d::GL_STREAM_READ as isize,
    /// Set once as output from an OpenGL command and used infrequently for drawing or copying to other buffers.
    StreamCopy = es30d::GL_STREAM_COPY as isize,

    /// Set once by the application and used frequently for drawing. A good default choice if you are not sure.
    StaticDraw = es20d::GL_STATIC_DRAW as isize,
    /// Set once as output from an OpenGL command and queried many times by the application.
    StaticRead = es30d::GL_STATIC_READ as isize,
    /// Set once as output from an OpenGL command and used frequently for drawing or copying to other buffers.
    StaticCopy = es30d::GL_STATIC_COPY as isize,

    /// Updated frequently by the application and used frequently for drawing or copying to other images.
    DynamicDraw = es20d::GL_DYNAMIC_DRAW as isize,
    /// Updated frequently as output from OpenGL command and queried many times from the application.
    DynamicRead = es30d::GL_DYNAMIC_READ as isize,
    /// Updated frequently as output from OpenGL command and used frequently for drawing or copying to other images.
    DynamicCopy = es30d::GL_DYNAMIC_COPY as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum AttachmentUsage {
    ColorAttach,
    DepthAttach,
    StencilAttach,
    DepthStencil,
}

#[derive(Copy, Clone, Debug)]
pub enum FrameBufferStatus {
    /** The framebuffer is complete */
    Complete = es20d::GL_FRAMEBUFFER_COMPLETE as isize,

    /** Any of the attachment points are incomplete */
    IncompleteAttachment = es20d::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT as isize,

    /** The framebuffer does not have at least one image attached to it */
    IncompleteMissingAttachment = es20d::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT as isize,

    /**
     * Combination of internal formats of the attached images violates
     * an implementation-dependent set of restrictions.
     */
    Unsupported = es20d::GL_FRAMEBUFFER_UNSUPPORTED as isize,

    /// Sample count or locations are not the same for all attached images.
    IncompleteMultisample = es30d::GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE as isize,
    // todo: ES 2.0 iOS
    //IncompleteMultisample = GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE,
}

#[derive(Copy, Clone, Debug, Hash)]
pub enum AttributeKind{
    Bool = es20d::GL_BOOL as isize,
    Int = es20d::GL_INT as isize,
    Float = es20d::GL_FLOAT as isize,
    BoolVec2 = es20d::GL_BOOL_VEC2 as isize,
    IntVec2 = es20d::GL_INT_VEC2 as isize,
    FloatVec2 = es20d::GL_FLOAT_VEC2 as isize,
    BoolVec3 = es20d::GL_BOOL_VEC3 as isize,
    IntVec3 = es20d::GL_INT_VEC3 as isize,
    FloatVec3 = es20d::GL_FLOAT_VEC3 as isize,
    BoolVec4 = es20d::GL_BOOL_VEC4 as isize,
    IntVec4 = es20d::GL_INT_VEC4 as isize,
    FloatVec4 = es20d::GL_FLOAT_VEC4 as isize,
}

impl AttributeKind {
    #[inline]
    pub fn item_data(&self) -> (usize, DataKind) {
        match self {
            &AttributeKind::Bool => (1, DataKind::Bool),
            &AttributeKind::Int => (1, DataKind::Int),
            &AttributeKind::Float => (1, DataKind::Float),

            &AttributeKind::BoolVec2 => (2, DataKind::Bool),
            &AttributeKind::IntVec2 => (2, DataKind::Int),
            &AttributeKind::FloatVec2 => (2, DataKind::Float),

            &AttributeKind::BoolVec3 => (3, DataKind::Bool),
            &AttributeKind::IntVec3 => (3, DataKind::Int),
            &AttributeKind::FloatVec3 => (3, DataKind::Float),

            &AttributeKind::BoolVec4 => (4, DataKind::Bool),
            &AttributeKind::IntVec4 => (4, DataKind::Int),
            &AttributeKind::FloatVec4 => (4, DataKind::Float),
        }
    }
}


//todo:Attribute 和uniform要不要分开？ 分开会有很多冗余的
pub enum UniformKind {
    Bool = es20d::GL_BOOL as isize,
    Int = es20d::GL_INT as isize,
    Float = es20d::GL_FLOAT as isize,
    BoolVec2 = es20d::GL_BOOL_VEC2 as isize,
    IntVec2 = es20d::GL_INT_VEC2 as isize,
    FloatVec2 = es20d::GL_FLOAT_VEC2 as isize,
    BoolVec3 = es20d::GL_BOOL_VEC3 as isize,
    IntVec3 = es20d::GL_INT_VEC3 as isize,
    FloatVec3 = es20d::GL_FLOAT_VEC3 as isize,
    BoolVec4 = es20d::GL_BOOL_VEC4 as isize,
    IntVec4 = es20d::GL_INT_VEC4 as isize,
    FloatVec4 = es20d::GL_FLOAT_VEC4 as isize,

    Mat2 = es20d::GL_FLOAT_MAT2 as isize,
    Mat3 = es20d::GL_FLOAT_MAT3 as isize,
    Mat4 = es20d::GL_FLOAT_MAT4 as isize,

    Sampler2D = es20d::GL_SAMPLER_2D as isize,
    //SamplerCube = es20d::GL_SAMPLER_CUBE as isize,
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