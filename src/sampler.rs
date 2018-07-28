use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es30;
use gles::es31;
use gles::es32;

use color::Color;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::mem;
use std::ops::Range;
use std::ptr;

fn glenum_to_isize(value: es20d::GLenum) -> isize {
    value as isize
}

#[derive(Copy, Clone, Debug)]
pub enum WrapMode {
    /// Repeat texture
    Repeat = es20d::GL_REPEAT as isize,
    /// Repeat mirrored texture.
    MirroredRepeat = es20d::GL_MIRRORED_REPEAT as isize,
    ///Clamp to edge. Coordinates out of the range will be clamped to first / last column / row in given direction.
    ClampToEdge = es20d::GL_CLAMP_TO_EDGE as isize,
    /// Clamp to border color. Coordinates out of range will be clamped to border color.
    ClampToBorder = es32d::GL_CLAMP_TO_BORDER as isize,
    //// Mirror the texture once in negative coordinates and clamp to edge after that.
//    MirrorClampToEdge,
}


#[derive(Copy, Clone, Debug)]
pub enum FilterMode {
    /// Nearest neighbor filtering.
    Nearest = es20d::GL_NEAREST as isize,
    /// Linear interpolation filtering.
    Linear = es20d::GL_LINEAR as isize,

    /// Texture sampler mip level selection
    NearestMipmapNearest = es20d::GL_NEAREST_MIPMAP_NEAREST as isize,
    LinearMipmapNearest = es20d::GL_LINEAR_MIPMAP_NEAREST as isize,
    NearestMipmapLinear = es20d::GL_NEAREST_MIPMAP_LINEAR as isize,
    LinearMipmapLinear = es20d::GL_LINEAR_MIPMAP_LINEAR as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum DepthFunc {
    LeQual = es20d::GL_LEQUAL as isize,
    GeQual = es20d::GL_GEQUAL as isize,
    Less = es20d::GL_LESS as isize,
    Greater = es20d::GL_GREATER as isize,
    Equal = es20d::GL_EQUAL as isize,
    NotEqual = es20d::GL_NOTEQUAL as isize,
    Always = es20d::GL_ALWAYS as isize,
    Never = es20d::GL_NEVER as isize,
}

#[derive(Copy, Clone, Debug)]
pub enum ComparisonMod {
    None = es20d::GL_NONE as isize,
    CompareRefToTexture = es30d::GL_COMPARE_REF_TO_TEXTURE as isize,
}


#[derive(Clone, Debug)]
pub struct Wrap {
    pub R: WrapMode,
    pub S: WrapMode,
    pub T: WrapMode,
}

impl Wrap {
    pub fn new() -> Wrap {
        Wrap {
            R: WrapMode::Repeat,
            S: WrapMode::Repeat,
            T: WrapMode::Repeat,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Filter {
    pub min: FilterMode,
    pub mag: FilterMode,
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            min: FilterMode::Linear,
            mag: FilterMode::Linear,
        }
    }
}

/// when texture is feed with depth component, the sampler need to
/// use this parameters
#[derive(Debug, Clone)]
pub struct SamplerComparison {
    pub com_mod: ComparisonMod,
    pub depth_func: DepthFunc,
}

impl SamplerComparison {
    pub fn new() -> SamplerComparison {
        SamplerComparison {
            com_mod: ComparisonMod::None,
            depth_func: DepthFunc::Less,
        }
    }

    pub fn new_with(com_mod: ComparisonMod, depth_func: DepthFunc) -> SamplerComparison {
        SamplerComparison {
            com_mod,
            depth_func,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SamplerDesc {
    pub label: String,
    pub lod: Range<u32>,
    pub lod_bias: f32,
    pub wrap: Wrap,
    pub filter: Filter,
    pub edge_sampler_mod: WrapMode,
    pub anisotropic_value: f32,
    pub board_color: Color,
    pub comparison: SamplerComparison,
}

impl SamplerDesc {
    pub fn new(label: String) -> SamplerDesc {
        let lod_range = Range { start: 0, end: 1 };
        SamplerDesc {
            label,
            lod: lod_range,
            lod_bias: 0f32,
            wrap: Wrap::new(),
            filter: Filter::new(),
            edge_sampler_mod: WrapMode::Repeat,
            anisotropic_value: 0f32,
            board_color: Color::new(0.0, 0.0, 0.0, 0.0),
            comparison: SamplerComparison::new(),
        }
    }

    pub fn new_with(
        label: String,
        lod_range: Range<u32>,
        wrap: Wrap,
        filter: Filter,
    ) -> SamplerDesc {
        SamplerDesc {
            label,
            lod: lod_range,
            lod_bias: 0f32,
            wrap,
            filter,
            edge_sampler_mod: WrapMode::Repeat,
            anisotropic_value: 0f32,
            board_color: Color::new(0.0, 0.0, 0.0, 0.0),
            comparison: SamplerComparison::new(),
        }
    }

    fn set_label(&mut self, label: String) {
        self.label = label;
    }

    fn get_label(&self) -> &String {
        &self.label
    }

    pub fn set_wrap(&mut self, wrap: Wrap) {
        self.wrap = wrap;
    }

    pub fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
    }

    pub fn set_lod(&mut self, lod: Range<u32>) {
        self.lod = lod;
    }

    pub fn set_lod_bias(&mut self, bias: f32) {
        self.lod_bias = bias;
    }

    pub fn edge_sampler_mode(&mut self, mode: WrapMode) {
        self.edge_sampler_mod = mode;
    }

    pub fn set_anisotropic_value(&mut self, value: f32) {
        self.anisotropic_value = value;
    }

    pub fn set_board_color(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.board_color.setColor(r, g, b, a);
    }

    pub fn set_comparison(&mut self, comparison: SamplerComparison) {
        self.comparison = comparison;
    }
}

#[derive(Clone, Debug)]
pub struct Sampler {
    desc: SamplerDesc,
    raw: Option<u32>,
}

impl Sampler {
    pub fn new(label: String) -> Sampler {
        let mut name = 0 as es20d::GLuint;
        unsafe {
            es30::ffi::glGenSamplers(1, &mut name);
        }
        Sampler {
            desc: SamplerDesc::new(label),
            raw: Some(name),
        }
    }

    pub fn new_with(desc: &SamplerDesc) -> Sampler {
        let mut name = 0 as es20d::GLuint;
        unsafe {
            es30::ffi::glGenSamplers(1, &mut name);
        }
        Sampler {
            desc: desc.clone(),
            raw: Some(name),
        }
    }

    fn write_desc(&self) {
        let name = match self.raw {
            Some(data) => data,
            None => panic!("Error: Sampler: write_desc , the id is null"),
        };

        unsafe {
            //TODO: set anisotropy in es?
            //es30::ffi::glSamplerParameterf(name, gl::TEXTURE_MAX_ANISOTROPY_EXT, sel);
            es30::ffi::glSamplerParameteri(
                name,
                es20d::GL_TEXTURE_MIN_FILTER,
                self.desc.filter.min.clone() as _,
            );
            es30::ffi::glSamplerParameteri(
                name,
                es20d::GL_TEXTURE_MAG_FILTER,
                self.desc.filter.mag.clone() as _,
            );

            es30::ffi::glSamplerParameteri(
                name,
                es20d::GL_TEXTURE_WRAP_S,
                self.desc.wrap.S.clone() as _,
            );
            es30::ffi::glSamplerParameteri(
                name,
                es20d::GL_TEXTURE_WRAP_T,
                self.desc.wrap.T.clone() as _,
            );
            es30::ffi::glSamplerParameteri(
                name,
                es30d::GL_TEXTURE_WRAP_R,
                self.desc.wrap.R.clone() as _,
            );

            //TODO: set lod bias and board color in es?
            // es30::ffi::glSamplerParameterf(name, es32d::GL_TEXTURE_LOD_BIAS,
            //                                self.desc.lod_bias);
            // es30::ffi::glSamplerParameterfv(name, es32d::GL_TEXTURE_BORDER_COLOR,
            //                                 &border[0]);

            es30::ffi::glSamplerParameterf(
                name,
                es30d::GL_TEXTURE_MIN_LOD,
                self.desc.lod.start as _,
            );
            es30::ffi::glSamplerParameterf(name, es30d::GL_TEXTURE_MAX_LOD, self.desc.lod.end as _);

            es30::ffi::glSamplerParameteri(
                name,
                es30d::GL_TEXTURE_COMPARE_MODE,
                self.desc.comparison.com_mod.clone() as _,
            );
            es30::ffi::glSamplerParameteri(
                name,
                es30d::GL_TEXTURE_COMPARE_FUNC,
                self.desc.comparison.depth_func.clone() as _,
            );
        }
    }

    //todo: maybe we need add some function for sampler operation directly. like set_lod_bias
}
