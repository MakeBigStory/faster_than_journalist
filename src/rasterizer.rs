
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
use format::*;
use color::*;

//todo: 深度冲突可以用polygon offset 但不知道es2.0支持到什么模式。
#[derive(Clone, Debug)]
pub struct DepthTest {
    pub on: bool,
    pub depth_mask: bool,
    pub func: Comparison,
}

impl DepthTest {
    fn new() -> DepthTest{
        DepthTest{
            on: true,
            depth_mask: true,
            func: DepthOp::Less,
        }
    }
}

#[derive(Clone, Debug)]
pub struct StencilTest{
    pub on: bool,
    //todo: separate front and back ?? glStencilOpSeparate
    //pub front_back_sep: bool,

    pub stencil_fail: StencilOp,
    pub depth_fail: StencilOp,
    pub pass: StencilOp,

    pub func: Comparison,
    pub ref_value: u32,
    pub mask: u32,
}

impl StencilTest{
    fn new() -> StencilTest{
        StencilTest{
            on: false,
            stencil_fail: StencilOp::Keep,
            depth_fail: StencilOp::Keep,
            pass: StencilOp::Keep,
            func: Comparison::Less,
            ref_value: 1,
            mask: 0xffffffff;
        }
    }
}

#[derive(Clone, Debug)]
pub struct Blend {
    pub on: bool,
    pub blend_mod: BlendMod,

    pub rgb_alpha_sep: bool,

    //factor
    pub src_rgba_factor: BlendFactor,
    pub dst_rgba_factor: BlendFactor,

    //separate
    pub src_rgb_factor: BlendFactor,
    pub src_alpha_factor: BlendFactor,
    pub dst_rgb_factor: BlendFactor,
    pub dst_alpha_factor: BlendFactor,

    pub const_color:Color,
}

impl Blend {
    fn new() -> Blend {
        Blend {
            on: false,
            blend_mod: BlendMod::Add,
            rgb_alpha_sep: false,
            src_rgba_factor: BlendFactor::One,
            dst_rgba_factor: BlendFactor::One,
            src_rgb_factor: BlendFactor::One,
            src_alpha_factor: BlendFactor::One,
            dst_rgb_factor: BlendFactor::One,
            dst_alpha_factor: BlendFactor::One,
            const_color: Color::new(1.0, 1.0, 1.0, 1.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Cull{
    pub on: bool,
    pub front_face_mode: FrontFaceMode,
    pub cull_mode: CullMode,
}

impl Cull {
    fn new() -> Cull{
        Cull{
            on: false,
            front_face_mode: FrontFaceMode::CCW,
            cull_mode: CullMode::Back,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Rasterizer{
    
}