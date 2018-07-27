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
use format::*;

#[derive(Clone,Debug)]
pub struct RenderBuffer {
    pub label: String,
    pub format: TextureFormat,
    pub raw: u32,
    pub width: u32,
    pub height: u32,
}

impl RenderBuffer {
    pub fn new(label:String) -> RenderBuffer{
        let raw = es20::wrapper::gen_renderbuffers(1)[0];
        RenderBuffer{
            label,
            format: TextureFormat::DepthComponent,
            raw,
            width: 0,
            height: 0,
        }
    }

    pub fn set_storage(&mut self, format: TextureFormat, width: u32, height: u32) {
        self.bind();
        es20::wrapper::renderbuffer_storage(es20d::GL_RENDERBUFFER,format as _ , width, height);
        self.unbind();
    }

    pub fn bind(&self) {
        es20::wrapper::bind_renderbuffer(es20d::GL_RENDERBUFFER, self.raw);
    }

    pub fn unbind(&self) {
        es20::wrapper::bind_renderbuffer(es20d::GL_RENDERBUFFER, 0);
    }
}

impl Drop for RenderBuffer {
    fn drop(&mut self) {
        es20::wrapper::delete_renderbuffers(&[self.raw]);
    }
}


/*

impl RenderBuffer {
    /// `MAX_RENDERBUFFER_SIZE`
    fn max_size() -> i32 {}

    /// `MAX_SAMPLES`
    fn max_samples() -> i32 {}

    fn wrap(mut self) {}

    fn set_storage(mut self) {}
    fn set_storage_multi_sample(mut self) {}
    fn get_label(mut self) {}
    fn set_label(mut self) {}
}
*/