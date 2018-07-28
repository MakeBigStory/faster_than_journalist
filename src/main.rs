extern crate opengles_rs as gles;

mod buffer;
mod color;
mod device;
mod format;
mod framebuffer;
mod mesh;
mod render_pass;
mod renderbuffer;
mod resource_pool;
mod sampler;
mod shader;
mod shader_program;
mod texture;
mod timer;
mod version;
use device::Device;
mod render_pipeline;
mod render_state;
mod rasterizer::*;

fn main() {
    println!("Hello, world!");
    let device = Device {};
}
