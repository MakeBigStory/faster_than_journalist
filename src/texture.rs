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

#[derive(Clone, Debug)]
pub enum TextureType{
    Texture2d,
    Texture2dArray,
    Texture3d,
    TextureCube,
    TextureCubeArray,
}

pub enum SwizzleMode{
    SwizzleR = es30d::GL_RED as isize,
    SwizzleG = es30d::GL_GREEN as isize,
    SwizzleB = es30d::GL_BLUE as isize,
    SwizzleA = es20d::GL_ALPHA as isize,
    SwizzleOne = es20d::GL_ONE as isize,
    SwizzleZero = es20d::GL_ZERO as isize,
}

pub struct Texture {
    label : String,
}

/*
impl Texture {
    fn set_minification_filter() -> Self {

    }
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
*/