
/*
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

    }

    /// use current program
    fn active(mut self) -> bool {
        //        gl.UseProgram(name);
    }

    pub fn get_program_log() -> String {

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

    fn get_uniform_location(&self, name: String) -> i32 {}
    fn get_uniform_block_index(&self, name: String) -> i32 {}

    fn transform_feedback_varyings(&self, name: String) -> i32 {}
    fn set_transform_feedback_outputs(&self, name: String) -> i32 {}
    fn set_transform_feedback_mode(&self, mode: TransformFeedbackBufferMode) -> i32 {}
}
*/