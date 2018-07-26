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

#[derive(Debug, Clone)]
enum MemoryTarget{
    ArrayBuffer = 0,
    ElementArrayBuffer,

    PackBuffer,
    UnpackBuffer,
    UniformBuffer,

    TransformFeedbackBuffer,
    DrawIndirectBuffer,

    //feiper: for copy buffer to another
    //COPY_READ_BUFFER,
    //COPY_WRITE_BUFFER,

}

fn memory_type_to_glsl_target(target: &MemoryTarget) -> es20d::GLenum {
    match target {
        MemoryTarget::ArrayBuffer => es20d::GL_ARRAY_BUFFER,
        MemoryTarget::ElementArrayBuffer => es20d::GL_ELEMENT_ARRAY_BUFFER,
        MemoryTarget::PackBuffer => es30d::GL_PIXEL_PACK_BUFFER,
        MemoryTarget::UnpackBuffer => es30d::GL_PIXEL_UNPACK_BUFFER,
        MemoryTarget::UnpackBuffer => es31d::GL_DRAW_INDIRECT_BUFFER,
        MemoryTarget::UniformBuffer => es30d::GL_UNIFORM_BUFFER,
        MemoryTarget::TransformFeedbackBuffer => es30d::GL_TRANSFORM_FEEDBACK_BUFFER,
        MemoryTarget::DrawIndirectBuffer => es31d::GL_DRAW_INDIRECT_BUFFER,
        //MemoryTarget::COPY_READ_BUFFER => es30d::GL_COPY_READ_BUFFER,
        //MemoryTarget::COPY_WRITE_BUFFER => es30d::GL_COPY_WRITE_BUFFER,
    }
}


#[derive(Debug, Clone)]
enum MemoryUsage{
    StreamDraw = 0,
    StreamRead,
    StreamCopy,
    StaticDraw,
    StaticRead,
    StaticCopy,
    DynamicDraw,
    DynamicRead,
    DynamicCopy,
}

fn memory_usage_to_glsl_usage(usage: &MemoryUsage) -> es20d::GLenum {
    match usage {
        MemoryUsage::StreamDraw => es20d::GL_STREAM_DRAW,
        MemoryUsage::StreamRead => es30d::GL_STREAM_READ,
        MemoryUsage::StreamCopy => es30d::GL_STREAM_COPY,

        MemoryUsage::StaticDraw => es20d::GL_STATIC_DRAW,
        MemoryUsage::StaticRead => es30d::GL_STATIC_READ,
        MemoryUsage::StaticCopy => es30d::GL_STATIC_READ,

        MemoryUsage::DynamicDraw => es20d::GL_DYNAMIC_DRAW,
        MemoryUsage::DynamicRead => es30d::GL_DYNAMIC_READ,
        MemoryUsage::DynamicCopy => es30d::GL_DYNAMIC_COPY,
    }
}

#[derive(Debug, Clone)]
pub struct GPUMemoryDesc{
    target: MemoryTarget,
    usage: MemoryUsage,
    size: u32,
}

impl GPUMemoryDesc {
    fn new(target: MemoryTarget, usage: MemoryUsage, size : u32)
        -> GPUMemoryDesc {
        GPUMemoryDesc{
            target,
            usage,
            size,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Buffer {
    pub name: String,
    memory: GPUMemoryDesc,
    raw: u32,
}

impl Buffer {
    //if allocate a buffer without size, we will get size when write data automatically
    pub fn new(name: String, desc: &GPUMemoryDesc) -> Self {
        let raw = es20::wrapper::gen_buffers(1)[0];
        let target = memory_type_to_glsl_target(&(desc.target));
        let usage = memory_usage_to_glsl_usage(&desc.usage);

        es20::wrapper::bind_buffer(target, raw);
        es20::wrapper::buffer_data(target,
                                   desc.size as es20d::GLsizeiptr,
                                   ptr::null(),
                                   usage);
        es20::wrapper::bind_buffer(target, 0);

        Buffer {
            name,
            memory: desc.clone(),
            raw,
        }
    }

    pub fn new_with_data<T>(name: String, desc: &GPUMemoryDesc, data: &[T]) -> Buffer {
        let raw = es20::wrapper::gen_buffers(1)[0];
        let target = memory_type_to_glsl_target(&(desc.target));
        let usage = memory_usage_to_glsl_usage(&desc.usage);

        //if data.len() * std::mem::size::<T>() > desc.size {
        //    panic!("Buffer::new_with_data: data bytes greater than  GPUMemory size {:?}", desc);
        //}

        es20::wrapper::bind_buffer(target, raw);
        es20::wrapper::buffer_data(target,
                                   desc.size as es20d::GLsizeiptr,
                                   data.as_ptr() as * const es20d::GLvoid,
                                   usage);
        es20::wrapper::bind_buffer(target, 0);

        Buffer {
            name,
            memory: desc.clone(),
            raw,
        }
    }

    //CPU data unpack to GPU
    pub fn write_buffer_data<T>(&self, offset: u32, size: u32, data :&[T])
        -> Option<&Buffer> {
        if self.memory.size == 0 {
            eprintln!("Buffer::write_data: hasn't been allocate a GPUMemory {:?}", self);
            return None;
        }

        let mut real_size = size;
        if (offset + size) > self.memory.size {
            eprintln!("Buffer::write_data: override GPUMemory {:?}, do clamp Operation", self.memory);
            real_size = &self.memory.size - offset;
        }

        let target = memory_type_to_glsl_target(&self.memory.target);
        self.bind();
        es20::wrapper::buffer_sub_data(target,
                                       offset as _,
                                       real_size as _,
                                       data.as_ptr() as *const es20d::GLvoid);
        self.unbind();
        Some(&self)
    }

    fn bind(&self){
        let target = memory_type_to_glsl_target(&self.memory.target);
        es20::wrapper::bind_buffer(target, self.raw);
    }

    fn unbind(&self){
        let target = memory_type_to_glsl_target(&self.memory.target);
        es20::wrapper::bind_buffer(target, 0);
    }
}
