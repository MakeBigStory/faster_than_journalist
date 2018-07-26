struct ShaderProgram {
    label: String,
    transform_feedback_varying_names: [String],
}

impl ShaderProgram {
    fn get_label() -> String {
        label
    }

    fn set_label(mut self, label: String) {
        self.label = label
    }


    fn attach_shader(mut self, shader: Shader) {
        unsafe { gl.AttachShader(name, shader) };
    }

    fn attach_shaders(mut self, shaders: &[Shader]) {
        for shader in shaders {
            attach_shader(shader);
        }
    }

    // todo: move to `new()`
    fn create(mut self) -> bool {
        gl.CreateProgram()
    }


    fn link(mut self) -> bool {
        unsafe { gl.LinkProgram(name) };
    }

    fn active(mut self) -> bool {
//        gl.UseProgram(name);
    }

    pub fn get_program_log(gl: &gl::Gl, name: n::Program) -> String {
        let mut length  = get_program_iv(gl, name, gl::INFO_LOG_LENGTH);
        if length > 0 {
            let mut log = String::with_capacity(length as usize);
            log.extend(repeat('\0').take(length as usize));
            unsafe {
                gl.GetProgramInfoLog(name, length, &mut length,
                                     (&log[..]).as_ptr() as *mut gl::types::GLchar);
            }
            log.truncate(length as usize);
            log
        } else {
            String::new()
        }
    }

    fn validate(mut self) -> bool {}

    fn dispatch_compute(mut self) -> bool {}

    /// Allow retrieving program binary
    /// PROGRAM_BINARY_RETRIEVABLE_HINT
    fn set_retrievable_binary(mut self, enabled: bool) {
        //    glProgramParameteri(_id, GL_PROGRAM_BINARY_RETRIEVABLE_HINT, enabled ? GL_TRUE : GL_FALSE);
    }

    /// Allow the program to be bound to individual pipeline stages
    /// PROGRAM_SEPARABLE
    fn set_separable(mut self, enabled: bool) {
        //    glProgramParameteri(_id, GL_PROGRAM_SEPARABLE, enabled ? GL_TRUE : GL_FALSE);
        // todo: conditional compile
        //    glProgramParameteriEXT(_id, GL_PROGRAM_SEPARABLE_EXT, enabled ? GL_TRUE : GL_FALSE);
    }

    fn uniform_location(&self, name: String) -> i32 {}

    fn transform_feedback_varyings(&self, name: String) -> i32 {}
}
