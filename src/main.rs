
extern crate opengles_rs as gles;

mod buffer;
mod device;
mod pipeline_state;
mod render_pass;
use device::Device;

fn main() {
    println!("Hello, world!");
    let device = Device {};
}
