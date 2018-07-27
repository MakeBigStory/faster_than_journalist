//enum ShaderType {
//    Vertex,
//    Fragment,
//    Compute,
//    TessellationControl,
//    TessellationEvaluation,
//}
//
//#[derive(Debug)]
//struct Shader {
//    label: String,
//    shader_type: ShaderType,
//    source: String,
//}
//
//impl Shader {
//    pub fn get_label() -> String {
//        label
//    }
//
//    pub fn set_label(mut self, label: String) {
//        self.label = label
//    }
//
//    pub fn new(label: String, shader_type: ShaderType) -> Self {
//        Shader {
//            label,
//            shader_type,
//            source: String::from(""),
//        }
//    }
//
//    pub fn compile(mut self) -> bool {
//        true
//    }
//
//    pub fn add_source(mut self, source: String) {
//        self.source = source
//    }
//
//    pub fn get_shader_log(gl: &gl::Gl, name: n::Shader) -> String {
//        let mut length = get_shader_iv(gl, name, gl::INFO_LOG_LENGTH);
//        if length > 0 {
//            let mut log = String::with_capacity(length as usize);
//            log.extend(repeat('\0').take(length as usize));
//            unsafe {
//                gl.GetShaderInfoLog(
//                    name,
//                    length,
//                    &mut length,
//                    (&log[..]).as_ptr() as *mut gl::types::GLchar,
//                );
//            }
//            log.truncate(length as usize);
//            log
//        } else {
//            String::new()
//        }
//
//    }
//
//    // todo: 安卓看情况实现
//    pub fn get_shader_binary(mut self) {}
//}
