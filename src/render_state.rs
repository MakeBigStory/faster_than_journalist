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

use super::rasterizer::*;
use super::color::Color;

#[derive(Debug)]
pub(crate) struct RenderState {
    blend_mod: Blend,
    depth_mode: DepthTest,
    stencil_mode: StencilTest,
    cull_mode: Cull,
    color_mask: ColorMask,
    polygon_offset: PolygonOffset,
}

impl RenderState {
    pub fn new() -> RenderState {
        RenderState {
            blend_mod: Blend::new(),
            depth_mode: DepthTest::new(),
            stencil_mode: StencilTest::new(),
            cull_mode: Cull::new(),
            color_mask: ColorMask::new(),
            polygon_offset: PolygonOffset::new(),
        }
    }
}