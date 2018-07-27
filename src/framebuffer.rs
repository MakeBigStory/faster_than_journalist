

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


#[derive(Clone, Debug)]
enum AttachmentType{
    ColorAttach,
    DepthAttach,
    StencilAttach,
    DepthStencil,
}


enum FrameBufferStatus {
    /** The framebuffer is complete */
    Complete = GL_FRAMEBUFFER_COMPLETE,

    /** Any of the attachment points are incomplete */
    IncompleteAttachment = GL_FRAMEBUFFER_INCOMPLETE_ATTACHMENT,

    /** The framebuffer does not have at least one image attached to it */
    IncompleteMissingAttachment = GL_FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT,

    /**
     * Combination of internal formats of the attached images violates
     * an implementation-dependent set of restrictions.
     */
    Unsupported = GL_FRAMEBUFFER_UNSUPPORTED,

    /// Sample count or locations are not the same for all attached images.
    IncompleteMultisample = GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
    // todo: ES 2.0 iOS
    //IncompleteMultisample = GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE,
}

struct FrameBuffer {
    label: String,
    status: Status,
    id: GLuint,
}

impl FrameBuffer {
    fn get_label() -> String {
        label
    }

    fn set_label(mut self, label: String) {
        self.label = label
    }

        #[inline(always)]
    pub fn new(gl_texture: &GLTexture, buffers: &[Attachment], level: isize) -> Self {
        let framebuffer = GLFramebuffer {
            id: {
                let mut id = 0;
                unsafe {
                    gl::GenFramebuffers(1, &mut id);
                }
                id
            },
        };

        framebuffer.set(gl_texture, buffers, level);
        framebuffer
    }

    #[inline(always)]
    pub fn id(&self) -> GLuint {
        self.id
    }

        #[inline]
    pub fn bind(&self) -> &Self {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
        self
    }
    #[inline]
    pub fn unbind(&self) -> &Self {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
        self
    }

    /// Max supported color attachment count
    fn max_color_attachments() -> i32 {}

    // copy pixels from attachments from the framebuffer
    pub fn copy_image() {}
    pub fn copy_sub_image() {}
    pub fn read() {}

    pub fn check_status(&self) -> Status {
                match gl::CheckFramebufferStatus(gl::FRAMEBUFFER) {
                gl::FRAMEBUFFER_UNDEFINED => panic!("Check framebuffer status failed undefined"),
                gl::FRAMEBUFFER_INCOMPLETE_ATTACHMENT => {
                    panic!("Check framebuffer status failed incomplete attachment")
                }
                gl::FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT => {
                    panic!("Check framebuffer status failed incomplete missing attachment")
                }
                gl::FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER => {
                    panic!("Check framebuffer status failed incomplete draw buffer")
                }
                gl::FRAMEBUFFER_INCOMPLETE_READ_BUFFER => {
                    panic!("Check framebuffer status failed incomplete read buffer")
                }
                gl::FRAMEBUFFER_UNSUPPORTED => {
                    panic!("Check framebuffer status failed unsupported")
                }
                gl::FRAMEBUFFER_INCOMPLETE_MULTISAMPLE => {
                    panic!("Check framebuffer status failed incomplete multisample")
                }
                gl::FRAMEBUFFER_INCOMPLETE_LAYER_TARGETS => {
                    panic!("Check framebuffer status failed incomplete layer targets")
                }
                // gl::FRAMEBUFFER_COMPLETE => (),
                _ => (),
            }
    }

    pub fn clear_color(mut self) {}

    pub fn invalidate(mut self) {}

    pub fn attach_renderbuffer(mut self) {}

    pub fn attach_texture(mut self) {
    gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);

                            gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl_enums[i],
                    gl::TEXTURE_2D,
                    gl_texture_id,
                    level as GLint,
                );

    }

    pub fn attach_textures(mut self) {
           let gl_texture_id = gl_texture.id();

        let mut gl_enums = Vec::with_capacity(buffers.len());
        for i in 0..buffers.len() {
            gl_enums.push(buffers[i].into());
        }

        for i in 0..gl_enums.len() {
               self.attach_texture();

               unsafe {
            gl::DrawBuffers(buffers.len() as GLint, gl_enums.as_ptr());
        }
            }
    }

    pub fn attach_render_buffer() {}

    fn attach_cube_map_texture(mut self) {}

    fn attach_texture_layer(mut self) {}

    fn detach(mut self) {}

    fn set_viewport(mut self) {}

    fn clear(mut self) {}
    fn clear_stencil(mut self) {}
    fn clear_depth(mut self) {}
    fn clear_depth_stencil(mut self) {}
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