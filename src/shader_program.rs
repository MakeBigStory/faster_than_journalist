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

    fn link(mut self) -> bool {}

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

    fn attach_shader(mut self, shader: Shader) {}
    fn attach_shaders(mut self, shaders: &[Shader]) {}

    fn uniform_location(&self, name: String) -> i32 {}

    fn transform_feedback_varyings(&self, name: String) -> i32 {}
}
