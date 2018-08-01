use gles::es20::data_struct::*;
use gles::es20::wrapper::*;

use gles::es30::data_struct as es30d;

use renderbuffer::*;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::mem;
use std::ptr;
use texture::*;
use format::*;

#[derive(Copy, Clone, Debug)]
pub enum AttachmentType<'a> {
    TextureAttachment(&'a Texture),
    RenderBufferAttachment(&'a RenderBuffer),
}

#[derive(Clone, Debug)]
pub struct Attachment<'a> {
    pub label: String,
    pub usage: AttachmentUsage,
    pub attachment_type: AttachmentType<'a>,
}

impl<'a> Attachment<'a> {
    fn new(label: String, usage: AttachmentUsage, attachment_type: AttachmentType) -> Attachment {
        Attachment {
            label,
            usage,
            attachment_type,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub enum FrameBufferUsage {
    //todo:
    Read,
    Write,
    ReadWrite = GL_FRAMEBUFFER as isize,
}

#[derive(Clone, Debug)]
pub(crate) struct FrameBuffer<'a> {
    pub label: String,
    pub status: FrameBufferStatus,
    pub id: u32,
    num: u32,
    usage: FrameBufferUsage,
    attachments: Vec<(Attachment<'a>, u32)>,
}

impl<'a> FrameBuffer<'a> {
    #[inline(always)]
    pub fn new(label: String) -> Self {
        let id = gen_framebuffers(1)[0];
        FrameBuffer {
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
        bind_framebuffer(GL_FRAMEBUFFER, 0);
    }

    #[inline]
    pub fn bind(&self) {
        bind_framebuffer(GL_FRAMEBUFFER, self.id);
    }

    /// todo: only 1 supported with es20
    fn max_color_attachments(&self) -> i32 {
        1
    }

    pub fn check_status(&self) -> FrameBufferStatus {
        //todo : 原来的有30或者31的，对不上号，改了。
        match check_framebuffer_status(self.usage as _) {
            GL_FRAMEBUFFER_COMPLETE => FrameBufferStatus::Complete,
            GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT => FrameBufferStatus::IncompleteAttachment,
            GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                FrameBufferStatus::IncompleteMissingAttachment
            }
            GL_FRAMEBUFFER_UNSUPPORTED => FrameBufferStatus::Unsupported,
            _ => panic!("Frame buffer: not support status"),
        }
    }

    pub fn attach(&mut self, attachments: &[Attachment<'a>]) {
        self.bind();
        for attach in attachments {
            let attach_usage = self.allocate_attachment_usage(attach);
            self.attachments.push((attach.clone(), attach_usage));

//            match attach.attachment_type {
//                AttachmentType::TextureAttachment(ref mut texture) => {
//                    texture.bind();
//                    framebuffer_texture_2d(
//                        self.usage as _,
//                        attach_usage,
//                        texture.texture_type as _,
//                        texture.texture_id,
//                        texture.level as _,
//                    );
//                    texture.unbind();
//                }
//                AttachmentType::RenderBufferAttachment(renderBuffer) => {
//                    renderBuffer.bind();
//                    framebuffer_renderbuffer(
//                        self.usage as _,
//                        attach_usage,
//                        GL_RENDERBUFFER,
//                        renderBuffer.id,
//                    );
//                    renderBuffer.unbind();
//                }
//            }
        }
        self.unbind();
    }

    pub fn detach(&mut self, label: String) {
        let mut index:i32 = -1;
        {
            for (i, attach) in self.attachments.iter().enumerate() {
                if attach.0.label == label {
                    let attach_usage = attach.1;
//                    match attach.0.attachment_type {
//                        AttachmentType::TextureAttachment(ref mut texture) => {
//                            texture.bind();
//                            framebuffer_texture_2d(
//                                self.usage as _,
//                                attach_usage,
//                                texture.texture_type as _,
//                                0,
//                                0,
//                            );
//                            texture.unbind();
//                        }
//                        AttachmentType::RenderBufferAttachment(renderBuffer) => {
//                            renderBuffer.bind();
//                            framebuffer_renderbuffer(self.usage as _,
//                                                     attach_usage,
//                                                     GL_RENDERBUFFER,
//                                                     0);
//                            renderBuffer.unbind();
//                        }
//                    }

                    index = i as i32;
                    //self.attachments.remove(i);
                    break;
                } else {
                    continue;
                }
            }
        }
        if index != -1 {
            self.attachments.remove(index as usize);
        }
    }

    fn allocate_attachment_usage(&mut self, attach: &Attachment) -> u32 {
        let attach_usage = match attach.usage {
            AttachmentUsage::ColorAttach => {
                let mut gl_color_attach: u32 = 0;
                if self.num == 0 {
                    gl_color_attach = GL_COLOR_ATTACHMENT0;
                } else {
                    gl_color_attach = es30d::GL_COLOR_ATTACHMENT1 + self.num - 1;
                }
                self.num = self.num + 1;
                (gl_color_attach)
            }
            AttachmentUsage::DepthAttach => GL_DEPTH_ATTACHMENT,
            AttachmentUsage::StencilAttach => panic!("frame buffer: do not support"),
            AttachmentUsage::DepthStencil => panic!("frame buffer: do not support"),
        };
        attach_usage
    }

    //todo: attachment 是不是只应该由framebuffer管理??s
    pub fn detach_batch(&mut self, attachments: &[Attachment]) {}

    //Todo: invalidate ?
    pub fn invalidate(&mut self) {}

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

impl<'a> Drop for FrameBuffer<'a> {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            delete_framebuffers(&[self.id]);
        }
    }
}
