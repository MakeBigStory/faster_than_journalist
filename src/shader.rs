enum ShaderType {
    Vertex,
    Fragment,
    Compute,
    TessellationControl,
    TessellationEvaluation,
}

struct Shader {
    label: String,
}

impl Shader {
    fn get_label() -> String {
        label
    }

    fn set_label(mut self, label: String) {
        self.label = label
    }

    fn compile(mut self) -> bool {
        true
    }

    fn add_source(source: String) {}

    fn get_shader_log(gl: &gl::Gl, name: n::Shader) -> String {
        let mut length = get_shader_iv(gl, name, gl::INFO_LOG_LENGTH);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(repeat('\0').take(length as usize));
            unsafe {
                gl.GetShaderInfoLog(name, length, &mut length,
                                    (&log[..]).as_ptr() as *mut gl::types::GLchar);
            }
            log.truncate(length as usize);
            log
        } else {
            String::new()
        }
    }
}
