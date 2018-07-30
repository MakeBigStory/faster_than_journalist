//// todo: 可能不需要，目前大家都用float
//use gles::es20::data_struct as es20d;
//use gles::es30::data_struct as es30d;
//use gles::es31::data_struct as es31d;
//use gles::es32::data_struct as es32d;
//
//use gles::es20;
//use gles::es30;
//use gles::es31;
//use gles::es32;
//
//use format::*;
//
//
////todo: 有点看不懂，感觉跟attribute冲突。。我个人怀疑是不是想要一个类似于glVertexArrayPointer??
//#[derive(Debug)]
//struct AttributeLayout {
//    // buffer: Buffer ,
//    // kind: DynamicAttribute::Kind ,
//    location: u32 ,
//    size: u32 ,
//    data_type: DataKind ,
//    offset: u32 ,
//    stride: u32 ,
//    // TODO: if needed ?
//    divisor: GLuint ,
//}
//
//impl AttributeLayout {
//    fn new(location: GLuint,
//    size: GLint,
//    data_type: GLenum,
//    offset: GLintptr,
//    stride: GLsizei,
//    divisor: GLuint) -> Self {
//        AttributeLayout {
//            location,
//            size,
//            data_type,
//            offset,
//            stride,
//            divisor,
//        }
//    }
//}
//
//impl AttributeLayout {
//    pub fn set_location(&mut self, location: GLuint) -> &mut Self {
//        self.location = location;
//        self
//    }
//
//    pub  fn set_size(&mut self, size: GLint) -> &mut Self {
//        self.size = size;
//        self
//    }
//
//    pub fn set_data_type(&mut self, data_type: GLenum) -> &mut Self {
//        self.data_type = data_type;
//        self
//    }
//
//    pub fn set_offset(&mut self, offset: GLintptr) -> &mut Self {
//        self.offset = offset;
//        self
//    }
//    pub fn set_stride(&mut self, stride: GLsizei) -> &mut Self {
//        self.stride = stride;
//        self
//    }
//
//    pub fn set_divisor(&mut self, divisor: GLuint) -> &mut Self {
//        self.divisor = divisor;
//        self
//    }
//}