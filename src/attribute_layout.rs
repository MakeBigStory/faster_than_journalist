// todo: 可能不需要，目前大家都用float
#[derive(Debug)]
enum AttributeKind {
    Integer,
    Long,
}

// todo: refine Buffer
struct Buffer{}

#[derive(Debug)]
struct AttributeLayout {
    // buffer: Buffer ,
    // kind: DynamicAttribute::Kind ,
    location: GLuint ,
    size: GLint ,
    data_type: GLenum ,
    offset: GLintptr ,
    stride: GLsizei ,
    divisor: GLuint ,
}

impl AttributeLayout {
    fn new(location: GLuint,
    size: GLint,
    data_type: GLenum,
    offset: GLintptr,
    stride: GLsizei,
    divisor: GLuint) -> Self {
        AttributeLayout {
            location,
            size,
            data_type,
            offset,
            stride,
            divisor,
        }
    }
}

impl AttributeLayout {
    pub fn set_location(mut self, location: GLuint) -> Self {
        self.location = location,
        self,
    }

    pub  fn set_size(mut self, size: GLint) -> Self {
        self.size = size,
        self,
    }

    pub fn set_data_type(mut self, data_type: GLenum) -> Self {
        self.data_type = data_type,
        self,
    }

    pub fn set_offset(mut self, offset: GLintptr) -> Self {
        self.offset = offset,
        self,
    }
    pub fn set_stride(mut self, stride: GLsizei) -> Self {
        self.stride = stride,
        self,
    }

    pub fn set_divisor(mut self, divisor: GLuint) -> Self {
        self.divisor = divisor,
        self,
    }
}