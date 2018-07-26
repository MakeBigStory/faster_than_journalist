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
use color::Color;

#[derive(Clone, Debug)]
pub enum WrapMode {
    Repeat,

    MirroredRepeat,

    ClampToEdge,

    ClampToBorder,

    MirrorClampToEdge,
}


#[derive(Clone, Debug)]
pub enum FilterMode{
    Nearest,
    Linear,
    NearestMipmapNearest,
    LinearMipmapNearest,
    NearestMipmapLinear,
    LinearMipmapLinear,
}

#[derive(Debug, Clone)]
pub enum DepthFunc{
    LeQual,
    GeQual,
    Less,
    Greater,
    Equal,
    NotEqual,
    Always,
    Never,
}

#[derive(Debug, Clone)]
pub enum ComparisonMod {
    None,
    CompareRefToTexture,
}

#[derive(Debug, Clone)]
pub struct Wrap{
    pub R : WrapMode,
    pub S : WrapMode,
    pub T : WrapMode,
}

impl Wrap {
    pub fn new() -> Wrap{
        Wrap{
            R: WrapMode::Repeat,
            S: WrapMode::Repeat,
            T: WrapMode::Repeat,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub min : FilterMode,
    pub mag : FilterMode,
}

impl Filter {
    pub fn new() -> Filter {
        Filter{
            min: FilterMode::Linear,
            mag: FilterMode::Linear,
        }
    }
}

/// when texture is feed with depth component, the sampler need to
/// use this parameters
#[derive(Debug, Clone)]
pub struct SamplerComparison{
    pub  com_mod: ComparisonMod,
    pub  depth_func: DepthFunc,
}

impl SamplerComparison{
    pub fn new() -> SamplerComparison {
        SamplerComparison{
            com_mod: ComparisonMod::None,
            depth_func: DepthFunc::Less,
        }
    }

    pub fn new_with(com_mod: ComparisonMod, depth_func: DepthFunc)
        -> SamplerComparison {
        SamplerComparison {
            com_mod,
            depth_func,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Sampler {
    pub label: String,
    pub lod: Range<u32>,
    pub lod_bias: u32,
    pub wrap: Wrap,
    pub filter: Filter,
    pub edge_sampler_mod: WrapMode,
    pub anisotropic_value: u32,
    pub board_color: Color,
    pub comparison: SamplerComparison
}

impl Sampler {
    pub fn new(lab:String)->Sampler {
        let lod_range = Range{start:0, end: 1};
        Sampler{
            label:lab,
            lod: lod_range,
            lod_bias: 0,
            wrap: Wrap::new(),
            filter: Filter::new(),
            edge_sampler_mod: WrapMode::Repeat,
            anisotropic_value: 0,
            board_color: Color::new(0.0,0.0,0.0,0.0),
            comparison: SamplerComparison::new(),
        }
    }

    pub fn new_with(lab:String, lod_range:Range<u32>, wrap:Wrap,
    filter: Filter) -> Sampler{
        Sampler{
            label:lab,
            lod: lod_range,
            lod_bias: 0,
            wrap,
            filter,
            edge_sampler_mod: WrapMode::Repeat,
            anisotropic_value: 0,
            board_color: Color::new(0.0,0.0,0.0,0.0),
            comparison: SamplerComparison::new(),
        }
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

    pub fn set_lod_bias(&mut self, bias: u32) {
        self.lod_bias = bias;
    }

    pub fn edge_sampler_mode(&mut self, mode: WrapMode) {
        self.edge_sampler_mod = mode;
    }

    pub fn set_anisotropic_value(&mut self, value: u32) {
        self.anisotropic_value = value;
    }

    pub fn set_board_color(&mut self, r:f32, g:f32,b:f32,a:f32) {
        self.board_color.setColor(r,g,b,a);
    }

    pub fn set_comparison(&mut self, comparison: SamplerComparison) {
        self.comparison = comparison;
    }
}

