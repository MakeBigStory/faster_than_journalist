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

use format::*;

#[derive(Debug, Clone, Hash)]
pub struct BufferDesc {
    pub label: String,
    pub target: BufferType,
    pub usage: BufferUsage,
    pub size: u32,
    pub kind: DataKind,
    pub stride: u32,
}

impl BufferDesc {
    fn new(label: String, target: BufferType, usage: BufferUsage, size: u32,
           kind: DataKind, stride:u32)
           -> BufferDesc {
        BufferDesc {
            label,
            target,
            usage,
            size,
            kind,
            stride,
        }
    }

    fn set_label(&mut self, label: String) {
        self.label = label;
    }

    fn get_label(&self) -> &String {
        &self.label
    }
}

#[derive(Clone, Debug, Hash)]
pub struct Buffer {
    pub desc: BufferDesc,
    pub raw: Option<u32>,
}

impl PartialEq for Buffer{
    fn eq(&self, other: &Buffer) -> bool {
        match self.raw {
            Some(id) => {
                match other.raw {
                    Some(other_id) => {
                        id == other_id
                    },
                    None=>{
                        false
                    }
                }
            },
            None=>{false}
        }
    }
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

    //todo: CPU data unpack to GPU,高版本或许可以使用mapbufferRange来实现高速资源拷贝
    pub fn write_sub_buffer_data<T>(&self, offset: u32, size: u32, data :&[T])
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

impl Drop for Buffer {
    fn drop(&mut self) {
        match self.raw {
            Some(id) => {
                es20::wrapper::delete_buffers(&[id]);
                self.raw = None;
            },
            _ => {}
        }
    }
}