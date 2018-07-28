mod shader_program;
//use shader_program::ShaderProgram;


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


#[no_mangle]
pub extern fn init_triangle_program(){
    let program = ShaderProgram::new();
    program.compile_shader(SIMPLE_VERTEX, SIMPLE_FRAGMENT);
}

//#[no_mangle]
//pub extern fn draw_triangle() {
//
//}