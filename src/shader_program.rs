use shader::Shader;

#[derive(Debug)]
struct ShaderProgram {
    label: String,
    transform_feedback_varying_names: [String],
    program_id: i32,
    enable_program_pipeline: bool,
    shader_count: u8,
    enable_merge_vertex_buffer_array: bool,
}

impl ShaderProgram {
    fn get_label(&mut self) -> &String {
        &(self.label)
    }

    fn set_label(&mut self, label: &String) -> &mut Self {
        self.label = label.clone();
    }

    // todo: refine
    pub fn create_render_program(){}
    pub fn create_compute_program(){}

    // todo: remain to discussion
//    pub fn new(&mut self, shader_count: u8) -> Self {
//        Program {
//            program_id: 0,
//            enable_program_pipeline: false,
//            shader_count: 0,
//            enable_merge_vertex_buffer_array: false,
//            label: (""),
//            transform_feedback_varying_names: (),
//        }
//    }

    pub fn initialize() -> bool {
        return true;
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

    /// reset to default program, namely program 0
    fn inactive(mut self) -> bool {
        //        gl.UseProgram(name);
    }

    #[inline]
    pub fn get_program_log() -> String {
//        let mut status = 0;
//
//        unsafe { gl::GetProgramiv(program, gl::LINK_STATUS, &mut status) };
//
//        if status != (gl::TRUE as GLint) {
//            let mut len: GLint = 0;
//            unsafe {
//                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
//            }
//            let mut buf = Vec::with_capacity(len as usize);
//            unsafe {
//                buf.set_len(len as usize);
//                gl::GetProgramInfoLog(
//                    program,
//                    len,
//                    ptr::null_mut(),
//                    buf.as_mut_ptr() as *mut GLchar,
//                );
//            }
//            panic!(
//                "{}",
//                str::from_utf8(&buf)
//                    .ok()
//                    .expect("ProgramInfoLog not valid utf8")
//            );
//        }
//        program
    }

    // todo: 补充设置Attribute, uniform的数据接口
    pub fn set_argument_value(&self) -> &Self {
        self
    }

    /// only use in debug mode
    fn validate(mut self) -> bool {}

    fn dispatch_compute(mut self) -> bool {}

    pub fn set_enable_program_pipeline(enable: bool) {
        enable_program_pipeline = enable;
    }

    pub fn set_enable_merge_vertex_buffer_array() {}
    pub fn set_use_vertex_buffer_array() {}


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

    fn get_attribute_location(&self, name: String) -> i32 {}
    fn get_uniform_location(&self, name: String) -> i32 {}
    fn get_uniform_block_index(&self, name: String) -> i32 {}

    fn transform_feedback_varyings(&self, name: String) -> i32 {}
    fn set_transform_feedback_outputs(&self, name: String) -> i32 {}
    fn set_transform_feedback_mode(&self, mode: TransformFeedbackBufferMode) -> i32 {}
}


impl Drop for Program {
    #[inline]
    fn drop(&mut self) {
//        if self.id != 0 {
//            unsafe {
//                gl::DeleteProgram(self.id);
//            }
//        }
    }
}