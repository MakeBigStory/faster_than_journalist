use gles::es20::data_struct::*;
use gles::es20::wrapper::*;

use gles::es30::data_struct as es30d;

use format::*;
use renderbuffer::*;
use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::mem;
use std::ptr;
use texture::*;

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

pub struct Range2D<T> {
    x: T,
    y: T,
    width: T,
    height: T,
}

pub struct Vector2<T> {
    x: T,
    y: T,
}

/// Mask for framebuffer clearing
enum FramebufferClearMask {
    /// Color buffer
    Color = GL_COLOR_BUFFER_BIT,
    /// Depth buffer
    Depth = GL_DEPTH_BUFFER_BIT,
    /// Stencil buffer
    Stencil = GL_STENCIL_BUFFER_BIT,
}

/// Invalidation attachment
enum InvalidationAttachment {
    /// Invalidate depth buffer
    ///
    /// GL_DEPTH_ATTACHMENT
    Depth,

    /// Invalidate stencil buffer
    ///
    /// GL_DEPTH_ATTACHMENT
    Stencil,
}

#[derive(Clone, Debug)]
pub(crate) struct FrameBuffer<'a> {
    pub label: String,
    pub status: FrameBufferStatus,
    pub id: u32,
    pub viewport: Range2D<i32>,
    num: u32,
    usage: FrameBufferUsage,
    attachments: Vec<(Attachment<'a>, u32)>,
}

impl<'a> FrameBuffer<'a> {
    #[inline(always)]
    pub fn new(label: String) -> Self {
        let id = gl_gen_framebuffers(1)[0];
        FrameBuffer {
            label,
            status: FrameBufferStatus::IncompleteAttachment,
            id,
            viewport: Range2D {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
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
    pub fn unbind(&self) -> &Self {
        gl_bind_framebuffer(GL_FRAMEBUFFER, 0);
        self
    }

    #[inline]
    pub fn bind(&self)  -> &Self {
        gl_bind_framebuffer(GL_FRAMEBUFFER, self.id);
        self
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

            match attach.attachment_type {
                AttachmentType::TextureAttachment(texture) => {
                    texture.bind();
                    framebuffer_texture_2d(
                        self.usage as _,
                        attach_usage,
                        texture.desc.texture_type as _,
                        texture.id,
                        texture.desc.level as _,
                    );
                    texture.unbind();
                }
                AttachmentType::RenderBufferAttachment(renderBuffer) => {
                    renderBuffer.bind();
                    framebuffer_renderbuffer(
                        self.usage as _,
                        attach_usage,
                        GL_RENDERBUFFER,
                        renderBuffer.id,
                    );
                    renderBuffer.unbind();
                }
            }
        }
        self.unbind();
    }

    pub fn detach(&mut self, label: String) {
        let mut index: i32 = -1;
        {
            for (i, attach) in self.attachments.iter().enumerate() {
                if attach.0.label == label {
                    let attach_usage = attach.1;
                    match attach.0.attachment_type {
                        AttachmentType::TextureAttachment(texture) => {
                            texture.bind();
                            framebuffer_texture_2d(
                                self.usage as _,
                                attach_usage,
                                texture.desc.texture_type as _,
                                0,
                                0,
                            );
                            texture.unbind();
                        }
                        AttachmentType::RenderBufferAttachment(renderBuffer) => {
                            renderBuffer.bind();
                            framebuffer_renderbuffer(
                                self.usage as _,
                                attach_usage,
                                GL_RENDERBUFFER,
                                0,
                            );
                            renderBuffer.unbind();
                        }
                    }

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

    /// Invalidate framebuffer
    ///
    /// # Arguments
    ///
    /// * `attachments` - Attachments to invalidate
    pub fn invalidate(&mut self, attachments: &[InvalidationAttachment]) {}

    pub fn attach_cube_map_texture(&self) -> &Self {
        self
    }

    pub fn attach_texture_layer(&self) -> &Self {
        self
    }

    pub fn set_viewport(&self) -> &Self {
        self
    }

    /// Clear specified buffers in framebuffer
    ///
    /// # Arguments
    ///
    /// * `mask` - Which buffers to clear
    pub fn clear(&mut self, mask: FramebufferClearMask) -> () {
        gl_clear(GLbitfield(mask));
    }

    /// Clear specified buffers in framebuffer
    ///
    /// # Arguments
    ///
    /// * `depth` - Value to clear with
    pub fn clear_depth(&mut self, depth: f32) {
        gl_clear_depth(GL_DEPTH_BITS, depth);
    }

    /// Clear stencil buffer to specified value
    ///
    /// # Arguments
    ///
    /// * `stencil` - Value to clear with
    pub fn clear_stencil(&mut self, stencil: i32) {
        //        glClearBufferiv(FramebufferTarget::Draw, 0, stencil)
    }

    /// Clear specified buffers in framebuffer
    ///
    /// # Arguments
    ///
    /// * `depth` - Value to clear with
    /// * `stencil` - Value to clear with
    pub fn clear_depth_stencil(&mut self, depth: f32, stencil: i32) {
        unimplemented!()
    }

    pub fn copy_image(&self) {
        unimplemented!()
    }
    pub fn copy_sub_image(&self) {
        unimplemented!()
    }

    /// Read block of pixels from framebuffer to image
    ///
    /// # Arguments
    ///
    /// * `rectangle` - Framebuffer rectangle to read
    /// * `image` - Image where to put the data
    pub fn read(&self, rectangle: Range2D<i32>, image: Image2D) {
        //        read_pixels(rectangle.x, rectangle.y, rectangle.width, rectangle.height, image.type, image.buffer;)
    }
}

impl FrameBuffer {
    /// Max supported viewport size
    pub fn max_viewport_size() -> Vector2<i32> {}

    /// Max supported draw buffer count
    pub fn max_draw_buffers() -> i32 {}

    // todo: deal with es 3.x
    /// Max supported color attachment count
    pub fn max_color_attachments(&self) -> i32 {
        1 // ES 2.0 standard supports one color attachment only
    }
}

impl<'a> Drop for FrameBuffer<'a> {
    #[inline]
    fn drop(&mut self) {
        if self.id != 0 {
            gl_delete_framebuffers(&[self.id]);
        }
    }
}
