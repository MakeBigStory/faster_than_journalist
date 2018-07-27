///Copyright reserve@feiper

use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es30;
use gles::es31;
use gles::es32;

use std::ptr;
use std::fmt;
use std::fmt::Formatter;
use std::error::Error;
use std::mem;


// TODO: 为什么不能直接 as u32?
#[derive(Copy, Debug, Clone)]
pub enum BufferType {
    ArrayBuffer = es20d::GL_ARRAY_BUFFER as isize,
    ElementArrayBuffer = es20d::GL_ELEMENT_ARRAY_BUFFER as isize,

    PixelPackBuffer = es30d::GL_PIXEL_PACK_BUFFER as isize,
    PixelUnpackBuffer = es30d::GL_PIXEL_UNPACK_BUFFER as isize,
    UniformBuffer = es30d::GL_UNIFORM_BUFFER as isize,

    TransformFeedbackBuffer = es30d::GL_TRANSFORM_FEEDBACK_BUFFER as isize,
    DrawIndirectBuffer = es31d::GL_DRAW_INDIRECT_BUFFER as isize,

    //feiper: for copy buffer to another
    //COPY_READ_BUFFER,
    //COPY_WRITE_BUFFER,

}

#[derive(Copy, Debug, Clone)]
pub enum BufferUsage {
    /// Set once by the application and used infrequently for drawing.
    StreamDraw = es20d::GL_STREAM_DRAW as isize,
    /// Set once as output from an OpenGL command and used infequently for drawing.
    StreamRead = es30d::GL_STREAM_READ as isize,
    /// Set once as output from an OpenGL command and used infrequently for drawing or copying to other buffers.
    StreamCopy = es30d::GL_STREAM_COPY as isize,

    /// Set once by the application and used frequently for drawing. A good default choice if you are not sure.
    StaticDraw = es20d::GL_STATIC_DRAW as isize,
    /// Set once as output from an OpenGL command and queried many times by the application.
    StaticRead = es30d::GL_STATIC_READ as isize,
    /// Set once as output from an OpenGL command and used frequently for drawing or copying to other buffers.
    StaticCopy = es30d::GL_STATIC_COPY as isize,

    /// Updated frequently by the application and used frequently for drawing or copying to other images.
    DynamicDraw = es20d::GL_DYNAMIC_DRAW as isize,
    /// Updated frequently as output from OpenGL command and queried many times from the application.
    DynamicRead = es30d::GL_DYNAMIC_READ as isize,
    /// Updated frequently as output from OpenGL command and used frequently for drawing or copying to other images.
    DynamicCopy = es30d::GL_DYNAMIC_COPY as isize,
}

#[derive(Debug, Clone)]
pub struct BufferDesc {
    pub label: String,
    target: BufferType,
    usage: BufferUsage,
    size: u32,
}

impl BufferDesc {
    fn new(label:String, target: BufferType, usage: BufferUsage, size : u32)
           -> BufferDesc {
        BufferDesc {
            label,
            target,
            usage,
            size,
        }
    }

    fn set_lable(&mut self, label: String) {
        self.label = label;
    }

    fn get_lable(&self) -> &String {
        &self.label
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    desc: BufferDesc,
    raw: Option<u32>,
}

impl Buffer {
    //if allocate a buffer without size, we will get size when write data automatically
    pub fn new(desc: &BufferDesc) -> Self {
        let raw = es20::wrapper::gen_buffers(1)[0];

        let target = desc.target.clone() as es20d::GLenum;
        let usage = desc.usage as u32;

        es20::wrapper::bind_buffer(target, raw);
        es20::wrapper::buffer_data(target,
                                   desc.size as es20d::GLsizeiptr,
                                   ptr::null(),
                                   usage);
        es20::wrapper::bind_buffer(target, 0);

        Buffer {
            desc: desc.clone(),
            raw: Some(raw)
        }
    }

    pub fn new_with_data<T>(name: String, desc: &BufferDesc, data: &[T]) -> Buffer {
        let raw = es20::wrapper::gen_buffers(1)[0];
        let target = desc.target as u32;
        let usage = desc.usage as u32;

        let real_size =  data.len() as usize * mem::size_of::<T>();
        if real_size != desc.size as usize {
            panic!("Buffer::new_with_data: data bytes greater than  GPUMemory size {:?}", desc);
        }

        es20::wrapper::bind_buffer(target, raw);
        es20::wrapper::buffer_data(target,
                                   desc.size as es20d::GLsizeiptr,
                                   data.as_ptr() as * const es20d::GLvoid,
                                   usage);
        es20::wrapper::bind_buffer(target, 0);

        Buffer {
            desc: desc.clone(),
            raw: Some(raw),
        }
    }

    //CPU data unpack to GPU
    pub fn write_buffer_data<T>(&self, offset: u32, size: u32, data :&[T])
        -> Option<&Buffer> {
        if self.desc.size == 0 {
            eprintln!("Buffer::write_data: hasn't been allocate a GPUMemory {:?}", self);
            return None;
        }

        let mut real_size = size;
        if (offset + size) > self.desc.size {
            eprintln!("Buffer::write_data: override GPUMemory {:?}, do clamp Operation", self.desc);
            real_size = &self.desc.size - offset;
        }

        let target = self.desc.target as u32;
        self.bind();
        es20::wrapper::buffer_sub_data(target,
                                       offset as _,
                                       real_size as _,
                                       data.as_ptr() as *const es20d::GLvoid);
        self.unbind();
        Some(&self)
    }

    fn bind(&self){
        let target = self.desc.target as u32;
        es20::wrapper::bind_buffer(target, self.raw.unwrap());
    }

    fn unbind(&self){
        let target = self.desc.target as u32;
        es20::wrapper::bind_buffer(target, 0);
    }
}
