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

// todo: 宽高为非2的n次方纹理的处理

#[derive(Clone, Debug)]
pub struct Swizzle {
    r: SwizzleMode,
    g: SwizzleMode,
    b: SwizzleMode,
    a: SwizzleMode
}

impl Swizzle {
    fn new() -> Self {
        Swizzle {
            r: SwizzleMode::SwizzleR,
            g: SwizzleMode::SwizzleG,
            b: SwizzleMode::SwizzleB,
            a: SwizzleMode::SwizzleA,
        }
    }

    fn with(swizzle: &[SwizzleMode; 4]) -> Self {
        Swizzle {
            r: swizzle[0],
            g: swizzle[1],
            b: swizzle[2],
            a: swizzle[3]
        }
    }
}

#[derive(Clone, Debug)]
pub struct Extend<T> {
    pub width: T,
    pub height: T,
    pub depth: T
}

impl<T> Extend<T> {
    pub fn new(width: T, height: T, depth: T) -> Extend<T> {
        Extend {
            width,
            height,
            depth,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TextureState {
    pub wrap_mode: Option<Wrap>,
    pub filter_mode: Option<Filter>,
    pub swizzle_mode: Option<Swizzle>
}

trait Apply {
    fn apply(&mut self, target: TextureType) -> Result<(), String>;

    fn apply_diff(&mut self, other: &mut Self, target: TextureType) -> Result<(), String>;
}

impl Apply for Wrap {
    // TODO: error-check
    fn apply(&mut self, target: TextureType) -> Result<(), String> {
        es20::wrapper::tex_parameteri(target as _,
                                      es20d::GL_TEXTURE_WRAP_S,
                                      self.S as _);

        es20::wrapper::tex_parameteri(target as _,
                                      es20d::GL_TEXTURE_WRAP_T,
                                      self.T as _);

        Ok(())
    }

    // TODO: error-check
    fn apply_diff(&mut self, other: &mut Self, target: TextureType) -> Result<(), String> {
        if other.S != self.S {
            es20::wrapper::tex_parameteri(target as _,
                                          es20d::GL_TEXTURE_WRAP_S,
                                          self.S as _);
        }

        if other.T != self.T {
            es20::wrapper::tex_parameteri(target as _,
                                          es20d::GL_TEXTURE_WRAP_T,
                                          self.T as _);
        }

        Ok(())
    }
}

impl Apply for Filter {
    // TODO: error-check
    fn apply(&mut self, target: TextureType) -> Result<(), String> {
        es20::wrapper::tex_parameteri(target as _,
                                      es20d::GL_TEXTURE_MIN_FILTER,
                                      self.min as _);

        es20::wrapper::tex_parameteri(target as _,
                                      es20d::GL_TEXTURE_MAG_FILTER,
                                      self.mag as _);

        Ok(())
    }

    // TODO: error-check
    fn apply_diff(&mut self, other: &mut Self, target: TextureType) -> Result<(), String> {
        if self.min != other.min {
            es20::wrapper::tex_parameteri(target as _,
                                          es20d::GL_TEXTURE_MIN_FILTER,
                                          self.min as _);
        }

        if self.mag != other.mag {
            es20::wrapper::tex_parameteri(target as _,
                                          es20d::GL_TEXTURE_MAG_FILTER,
                                          self.mag as _);
        }

        Ok(())
    }
}

impl Apply for Swizzle {
    // TODO: es30暂时不支持
    fn apply(&mut self, target: TextureType) -> Result<(), String> {
        Ok(())
    }

    fn apply_diff(&mut self, other: &mut Self, target: TextureType) -> Result<(), String> {
        Ok(())
    }
}

impl TextureState {
    fn apply(&mut self, target: TextureType) -> Result<(), String> {
        match self.filter_mode {
            Some(ref mut filter_mode) => filter_mode.apply(target),
            // TODO: 暂时这样
            None => Ok(())
        }?;

        match self.wrap_mode {
            Some(ref mut wrap_mode) => wrap_mode.apply(target),
            // TODO: 暂时这样
            None => Ok(())
        }
    }

    fn apply_diff(&mut self, old_texture_state : &mut TextureState, target: TextureType) -> Result<(), String> {
        match self.filter_mode {
            Some(ref mut filter_mode) => {
                match old_texture_state.filter_mode {
                    Some(ref mut old_filter_mode) => filter_mode.apply_diff(old_filter_mode, target),
                    None => filter_mode.apply(target)
                }
            }

            // Nothing, no op
            None => {
                Ok(())
            }
        }?;

        match self.wrap_mode {
            Some(ref mut wrap_mode) => {
                match old_texture_state.wrap_mode {
                    Some(ref mut old_wrap_mode) => wrap_mode.apply_diff(old_wrap_mode, target),
                    None => wrap_mode.apply(target)
                }
            }
            // Nothing, no op
            None => {
                Ok(())
            }
        }
    }
}

impl Default for TextureState {
    fn default() -> Self {
        TextureState {
            wrap_mode: None,
            filter_mode: None,
            swizzle_mode: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture {
    pub label: String,

    pub texture_id: u32,
    pub use_swizzle: bool,
    pub use_sampler: bool,
    mipmap_ready: bool,
    staging_texture_state: TextureState,
    active_texture_state: TextureState,

    pub texture_type: TextureType,
    pub use_mipmap: bool,
    pub level: u32,
    pub size: Extend<u32>,
    pub board_size: u32,
    pub format: DataFormat,

    //Todo: 支持多重采样的纹理以及压缩纹理
    pub support_multiple_sampler: bool,
    pub support_compress: bool,

    current_out_pixel_type: DataKind,

    ready: bool
}

impl Texture {

    pub fn new(label: &str, size: Extend<u32>) -> Self {
        Texture {
            label: label.to_string(),

            texture_id: 0,
            texture_type: TextureType::Texture2d,

            use_swizzle: false,
            use_sampler: false,

            mipmap_ready: false,
            use_mipmap: false,

            staging_texture_state: TextureState::default(),
            active_texture_state: TextureState::default(),

            current_out_pixel_type: DataKind::UnsignedByte,

            level: 0,
            size,
            board_size: 0,
            format: DataFormat::RGBA,

            support_multiple_sampler: false,
            support_compress: false,

            ready: false
        }
    }

    fn set_data_format(&mut self, data_format: DataFormat) ->&mut Self {
        self.format = data_format;
        self
    }

    fn set_texture_type(&mut self, texture_type: TextureType) -> &mut Self {
        self.texture_type = texture_type;
        self
    }

    fn set_wrap(&mut self, wrap: Wrap) -> &mut Self {
        self.staging_texture_state.wrap_mode = Option::Some(wrap);
        self
    }

    fn set_filter(&mut self, filter: Filter) -> &mut Self {
        self.staging_texture_state.filter_mode = Option::Some(filter);
        self
    }

    fn set_swizzle(&mut self, swizzle: Swizzle) -> &mut Self {
        self.staging_texture_state.swizzle_mode = Option::Some(swizzle);
        self
    }

    fn use_mipmap(&mut self, use_mipmap: bool) -> &mut Self {
        self.use_mipmap = use_mipmap;
        self
    }

    fn set_level(&mut self, level: u32) -> &mut Self {
        self.level = level;
        self
    }

    fn set_size(&mut self, size: Extend<u32>) -> &mut Self {
        self.size = size;
        self
    }

    fn set_board_size(&mut self, board_size: u32) -> &mut Self {
        self.board_size = board_size;
        self
    }

    // TODO: 暂时隐蔽这两个细化接口
//    pub fn set_min_filter(&mut self, min_filter_mode: FilterMode) -> &mut self {
//        match self.staging_texture_state.filter_mode {
//            Some(filter_mode) => filter_mode.min = min_filter_mode,
//            None => self.staging_texture_state.filter_mode = Option::Some(filter)
//        }
//
//        self
//    }
//
//    pub fn set_mag_filter(&mut self, filter_mode: FilterMode) -> &mut self {
////        self.bind();
//        self.staging_texture_state.filter_mode.mag = filter_mode;
//
//        match self.staging_texture_state.filter_mode {
//            Some(filter_mode) => filter_mode.min = min_filter_mode,
//            None =>
//        }
//
//        self
//    }

    pub fn set_min_lod(&mut self, lod: u32) {
        // TODO; Sampler Object Property
    }
    pub fn set_max_lod(&mut self, lod: u32) {
        // TODO: Sampler Object Property
    }
    pub fn set_lod_bias(&mut self, lod_bias: f32) {
        //TODO:
    }

    pub fn set_max_anisotropy(&mut self) {
        //TODO: Sampler Object Property
    }
    pub fn set_data<T>(&mut self, pixel_format: DataFormat,
                        data: &[T], pixel_type: DataKind) {

        self.current_out_pixel_type = pixel_type;
        self.format = pixel_format;

        self.bind();

        es20::wrapper::tex_image_2d(self.texture_type as _,
                                    self.level as _,
                                    self.format as _,
                                    self.size.width as _,
                                    self.size.height as _,
                                    self.board_size as _,
                                    self.format as _,
                                    pixel_type as _,
                                    data,
        );

        self.mipmap_ready = false;

        if self.use_mipmap {
            self.generate_mipmap();
        }

        self.unbind();
    }
    pub fn update_data<T>(&mut self,
                         offset_x: u32, offset_y: u32,
                        width: u32, height: u32, data: &[T]) {
        self.bind();

        es20::wrapper::tex_sub_image_2d(self.texture_type as _,
                                    self.level as _,
                                    offset_x as _,
                                    offset_y as _,
                                    width as _,
                                    height as _,
                                    self.format as _,
                                    self.current_out_pixel_type as _,
                                    data,
        );

        self.mipmap_ready = false;

        if self.use_mipmap {
            self.generate_mipmap();
        }

        self.unbind();
    }

    pub fn set_compressed_image(&mut self) {
        //TODO:
    }

    pub fn set_compressed_sub_image(&mut self) {
        //TODO: 与非压缩版区别是image_size参数
//        self.bind();
//        es20::wrapper::compressed_tex_sub_image_2d(self.desc.texture_type as _,
//                                        self.desc.level as _,
//                                        offset_x as _,
//                                        offset_y as _,
//                                        width as _,
//                                        height as _,
//                                        self.current_out_pixel_format as _,
//                                        self.current_out_pixel_type as _,
//                                        data,
//        );
//
//        if self.use_mip_map {
//            es20::wrapper::generate_mipmap(self.desc.texture_type as _);
//        }
//        self.unbind();
    }

    // TODO: bind和unbind由调用者来保证，是否可以维护一个bound flag?
    pub fn generate_mipmap(&mut self) {
        if !self.mipmap_ready {
            es20::wrapper::generate_mipmap(self.texture_type as _);
            self.mipmap_ready = true
        }
    }

    pub fn active(&mut self, texture_slot : es20d::GLenum) -> Result<(), String> {
        es20::wrapper::active_texture(texture_slot);

        self.bind()
    }

    pub fn use_sampler(&mut self, use_sampler: bool) {
        self.use_sampler = use_sampler;
    }

    pub fn use_swizzle(&mut self, use_swizzle: bool) {
        self.use_swizzle = use_swizzle;
    }

    fn create_texture(&mut self) -> Result<(), String> {
        if self.ready {
            return Ok(())
        }

        let texture_id = es20::wrapper::gen_textures(1)[0];

        match texture_id {
            0 => Err("Generate invalid texture id 0 !!!".to_string()),
            texture_id => {
                    self.texture_id = texture_id;
                    Ok(())
                }
            }?;

        self.ready = true;

        Ok(())
    }

    #[inline]
    pub fn bind(&mut self) -> Result<(), String> {
        if !self.ready {
            self.create_texture()?;
        }

        es20::wrapper::bind_texture(self.texture_type as u32, self.texture_id);

        self.staging_texture_state.apply_diff(&mut self.active_texture_state, self.texture_type)?;
        // TODO: 性能，需要加一个flag
        self.active_texture_state = self.staging_texture_state.clone();

        // TODO: check result
        Ok(())
    }

    #[inline]
    pub fn unbind(&self) -> Result<(), String> {
        es20::wrapper::bind_texture(self.texture_type as u32, 0);

        // TODO: check result
        Ok(())
    }

//    #[inline(always)]
//    pub fn id(&self) -> GLuint {
//        self.id
//    }
//    #[inline(always)]
//    pub fn width(&self) -> usize {
//        self.desc.size.width as usize
//    }
//    #[inline(always)]
//    pub fn height(&self) -> usize {
//        self.desc.size.height as usize
//    }
//
//    #[inline(always)]
//    pub fn format(&self) -> DataFormat {
//        self.format
//    }
//    #[inline(always)]
//    pub fn filter(&self) -> FilterMode {
//        self.filter
//    }
//    #[inline(always)]
//    pub fn wrap(&self) -> Wrap {
//        self.wrap
//    }
//    #[inline(always)]
//    pub fn mipmap(&self) -> bool {
//        self.mipmap
//    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        if self.ready {
            es20::wrapper::delete_textures(&[self.texture_id]);

            self.texture_id = 0;
            self.ready = false;
        }
    }
}