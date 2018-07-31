use gles::es20::data_struct::*;
use gles::es20::*;

#[derive(Debug, Copy, Clone)]
pub enum ShaderType {
    Vertex = GL_VERTEX_SHADER as isize,
    Fragment = GL_FRAGMENT_SHADER as isize,
// TODO: es20不支持，暂时注释
//    Compute,
//    TessellationControl,
//    TessellationEvaluation,
}

#[derive(Debug, Copy, Clone)]
pub enum ShaderLanguageVersion {
    Version100,
    Version300,
    Version310,
    Version320,
}

#[derive(Debug)]
pub struct Shader {
    pub label: String,
    shader_type: ShaderType,
    source: String,
    can_reuse: bool,
    version: ShaderLanguageVersion,
    shader_id : GLuint,
    need_compile : bool
}

impl Shader {
    pub fn new(label: String, shader_type: ShaderType) -> Self {
        Shader {
            label,
            shader_type,
            source: String::from(""),
            can_reuse: false,
            version: ShaderLanguageVersion::Version100,
            shader_id: 0,
            need_compile: true
        }
    }

    pub fn is_valid_shader(shader_id: GLuint) -> bool {
        let id_valid = match shader_id {
            0 => false,
            _ => true
        };

        match id_valid {
            // TODO: is_shader是否检查代价有点大
            // TODO: get shader iv的结果是否也需要参考
            true => wrapper::is_shader(shader_id),
            false => false
        }
    }

    pub fn attach_shader(program_id : GLuint, shader_id : GLuint) -> Result<(), String> {
        wrapper::attach_shader(program_id, shader_id);

        // TODO: glGetError check
        Ok(())
    }

    pub fn detach_shader(program_id : GLuint, shader_id : GLuint) -> Result<(), String> {
        wrapper::detach_shader(program_id, shader_id);

        // TODO: glGetError check
        Ok(())
    }

    pub fn create_shader(shader_type: GLenum) -> Result<GLuint, String> {
        let shader_id = wrapper::create_shader(shader_type);

        // TODO: 和is_valid_shader有dup
        // TODO: glGetError check
        match shader_id {
            0 => Err("Can not generate shader id".to_string()),
            _ => Ok(shader_id)
        }
    }

    pub fn delete_shader(shader_id: GLuint) -> Result<(), String> {
        wrapper::delete_shader(shader_id);

        // TODO: glGetError check
        Ok(())
    }

    // TODO: need shader count
    pub fn shader_source(shader_id: GLuint, source: &String) -> Result<(), String> {
        wrapper::shader_source(shader_id, &source[..].as_bytes());

        // TODO: glGetError check
        Ok(())
    }

    pub fn compile_shader(shader_id: GLuint) -> Result<(), String> {
        wrapper::compile_shader(shader_id);

        // TODO: glGetError check
        Ok(())
    }

    pub fn get_shader_info(shader_id: GLuint, shader_param_name: GLenum) -> Result<GLint, String> {
        let res = wrapper::get_shaderiv(shader_id, shader_param_name);

        // TODO: glGetError check
        Ok(res)
    }

    pub fn check_shader_complete_status(shader_id: GLuint) -> bool {
        match Shader::get_shader_info(shader_id, GL_COMPILE_STATUS) {
            Ok(res) => {
                // TODO: ugly
                res == (GL_TRUE as i32)
            }
            Err(error_desc) => false
        }

        // TODO: glGetError check
    }

    // TODO: is glGetError check needed ?
    pub fn get_shader_log(shader_id: GLuint) -> Option<String> {
        let mut log_length
        = Shader::get_shader_info(shader_id, GL_INFO_LOG_LENGTH);

        match log_length {
            Ok(log_length) if log_length > 0 => {
                wrapper::get_shader_info_log(shader_id, log_length)
            }
            Ok(_) => None,
            Err(_) => None
        }
    }

    // TODO: 和静态的create_shader命名冲突，暂时加上aux前缀区分
    fn aux_create_shader(&mut self) -> Result<(), String> {
        let need_recreate_shader: bool = !self.can_reuse || !Shader::is_valid_shader(self.shader_id);

        if false == need_recreate_shader {
            return Ok(());
        } else {
            let create_res = Shader::create_shader(self.shader_type as u32);

            match create_res {
                Ok(new_shader_id) => {
                    match Shader::is_valid_shader(new_shader_id) {
                        true => {
                            self.shader_id = new_shader_id;
                            Ok(())
                        }
                        false => Err("Can not create shader Id !!!!".to_string())
                    }
                }
                Err(_) => Err("Can not create shader Id !!!!".to_string())
            }
        }
    }

    pub fn set_source(&mut self, new_source: String) {
        self.source = new_source;
        self.need_compile = true;
    }

    pub fn attach(&mut self, program_id : GLuint) -> Result<(), String> {
        Shader::attach_shader(program_id, self.shader_id)
    }

    pub fn detach(&mut self, program_id : GLuint) -> Result<(), String> {
        Shader::detach_shader(program_id, self.shader_id)
    }

    /// Compiles a shader.
    ///
    /// Returns a shader or a message with the error.
    pub fn compile(&mut self) -> Result<(), String> {
        if !self.need_compile {
            return Ok(())
        }

        self.aux_create_shader()?;
        Shader::shader_source(self.shader_id, &(self.source))?;
        // TODO: source保留代价不大，可以先留着。
//            drop(source);

        Shader::compile_shader(self.shader_id)?;

        let check_complete_res = Shader::check_shader_complete_status(self.shader_id);

        if !check_complete_res {
            let log = Shader::get_shader_log(self.shader_id);

            return match log {
                Some(error_desc) => Err(error_desc),
                None => Err("Shader compile error, but no log ....".to_string())
            }
        }

        self.need_compile = true;

        Ok(())
    }
}
    // Finds attribute location from a program.
    //
    // Returns `Err` if there is no attribute with such name.
//    pub fn attribute_location(program: GLuint, name: &str) -> Result<GLuint, String> {
//        unsafe {
//            let c_name = match CString::new(name) {
//                Ok(x) => x,
//                Err(err) => return Err(format!("attribute_location: {}", err)),
//            };
//            let id = gl::GetAttribLocation(program, c_name.as_ptr());
//            drop(c_name);
//            if id < 0 {
//                Err(format!("Attribute '{}' does not exists in shader", name))
//            } else {
//                Ok(id as GLuint)
//            }
//        }
//    }
//
//    /// Finds uniform location from a program.
//    ///
//    /// Returns `Err` if there is no uniform with such name.
//    pub fn uniform_location(program: GLuint, name: &str) -> Result<GLuint, String> {
//        unsafe {
//            let c_name = match CString::new(name) {
//                Ok(x) => x,
//                Err(err) => return Err(format!("uniform_location: {}", err)),
//            };
//            let id = gl::GetUniformLocation(program, c_name.as_ptr());
//            drop(c_name);
//            if id < 0 {
//                Err(format!("Uniform '{}' does not exists in shader", name))
//            } else {
//                Ok(id as GLuint)
//            }
//        }
//    }
//
// todo: 安卓看情况实现
//    pub fn get_shader_binary(mut self) {}
//pub fn max_tessellation_evaluation_output_components() -> u8 {}
//pub fn max_tessellation_evaluation_input_components() -> u8 {}
//pub fn max_tessellation_control_total_output_components() -> u8 {}
//pub fn max_tessellation_control_output_components() -> u8 {}
//pub fn max_tessellation_control_input_components() -> u8 {}
//pub fn max_vertex_output_components() -> u8 {}
//pub fn max_fragment_input_components() -> u8 {}
//pub fn max_uniform_components() -> u8 {}
//pub fn max_image_uniforms() -> u8 {}
//pub fn max_combined_image_uniforms() -> u8 {}
//pub fn max_shader_storage_blocks() -> u8 {}
//pub fn max_combined_shader_storage_blocks() -> u8 {}
//pub fn max_texture_image_units() -> u8 {}
//pub fn max_combined_texture_image_units() -> u8 {}
//pub fn max_uniform_blocks() -> u8 {}
//pub fn max_combined_uniform_blocks() -> u8 {}
//pub fn max_combined_uniform_components() -> u8 {}
//}
