#[derive(Debug, Clone, Hash)]
pub struct Uniform {
    kind: UniformKind,
    count: usize,
    location: usize,
}

impl Uniform {
    #[inline(always)]
    pub fn new(kind: UniformKind, count: usize, location: usize) -> Self {
        Uniform {
            kind,
            count,
            location,
        }
    }

    #[inline(always)]
    pub fn kind(&self) -> &UniformKind {
        &self.kind
    }
    #[inline(always)]
    pub fn count(&self) -> usize {
        self.count
    }
    #[inline(always)]
    pub fn location(&self) -> usize {
        self.location
    }
}


macro_rules! create_set_uniform {
    ($name: ident, $func: ident, $kind: ident) => (
        #[inline]
        pub fn $name(&self, value: $kind) {
            unsafe {
                gl::$func(self.location as GLint, value);
            }
        }
    )
}

macro_rules! create_set_vec_uniform {
    ($name: ident, $func: ident, $kind: ident, $item_count: expr) => (
        #[inline]
        pub fn $name(&self, value: &[$kind; $item_count]) {
            unsafe {
                gl::$func(self.location as GLint, 1 as GLint, value.as_ptr());
            }
        }
    )
}

macro_rules! create_set_matrix_uniform {
    ($name: ident, $func: ident, $kind: ident, $item_count: expr) => (
        #[inline]
        pub fn $name(&self, value: &[$kind; $item_count]) {
            unsafe {
                gl::$func(self.location as GLint, 1 as GLint, gl::FALSE, value.as_ptr());
            }
        }
    )
}

impl Uniform {
    create_set_uniform!(set_1f, Uniform1f, f32);
    create_set_uniform!(set_1i, Uniform1i, i32);

    create_set_vec_uniform!(set_2f, Uniform2fv, f32, 2);
    create_set_vec_uniform!(set_2i, Uniform2iv, i32, 2);

    create_set_vec_uniform!(set_3f, Uniform3fv, f32, 3);
    create_set_vec_uniform!(set_3i, Uniform3iv, i32, 3);

    create_set_vec_uniform!(set_4f, Uniform4fv, f32, 4);
    create_set_vec_uniform!(set_4i, Uniform4iv, i32, 4);

    create_set_matrix_uniform!(set_mat2f, UniformMatrix2fv, f32, 4);
    create_set_matrix_uniform!(set_mat3f, UniformMatrix3fv, f32, 9);
    create_set_matrix_uniform!(set_mat4f, UniformMatrix4fv, f32, 16);
}

macro_rules! create_set_vec_uniform_size {
    ($name: ident, $func: ident, $kind: ident) => (
        #[inline]
        pub fn $name(&self, values: &[$kind]) {
            unsafe {
                gl::$func(self.location as GLint, self.count as GLint, values.as_ptr());
            }
        }
    )
}

macro_rules! create_set_matrix_uniform_size {
    ($name: ident, $func: ident, $kind: ident) => (
        #[inline]
        pub fn $name(&self, values: &[$kind]) {
            unsafe {
                gl::$func(self.location as GLint, self.count as GLint, gl::FALSE, values.as_ptr());
            }
        }
    )
}

impl Uniform {
    create_set_vec_uniform_size!(set_2fv, Uniform2fv, f32);
    create_set_vec_uniform_size!(set_2iv, Uniform2iv, i32);

    create_set_vec_uniform_size!(set_3fv, Uniform3fv, f32);
    create_set_vec_uniform_size!(set_3iv, Uniform3iv, i32);

    create_set_vec_uniform_size!(set_4fv, Uniform4fv, f32);
    create_set_vec_uniform_size!(set_4iv, Uniform4iv, i32);

    create_set_matrix_uniform_size!(set_mat2fv, UniformMatrix2fv, f32);
    create_set_matrix_uniform_size!(set_mat3fv, UniformMatrix3fv, f32);
    create_set_matrix_uniform_size!(set_mat4fv, UniformMatrix4fv, f32);

    #[inline]
    pub fn set_sampler_2d(&self, texture: &GLTexture, index: usize) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index as GLuint);
            gl::Uniform1i(self.location as GLint, index as GLint);
            gl::BindTexture(texture.kind().into(), texture.id());
        }
    }
}