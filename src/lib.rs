extern crate opengles_rs as gles;

pub use self::shader::*;
pub use self::shader_program::*;

pub mod shader;
pub mod shader_program;

mod sampler;
mod format;
mod texture;
mod color;
mod attribute;
mod attribute_layout;
mod buffer;

static SIMPLE_VERTEX_DATA: [f32; 16] = [
    //   position     uv
    1f32,  1f32,   1f32, 1f32,
    -1f32,  1f32,   0f32, 1f32,
    1f32, -1f32,   1f32, 0f32,
    -1f32, -1f32,   0f32, 0f32
];


static SIMPLE_VERTEX: &'static str = "
    uniform vec2 size;

    attribute vec2 position;
    attribute vec2 uv;

    varying vec2 v_uv;

    void main() {
        gl_Position = vec4(position * size * 0.5, 0, 1.0);
        v_uv = uv;
    }
";

static SIMPLE_FRAGMENT: &'static str = "
    precision mediump float;

    uniform float alpha;

    varying vec2 v_uv;

    void main() {
        gl_FragColor = vec4(v_uv, 1.0, alpha);
    }
";

const LOG_TAG : &str = "winterfell";

#[macro_use] extern crate log;
extern crate android_logger;
use log::Level;
use android_logger::Filter;

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
pub mod android {
    extern crate jni;

    use self::jni::objects::{JClass, JString};
    use self::jni::sys::{jlong, jstring};
    use self::jni::JNIEnv;
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_vampire_opengl_Rust_init(
        env: JNIEnv,
        _: JClass,
    ) -> jlong {
        trace!("{} create program .... ", LOG_TAG);

        let mut program = ShaderProgram::create_shader_program(SIMPLE_VERTEX, SIMPLE_FRAGMENT);
        program.activate();

        let mut texture = texture::Texture::new("test texture", texture::Extend::new(1024, 768, 8));
        let bind_texture_res = texture.bind();

        let mut buffer = buffer::Buffer::new("test buffer");
        let bind_buffer_res = buffer.bind();

        if cfg!(target_os = "android") {
            android_logger::init_once(Filter::default()
                .with_min_level(Level::Trace));
            trace!("{} android logger init .... ", LOG_TAG);

            match bind_texture_res {
                Ok(_) => trace!("{} texture bind OK .... ", LOG_TAG),
                Err(error_desc) => trace!("{} texture bind fail {} .... ", LOG_TAG, error_desc)
            }

            match bind_buffer_res {
                Ok(_) => trace!("{} texture buffer OK .... ", LOG_TAG),
                Err(error_desc) => trace!("{} buffer bind fail {} .... ", LOG_TAG, error_desc)
            }

            trace!("{} program {:?} .... ", LOG_TAG, program);
            trace!("{} texture {:?} .... ", LOG_TAG, texture);
            trace!("{} texture {:?} .... ", LOG_TAG, buffer);
        }

        0 as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_example_vampire_opengl_Rust_draw(
        env: JNIEnv,
        _: JClass,
        handle: jlong,
    ) {
    }
}

#[no_mangle]
pub extern "C" fn init_triangle_program() {
    let mut program = ShaderProgram::create_shader_program(SIMPLE_VERTEX, SIMPLE_FRAGMENT);
    program.activate();
}

//#[no_mangle]
//pub extern fn draw_triangle() {
//
//}
