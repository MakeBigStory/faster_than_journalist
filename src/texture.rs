

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
use sampler::*;
use format::*;

#[derive(Copy, Clone, Debug)]
pub enum TextureType{
    Texture2d = es20d::GL_TEXTURE_2D as isize,
    Texture2dArray = es30d::GL_TEXTURE_2D_ARRAY as isize,
    Texture3d = es30d::GL_TEXTURE_3D as isize,
    TextureCube = es20d::GL_TEXTURE_CUBE_MAP as isize,
    TextureCubeArray = es32d::GL_TEXTURE_CUBE_MAP_ARRAY as isize,

    //todo: compress texture??
}

#[derive(Copy, Clone, Debug)]
pub enum SwizzleMode{
    SwizzleR = es30d::GL_RED as isize,
    SwizzleG = es30d::GL_GREEN as isize,
    SwizzleB = es30d::GL_BLUE as isize,
    SwizzleA = es20d::GL_ALPHA as isize,
    SwizzleOne = es20d::GL_ONE as isize,
    SwizzleZero = es20d::GL_ZERO as isize,
}

#[derive( Clone, Debug)]
pub struct Swizzle{
    R: SwizzleMode,
    G: SwizzleMode,
    B: SwizzleMode,
    A: SwizzleMode,
}

impl Swizzle {
    fn new() -> Swizzle{
        Swizzle{
            R: SwizzleMode::SwizzleR,
            G: SwizzleMode::SwizzleG,
            B: SwizzleMode::SwizzleB,
            A: SwizzleMode::SwizzleA,
        }
    }

    fn new_with(swizzle: &[SwizzleMode;4]) -> Swizzle {
        Swizzle {
            R : swizzle[0].clone(),
            G : swizzle[1].clone(),
            B : swizzle[2].clone(),
            A : swizzle[3].clone(),
        }
    }
}

#[derive( Clone, Debug)]
pub struct Extend<T> {
    pub width: T,
    pub height: T,
    pub depth: T,
}

impl<T> Extend<T> {
    pub fn new(width:T, height: T, depth:T) -> Extend<T> {
        Extend{
            width,
            height,
            depth,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextureDesc{
    pub label: String,
    pub texture_type: TextureType,
    pub wrap: Wrap,
    pub filter: Filter,
    pub swizzle: Swizzle,
    pub use_mip_map: bool,
    pub level: u32,
    pub size: Extend<u32>,
    pub format: TextureFormat,
}

impl TextureDesc {
    pub fn new(label: String, size: Extend<u32>) -> TextureDesc {
        TextureDesc {
            label,
            texture_type: TextureType::Texture2d,
            wrap: Wrap::new(),
            filter: Filter::new(),
            swizzle: Swizzle::new(),
            use_mip_map: false,
            level: 0,
            size,
            format: TextureFormat::RGBA,
        }
    }

    pub fn new_with(label: String, wrap:Wrap, filter: Filter,
                    texture_type: TextureType, size: Extend<u32>)
        -> TextureDesc {
        TextureDesc {
            label,
            texture_type,
            wrap,
            filter: Filter::new(),
            swizzle: Swizzle::new(),
            use_mip_map: false,
            level: 0,
            size,
            format: TextureFormat::RGBA
        }
    }

    fn set_lable(&mut self, label: String) {
        self.label = label;
    }

    fn get_lable(&self) -> &String {
        &self.label
    }

    fn set_texture_type(&mut self, texture_type: TextureType) {
        self.texture_type = texture_type;
    }

    fn set_wrap(&mut self, wrap: Wrap) {
        self.wrap = wrap;
    }

    fn set_filter(&mut self, filter: Filter) {
        self.filter = filter;
    }

    fn set_swizzle(&mut self, swizzle: Swizzle) {
        self.swizzle = swizzle;
    }

    fn use_mip_map(&mut self, flag: bool) {
        self.use_mip_map = flag;
    }

    fn set_level(&mut self, level:u32) {
        self.level = level;
    }

    fn set_size(&mut self, size: Extend<u32>) {
        self.size = size;
    }
}

pub struct Texture {
    pub desc: TextureDesc,
    pub raw: u32,
    pub use_swizzle:bool,
    pub use_sampler: bool,
}


impl Texture {

   /* pub fn new(desc: &TextureDesc) -> Texture{
        let raw = es20::wrapper::gen_textures(1)[0];
        es20::wrapper::bind_texture(desc.texture_type as _, raw );

/*
        es20::wrapper::bind_texture(desc.texture_type as _, 0);
        Texture{
            desc,
            raw,
            use_swizzle: false,
            use_sampler: false,
        }
        */
    }
    */
/*
    pub fn new_with(desc: &TextureDesc, use_swizzel:bool) -> Texture {

    }

    pub fn set_minification_filter() -> Self {
        //todo:

    }

    pub fn set_magnification_filter() -> Self {
        //todo:
    }

    pub fn set_min_lod(&mut self, lod: u32) -> Self {
        //todo:
    }
    pub fn set_max_lod(&mut self, lod: u32) -> Self {}
    pub fn set_lod_bias(&mut self) -> Self {}
    pub fn set_wrapping(&mut self) -> Self {}
    pub fn set_max_anisotropy(&mut self) -> &Self {}
    pub fn set_image(&mut self) -> &Self {}
    pub fn set_sub_image(&mut self) -> &Self {}
    pub fn set_compressed_image(&mut self) -> Self {}
    pub  fn set_compressed_sub_image(&mut self) -> &Self {}

    pub fn generate_mipmap() -> Self {}

    pub fn use_sampler(&mut self, flag: bool) -> Self{

    }

    pub fn use_swizzle(&mut self, flag: bool) ->Self {

    }

    pub fn bind(&self) {
        es20::ffi::glBindTexture(self.desc.texture_type as _, self.raw);
    }

    pub fn unbind(&self) {
        es20::ffi::glBindTexture(self.desc.texture_type as _, self.raw);
    }
    */
}