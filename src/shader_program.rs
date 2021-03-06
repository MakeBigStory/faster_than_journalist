use std::collections::HashMap;

use gles::es20::data_struct::GL_TRUE;
use gles::es20::data_struct::GL_LINK_STATUS;
use gles::es20::data_struct::GL_INFO_LOG_LENGTH;
use gles::es20::data_struct::GLuint;
use gles::es20::wrapper;
use shader::Shader;
use shader::ShaderType;
use attribute::Attribute;

#[derive(Debug)]
pub struct ShaderProgram {
    pub label: String,
    //    transform_feedback_varying_names: [String],
    program_id: GLuint,
    enable_program_pipeline: bool,
    enable_merge_vertex_buffer_array: bool,
    ready: bool,
    can_reuse: bool,
    shader_collection: HashMap<String, Shader>
}

// static method
impl ShaderProgram {

    #[inline]
    pub fn create_shader_program(vertex: &str, fragment: &str) -> Self {
        let mut program = ShaderProgram {
            program_id: 0,
            enable_program_pipeline: false,
            enable_merge_vertex_buffer_array: false,
            label: String::from(""),
            //            transform_feedback_varying_names: ,
            ready: false,
            can_reuse: false,
            shader_collection: HashMap::new()
        };

        let mut vertex_shader = Shader::new("vertex",
                                        ShaderType::Vertex,
                                            vertex);
        let mut fragment_shader = Shader::new("fragment",
                                            ShaderType::Fragment,
                                              fragment);

        program.add_shader(vertex_shader);
        program.add_shader(fragment_shader);

        program
    }

    #[inline]
    pub fn create_compute_program() {}

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

    fn create_program() -> Result<GLuint, String> {
        match wrapper::create_program() {
            0 => Err("Can not create program".to_string()),
            program_id => Ok(program_id)
        }
    }

    fn validate_program(program_id : GLuint) -> Result<(), String> {
        wrapper::validate_program(program_id);

        // TODO: glGetError check
        Ok(())
    }

    fn use_program(program_id : GLuint) -> Result<(), String> {
        wrapper::use_program(program_id);

        // TODO: glGetError check
        Ok(())
    }

    fn delete_program(program_id : GLuint) -> Result<(), String> {
        wrapper::delete_program(program_id);

        // TODO: glGetError check
        Ok(())
    }

    fn link_program(program_id : GLuint) -> Result<(), String> {
        wrapper::link_program(program_id);

        // TODO: glGetError check
        Ok(())
    }

    pub fn is_valid_program_id(program_id : GLuint) -> bool {
        match program_id {
            0 => false,
            _ => wrapper::is_program(program_id)
        }
    }

    //    fn get_attribute_location(&self, name: String) -> i32 {}
    //    fn get_uniform_location(&self, name: String) -> i32 {}
    //    fn get_uniform_block_index(&self, name: String) -> i32 {}

    //    fn transform_feedback_varyings(&self, name: String) -> i32 {}
    //    fn set_transform_feedback_outputs(&self, name: String) -> i32 {}
    //    fn set_transform_feedback_mode(&self, mode: TransformFeedbackBufferMode) -> i32 {}
}

// instance method
impl ShaderProgram {

    fn is_ready(&self) -> bool {
        self.ready
    }

    pub fn compile(&mut self) -> Result<(), String> {
        if !self.ready {
            self.aux_create_program()?;

            for (shader_name, shader) in &mut self.shader_collection {
                shader.attach(self.program_id)?;
            }

            // TODO: too simple too make below code a as method
            ShaderProgram::link_program(self.program_id)?;

            self.ready = true;
        }

        Ok(())
    }

    // TODO: may be better interface ?
    pub fn fill_attribute(&mut self, attribute: &mut Attribute) -> Result<(), String> {
        if !self.is_ready() {
            self.compile()?;
        }

        // TODO: get_attrib_location as program's method?
        let location_id = wrapper::get_attrib_location(self.program_id,
                                                       &attribute.name);
        attribute.set_location(location_id as isize);

        Ok(())
    }

    pub fn add_shader(&mut self, shader: Shader) {
        self.shader_collection.insert(shader.label.clone(), shader);
        self.ready = false;
    }

    pub fn remove_shader(&mut self, name: &String) {
        self.shader_collection.remove(name);
        self.ready = false;
    }

    fn aux_create_program(&mut self) -> Result<(), String> {
        let need_create_new_id = !self.can_reuse || !ShaderProgram::is_valid_program_id(self.program_id);

        match need_create_new_id {
            false => Ok(()),
            true => match ShaderProgram::create_program() {
                Ok(new_program_id) => {
                    self.program_id = new_program_id;
                    Ok(())
                }
                Err(error_desc) => Err(error_desc)
            }
        }
    }

    /// use current program
    pub fn activate(&mut self) -> Result<(), String> {
        self.compile()?;

        ShaderProgram::use_program(self.program_id)
    }

    /// reset to default program, namely program 0
    pub fn deactivate(&mut self) -> Result<(), String> {
        // TODO: 0 should be const and global
        ShaderProgram::use_program(0)
    }

    #[inline]
    pub fn get_program_log(&mut self) -> String {
        let mut status = 0;

        let link_status = wrapper::get_programiv(self.program_id, GL_LINK_STATUS);

        if status != GL_TRUE {
            let log_length = wrapper::get_programiv(self.program_id, GL_INFO_LOG_LENGTH);

            match wrapper::get_program_info_log(self.program_id, log_length) {
                Some(log) => log,
                None => "Failed to get program log !!!".to_string()
            }
        } else {
            String::from("Program compile OK")
        }
    }

    // todo: 补充设置Attribute, uniform的数据接口
    pub fn set_argument_value(&self) -> &Self {
        self
    }

    /// only use in debug mode
    pub fn validate(mut self) -> bool {
        true
    }

    pub fn dispatch_compute(mut self) -> bool {
        true
    }

    pub fn set_enable_program_pipeline(mut self, enable: bool) {
        self.enable_program_pipeline = enable;
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
}

impl Drop for ShaderProgram {
    #[inline]
    fn drop(&mut self) {
        if self.ready {
            ShaderProgram::delete_program(self.program_id);

            // TODO: abstract as method
            self.program_id = 0;
            self.ready = false;
        }
    }
}
