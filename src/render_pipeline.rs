
use super::shader_program::ShaderProgram;
use super::framebuffer::FrameBuffer;
use super::device::Device;
use super::render_state::RenderState;

#[derive(Debug)]
struct RenderPipeline<'a, 'b: 'a> {
    program: &'a mut ShaderProgram,
    framebuffer: &'a mut FrameBuffer<'b>,
    device: &'a mut Device,
    render_state: &'a mut RenderState
}

impl<'a, 'b: 'a> RenderPipeline<'a, 'b> {
    fn execute(&mut self) -> Result<u32, &str> {
        self.device.prepare();

        self.device.flush();

        Ok(0)
    }

    fn attach_device(&mut self, device: &'b mut Device) -> &mut Self {
        self.device = device;
        self
    }

    fn set_program(&mut self, program: &'b mut ShaderProgram) -> &mut Self {
        self.program = program;
        self
    }

    fn set_framebuffer(&mut self, framebuffer: &'b mut FrameBuffer<'b>) -> &mut Self {
        self.framebuffer = framebuffer;
        self
    }

    fn set_render_state(&mut self, render_state: &'b mut RenderState) -> &mut Self {
        self.render_state = render_state;
        self
    }
}