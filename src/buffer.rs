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


#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum BufferUsage {
    StreamDraw = es20d::GL_STREAM_DRAW as isize,
    StreamRead = es30d::GL_STREAM_READ as isize,
    StreamCopy = es30d::GL_STREAM_COPY as isize,

    StaticDraw = es20d::GL_STATIC_DRAW as isize,
    StaticRead = es30d::GL_STATIC_READ as isize,
    StaticCopy = es30d::GL_STATIC_COPY as isize,

    DynamicDraw = es20d::GL_DYNAMIC_DRAW as isize,
    DynamicRead = es30d::GL_DYNAMIC_READ as isize,
    DynamicCopy = es30d::GL_DYNAMIC_COPY as isize,
}

trait TransferEnum {
    fn transfer(&self) -> es20d::GLenum;
}

impl TransferEnum for BufferType {
    fn transfer(&self) -> es20d::GLenum {
        let value = self.clone();
        value as u32
    }
}

impl TransferEnum for BufferUsage {
    fn transfer(&self) -> es20d::GLenum {
        let value = self.clone();
        value as u32
    }
}

#[derive(Debug, Clone)]
pub struct BufferDesc {
    target: BufferType,
    usage: BufferUsage,
    size: u32,
}

impl BufferDesc {
    fn new(target: BufferType, usage: BufferUsage, size : u32)
           -> BufferDesc {
        BufferDesc {
            target,
            usage,
            size,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    pub label: String,
    desc: BufferDesc,
    raw: Option<u32>,
}

impl Buffer {
    //if allocate a buffer without size, we will get size when write data automatically
    pub fn new(name: String, desc: &BufferDesc) -> Self {
        let raw = es20::wrapper::gen_buffers(1)[0];

        let target = desc.target.clone() as es20d::GLenum;
        let usage = desc.usage.transfer();

        es20::wrapper::bind_buffer(target, raw);
        es20::wrapper::buffer_data(target,
                                   desc.size as es20d::GLsizeiptr,
                                   ptr::null(),
                                   usage);
        es20::wrapper::bind_buffer(target, 0);

        Buffer {
            label: name,
            desc: desc.clone(),
            raw: Some(raw)
        }
    }

    pub fn new_with_data<T>(name: String, desc: &BufferDesc, data: &[T]) -> Buffer {
        let raw = es20::wrapper::gen_buffers(1)[0];
        let target = desc.target.transfer();
        let usage = desc.usage.transfer();

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
            label: name,
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

        let target = self.desc.target.transfer();
        self.bind();
        es20::wrapper::buffer_sub_data(target,
                                       offset as _,
                                       real_size as _,
                                       data.as_ptr() as *const es20d::GLvoid);
        self.unbind();
        Some(&self)
    }

    fn bind(&self){
        let target = self.desc.target.transfer();
        es20::wrapper::bind_buffer(target, self.raw.unwrap());
    }

    fn unbind(&self){
        let target = self.desc.target.transfer();
        es20::wrapper::bind_buffer(target, 0);
    }
}
