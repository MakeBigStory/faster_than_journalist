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
//use std::collections::HashMap;
//
//use attribute::Attribute;
//use buffer::Buffer;
//
////mapping to glEnableVertexArrayPointer..
//#[derive(Debug, Clone)]
//pub struct ShaderArgumentVertex<'a> {
//    pub attribute: &'a Attribute,
//    pub stride: &'a Buffer,
//    pub offset: u32,
//}
//
//impl<'a> ShaderArgumentVertex<'a> {
//
//
//    fn new(attribute: &'a Attribute, stride: &'a Buffer, ) {
//
//    }
//
//    pub fn enable(&self) {
//        es20::wrapper::enable_vertex_attrib_array(self.attribute.location as _);
//    }
//
//    pub fn disable(&self) {
//        es20::wrapper::disable_vertex_attrib_array(self.attribute.location as _);
//    }
//
//    pub fn bind(&mut self) {
//
//    }
//}