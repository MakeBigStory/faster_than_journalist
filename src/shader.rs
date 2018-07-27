#[derive(Debug)]
enum ShaderType {
    Vertex,
    Fragment,
    Compute,
    TessellationControl,
    TessellationEvaluation,
}

#[derive(Debug)]
enum ShaderLanguageVersion {
    Version100,
    Version300,
    Version310,
    Version320,
}

#[derive(Debug)]
struct Shader {
    label: String,
    shader_type: ShaderType,
    source: String,
    enable_shader_reuse: bool,
    version: ShaderLanguageVersion,
}

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

 //   /// Compiles a shader.
 //   ///
 //   /// Returns a shader or a message with the error.
//    pub fn compile(mut self, shader_type: GLenum, source: &str) -> Result<GLuint, String>  {
//        unsafe {
//                    let shader = gl::CreateShader(shader_type);
//                    let c_source = match CString::new(source) {
//                        Ok(x) => x,
//                        Err(err) => return Err(format!("compile_shader: {}", err)),
//                    };
//                    gl::ShaderSource(shader, 1, &c_source.as_ptr(), ptr::null());
//                    drop(source);
//                    gl::CompileShader(shader);
//                    let mut status = gl::FALSE as GLint;
//                    gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
//                    if status == (gl::TRUE as GLint) {
//                        Ok(shader)
//                    } else {
//                        let mut len = 0;
//                        gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
//
//                        if len == 0 {
//                            Err("Compilation failed with no log. \
//                                 The OpenGL context might have been created on another thread, \
//                                 or not have been created."
//                                .to_string())
//                        } else {
//                            // Subtract 1 to skip the trailing null character.
//                            let mut buf = vec![0; len as usize - 1];
//                            gl::GetShaderInfoLog(
//                                shader,
//                                len,
//                                ptr::null_mut(),
//                                buf.as_mut_ptr() as *mut GLchar,
//                            );
//
//                            gl::DeleteShader(shader);
//
//                            Err(String::from_utf8(buf)
//                                .ok()
//                                .expect("ShaderInfoLog not valid utf8"))
//                        }
//                    }
//                }
//    }

 //   /// Finds attribute location from a program.
 //   ///
 //   /// Returns `Err` if there is no attribute with such name.
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

  //  /// Finds uniform location from a program.
  //  ///
  //  /// Returns `Err` if there is no uniform with such name.
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

//    #[inline]
//    pub fn check_shader_status(shader: GLuint) -> GLuint {
//        let mut status = 0;
//        unsafe { gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status) };
//        if status != (gl::TRUE as GLint) {
//            let mut len = 0;
//            unsafe {
//                gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
//            }
//            let mut buf = Vec::with_capacity(len as usize);
//            unsafe {
//                buf.set_len(len as usize);
//                gl::GetShaderInfoLog(
//                    shader,
//                    len,
//                    ptr::null_mut(),
//                    buf.as_mut_ptr() as *mut GLchar,
//                );
//            }
//            panic!(
//                "{}",
//                str::from_utf8(&buf)
//                    .ok()
//                    .expect("ShaderInfoLog not valid utf8")
//            );
//        }
//        shader
//    }
//
//    // todo: 安卓看情况实现
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
