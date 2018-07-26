struct ColorAttachment {}

struct DrawAttachment {}

struct BufferAttachment {}

enum Status {
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

    IncompleteMultisample = GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE,
    // todo: ES 2.0 iOS
    //IncompleteMultisample = GL_FRAMEBUFFER_INCOMPLETE_MULTISAMPLE_APPLE,
}

struct FrameBuffer {
    label: String,
    status: Status,
}

impl FrameBuffer {
    /// Max supported color attachment count
    fn max_color_attachments() -> i32 {}

    fn get_label() -> String {
        label
    }

    fn set_label(mut self, label: String) {
        label
    }

    fn check_status(&self) -> Status {}

    fn clear_color(mut self) {}

    fn invalidate(mut self) {}

    fn attach_renderbuffer(mut self) {}

    fn attach_texture(mut self) {}

    fn attach_cube_map_texture(mut self) {}

    fn attach_texture_layer(mut self) {}

    fn detach(mut self) {}

    fn set_viewport(mut self) {}

    fn clear(mut self) {}

    fn clear_stencil(mut self) {}

    fn clear_depth(mut self) {}
    fn clear_depth_stencil(mut self) {}
}
