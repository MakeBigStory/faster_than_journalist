
use super::shader_program::ShaderProgram;
use super::framebuffer::FrameBuffer;
use super::device::Device;
use super::render_state::RenderState;

#[derive(Debug)]
struct RenderPipeline<'a> {
    program: &'a mut ShaderProgram,
    framebuffer: &'a mut FrameBuffer,
    device: &'a mut Device,
    render_state: &'a mut RenderState
}

impl<'a> RenderPipeline<'a> {
    fn execute(&mut self) -> Result<u32, &str> {
        self.device.prepare();

        self.device.flush();

        Ok(0)
    }

    fn attach_device(&mut self, device: &mut Device) -> &mut Self {
        self.device = device;
        self
    }

    fn set_program(&mut self, program: &mut ShaderProgram) -> &mut Self {
        self.program = program;
        self
    }

    fn set_framebuffer(&mut self, framebuffer: &mut FrameBuffer) -> &mut Self {
        self.framebuffer = framebuffer;
        self
    }

    fn set_render_state(&mut self, render_state: &mut RenderState) -> &mut Self {
        self.render_state = render_state;
        self
    }
}