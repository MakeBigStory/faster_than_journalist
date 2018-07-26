extern crate opengles_rs as gles;

mod buffer;
mod device;
mod render_pass;
mod shader_program;
use device::Device;

fn main() {
    println!("Hello, world!");
    let device = Device {};
}
