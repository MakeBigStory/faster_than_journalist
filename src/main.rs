extern crate opengles_rs as gles;

mod buffer;
mod device;
mod render_pass;
mod shader_program;
mod framebuffer;
mod camera;
mod light;
mod material;
mod mesh;
mod render_pass;
mod renderbuffer;
mod resource_pool;
mod sampler;
mod scene;
mod shader;
mod shader_program;
mod skeleton;
mod sprite;
mod text;
mod texture;
mod timer;
mod version;
use device::Device;


fn main() {
    println!("Hello, world!");
    let device = Device {};
}
