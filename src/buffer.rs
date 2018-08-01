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

#[derive(Clone, Debug, Hash)]
pub struct Buffer {
    pub name: String,
    pub target: BufferType,
    pub usage: BufferUsage,
    size: usize,
    buffer_id : u32,
    ready: bool
}

impl Buffer {
    pub fn new(name: &str) -> Self {
        Buffer {
            name: name.to_string(),
            target: BufferType::ArrayBuffer,
            usage: BufferUsage::StaticDraw,
            size: 0,
            buffer_id : 0,
            ready: false
        }
    }

    fn set_target(&mut self, target: BufferType) -> &mut Self {
        self.target = target;
        self
    }

    fn set_usage(&mut self, usage: BufferUsage) -> &mut Self {
        self.usage = usage;
        self
    }

    fn init(&mut self) -> Result<(), String> {
        if self.ready {
            return Ok(())
        }

        let buffer_id = es20::wrapper::gen_buffers(1)[0];

        match buffer_id {
            0 => Err("Fail to generate buffer !!!"),
            _ => {
                self.buffer_id = buffer_id;
                Ok(())
            }
        }?;

        self.ready = true;

        Ok(())
    }

    // TODO: error check
    pub fn bind(&mut self) -> Result<(), String> {
        if !self.ready {
            self.init()?;
        }

        // TODO: wrapper param should be more precise
        es20::wrapper::bind_buffer(self.target as u32, self.buffer_id);
        Ok(())
    }

    // TODO: error check
    // TODO: unbind as static method ?
    pub fn unbind(&mut self) -> Result<(), String> {

        // TODO: wrapper param should be more precise
        es20::wrapper::bind_buffer(self.target as u32, 0);
        Ok(())
    }

    pub fn set_data<T>(&mut self, data: &[T]) -> Result<(), String> {
        self.bind()?;

        self.size = data.len() * mem::size_of::<T>();

        // TODO: wrapper param should be more precise
        // TODO: error check
        es20::wrapper::buffer_data(self.target as u32,
                                   self.size as i32,
                                   data.as_ptr() as * const es20d::GLvoid,
                                   self.usage as u32);

        self.unbind()?;

        Ok(())
    }

    //todo: CPU data unpack to GPU,高版本或许可以使用mapbufferRange来实现高速资源拷贝
    pub fn write_sub_buffer_data<T>(&mut self, offset: u32, size: u32, data :&[T])
        -> Result<(), String> {
// TODO: 下面的安全校验可以单独抽为Validation
//        if self.size == 0 {
//            eprintln!("Buffer::write_data: hasn't been allocate a GPUMemory {:?}", self);
//            return None;
//        }
//
//        let mut real_size = size;
//        if (offset + size) > self.size {
//            eprintln!("Buffer::write_data: override GPUMemory {:?}, do clamp Operation", self.desc);
//            real_size = &self.size - offset;
//        }

        self.bind()?;

        let sub_data_size = data.len() * mem::size_of::<T>();

        // TODO: error check
        es20::wrapper::buffer_sub_data(self.target as u32,
                                       offset as _,
                                       sub_data_size as i32,
                                       data.as_ptr() as *const es20d::GLvoid);
        self.unbind()?;

        Ok(())
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        if self.ready {
            // TODO: glIsBuffer check
            // TODO: wrapper can be more friendly, no need array, but just a number
            let buffer_ids = [self.buffer_id];
            es20::wrapper::delete_buffers(&buffer_ids);

            self.ready = false;
            self.buffer_id = 0;
        }
    }
}