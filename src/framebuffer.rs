
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
use texture::*;
use renderbuffer::*;

#[derive(Copy, Clone, Debug)]
pub enum AttachmentUsage{
    ColorAttach,
    DepthAttach,
    StencilAttach,
    DepthStencil,
}

#[derive(Copy, Debug)]
pub enum AttachmentType<'a> {
    TextureAttachment(&'a Texture),
    RenderBufferAttachment(&'a RenderBuffer),
}

#[derive(Copy, Clone, Debug)]
pub enum FrameBufferStatus {
    /** The framebuffer is complete */
    Complete = es20d::GL_FRAMEBUFFER_COMPLETE,

    /** Any of the attachment points are incomplete */
    IncompleteAttachment = es20d::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT,

    /** The framebuffer does not have at least one image attached to it */
    IncompleteMissingAttachment = es20d::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,

    /**
     * Combination of internal formats of the attached images violates
     * an implementation-dependent set of restrictions.
     */
    Unsupported = es20d::GL_FRAMEBUFFER_UNSUPPORTED,

    /// Sample count or locations are not the same for all attached images.
    IncompleteMultisample = es30d::GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
    // todo: ES 2.0 iOS
    //IncompleteMultisample = GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE,
}

#[derive(Clone, Debug)]
pub struct Attachment<'a>{
    pub lable: String,
    pub usage: AttachmentUsage,
    pub attachment_type: AttachmentType<'a>
}

impl Attachment{
    fn new(label:String, usage: AttachmentUsage, attachment_type: AttachmentType) -> Attachment {
        Attachment{
            lable,
            usage,
            attachment_type,
        }
    }
}

#[derive(Copy, Debug)]
pub enum FrameBufferUsage {
    //todo:
    Read,
    Write,
    ReadWrite = es20d::GL_FRAMEBUFFER as isize,
}

#[derive(Clone, Debug)]
struct FrameBuffer {
    pub label: String,
    pub status: FrameBufferStatus,
    pub id: u32,
    num: u32,
    usage: FrameBufferUsage,
}

impl FrameBuffer {
    #[inline(always)]
    pub fn new(label:String) -> Self {
        let id = es20::wrapper::gen_framebuffers(1)[0];
        FrameBuffer{
            label,
            status: FrameBufferStatus::IncompleteAttachment,
            id,
            num: 0,
            usage: FrameBufferUsage::ReadWrite,
        }
    }

    //todo : new with usage over es20 version?

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline]
    pub fn unbind(&self) {
        es20::wrapper::bind_framebuffer(es20d::GL_FRAMEBUFFER, 0);
    }

    #[inline]
    pub fn bind(&self) {
        es20::wrapper::bind_framebuffer(es20d::GL_FRAMEBUFFER, 0);
    }

    /// todo: only 1 supported with es20
    fn max_color_attachments(&self) -> i32 {
        1
    }

    pub fn check_status(&self) -> FrameBufferStatus {
        //todo : 原来的有30或者31的，对不上号，改了。
        match es20::wrapper::check_framebuffer_status(self.usage as _) {
            es20d::GL_FRAMEBUFFER_COMPLETE => {
                FrameBufferStatus::Complete
            }
            es20d::GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT=> {
                FrameBufferStatus::IncompleteAttachment
            }
            es20d::GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT=>{
                FrameBufferStatus::IncompleteMissingAttachment
            }
            es20d::GL_FRAMEBUFFER_UNSUPPORTED=>{
                FrameBufferStatus::Unsupported
            }
            _ => {
                panic!("Frame buffer: not support status")
            }
        }
    }

    pub fn attach(&mut self, attachment: &[Attachment]){}
    pub fn detach(&mut self, attach:&Attachment) {}
    pub fn invalidate(&mut self) {}


    //todo: useless? when use state?
    fn attach_cube_map_texture(&mut self) {}
    fn attach_texture_layer(&mut self) {}
    fn set_viewport(&mut self) {}
    fn clear(&mut self) {}
    fn clear_stencil(&mut self) {}
    fn clear_depth(&mut self) {}
    fn clear_depth_stencil(&mut self) {}
    pub fn copy_image(&self) {}
    pub fn copy_sub_image(&self) {}
    pub fn read(&self) {}
}

impl Drop for FrameBuffer {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteFramebuffers(1, &self.id);
            }
        }
    }
}