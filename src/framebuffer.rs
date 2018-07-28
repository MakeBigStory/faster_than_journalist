use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use self::es20d::*;
use self::es30d::*;

use gles::es20::{data_struct, wrapper};

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
//    pub label: String,
    pub usage: AttachmentUsage,
    pub attachment_type: AttachmentType<'a>
}

impl Attachment{
    fn new(label:String, usage: AttachmentUsage, attachment_type: AttachmentType) -> Attachment {
        Attachment{
//            label: lable,
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
pub(crate) struct FrameBuffer<'a> {
    pub label: String,
    pub status: FrameBufferStatus,
    pub id: u32,
    num: u32,
    usage: FrameBufferUsage,
    attachments: Vec<(Attachment<'a>, u32)>
}

impl FrameBuffer {
    #[inline(always)]
    pub fn new(label:String) -> Self {
        let id = es20d::wrapper::gen_framebuffers(1)[0];
        FrameBuffer{
            label,
            status: FrameBufferStatus::IncompleteAttachment,
            id,
            num: 0,
            usage: FrameBufferUsage::ReadWrite,
            attachments: Vec::new(),
        }
    }

    //todo : new with usage over es20 version?

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

    #[inline]
    pub fn unbind(&self) {
        es20d::wrapper::bind_framebuffer(es20d::GL_FRAMEBUFFER, 0);
    }

    #[inline]
    pub fn bind(&self) { es20d::wrapper::bind_framebuffer(es20d::GL_FRAMEBUFFER, self.id);
    }

    /// todo: only 1 supported with es20
    fn max_color_attachments(&self) -> i32 {
        1
    }

    pub fn check_status(&self) -> FrameBufferStatus {
        //todo : 原来的有30或者31的，对不上号，改了。
        match es20d::wrapper::check_framebuffer_status(self.usage as _) {
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

    pub fn attach(&mut self, attachments: &[Attachment]){
        self.bind();
        for attach in attachments {
            let attach_usage = self.allocate_attachment_usage(attach);
            self.attachments.push((attach.clone(), attach_usage));

            match attach.attachment_type {
                AttachmentType::TextureAttachment(texture) => {
                    texture.bind();
                    es20d::wrapper::framebuffer_texture_2d(self.usage, attach_usage,
                    texture.desc.texture_type as _,texture.id, texture.desc.level);
                    texture.unbind();
                },
                AttachmentType::RenderBufferAttachment(renderBuffer)=>{
                    renderBuffer.bind();
                    es20d::wrapper::framebuffer_renderbuffer(self.usage, attach_usage,
                    es20d::GL_RENDERBUFFER, renderBuffer.id);
                    renderBuffer.unbind();

                }
            }
        }
        self.unbind();
    }

    pub fn detach(&mut self, label: String) {
        for (i, attach) in self.attachments.iter().enumerate() {
            if attachment.lable == lable {
                let attach_usage = attach.1;
                match attach.0.attachment_type {
                    AttachmentType::TextureAttachment(texture) => {
                        texture.bind();
                        es20d::wrapper::framebuffer_texture_2d(self.usage, attach_usage,
                                                              texture.desc.texture_type as _,0, 0);
                        texture.unbind();
                    },
                    AttachmentType::RenderBufferAttachment(renderBuffer)=>{
                        renderBuffer.bind();
                        es20d::wrapper::framebuffer_renderbuffer(self.usage, attach_usage,
                                                                es20d::GL_RENDERBUFFER, 0);
                        renderBuffer.unbind();

                    }
                }

                self.attachments.remove(i);
                break;
            }
            else {
                continue;
            }
        }
    }

    fn allocate_attachment_usage(&self, attach: &Attachment) -> u32 {
        let attach_usage = match attach.usage {
            AttachmentUsage::ColorAttach => {
                let mut gl_color_attach:u32 = 0;
                if self.num == 0 {
                    gl_color_attach = es20d::GL_COLOR_ATTACHMENT0;
                }
                    else {
                        gl_color_attach = es30d::GL_COLOR_ATTACHMENT1 + self.num - 1;
                    }
                self.num = self.num + 1;
                (gl_color_attach)
            },
            AttachmentUsage::DepthAttach => {
                es20d::GL_DEPTH_ATTACHMENT
            },
            AttachmentUsage::StencilAttach=> {
                panic!("frame buffer: do not support")
            },
            AttachmentUsage::DepthStencil => {
                panic!("frame buffer: do not support")
            }
        };
        attach_usage
    }

    //todo: attachment 是不是只应该由framebuffer管理??s
    pub fn detach_batch(&mut self, attachments:&[Attachment]) {

    }

    //Todo: invalidate ?
    pub fn invalidate(&mut self) {

    }

    //todo: useless? when use state?
    fn attach_cube_map_texture(&mut self) {
        unimplemented!()
    }
    fn attach_texture_layer(&mut self) {
        unimplemented!()
    }
    fn set_viewport(&mut self) {
        unimplemented!()
    }
    fn clear(&mut self) {
        unimplemented!()
    }
    fn clear_stencil(&mut self) {
        unimplemented!()
    }
    fn clear_depth(&mut self) {
        unimplemented!()
    }
    fn clear_depth_stencil(&mut self) {
        unimplemented!()
    }
    pub fn copy_image(&self) {
        unimplemented!()
    }
    pub fn copy_sub_image(&self) {
        unimplemented!()
    }
    pub fn read(&self) {
        unimplemented!()
    }
}

impl Drop for FrameBuffer {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            delete_framebuffers([self.id]);
        }
    }
}