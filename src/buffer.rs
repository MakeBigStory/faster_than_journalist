use gles::es20::data_struct as es20d;
use gles::es30::data_struct as es30d;
use gles::es31::data_struct as es31d;
use gles::es32::data_struct as es32d;

use gles::es20;
use gles::es30;
use gles::es31;
use gles::es32;

use std::error::Error;
use std::fmt;
use std::fmt::Formatter;
use std::mem;
use std::ptr;

use format::*;

/// Memory mapping access
#[derive(Copy, Clone, Debug)]
pub enum BufferUsage {
    /// Map buffer for reading only.
    ReadOnly = GL_READ_ONLY,

    /// Map buffer for writing only.
    WriteOnly = GL_WRITE_ONLY,
    // todo: conditional compile
    //WriteOnly = GL_WRITE_ONLY_OES
    /// Map buffer for both reading and writing.
    ReadWrite = GL_READ_WRITE,
}

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
    fn new(
        label: String,
        target: BufferType,
        usage: BufferUsage,
        size: u32,
        kind: DataKind,
        stride: u32,
    ) -> BufferDesc {
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

impl PartialEq for Buffer {
    fn eq(&self, other: &Buffer) -> bool {
        match self.raw {
            Some(id) => match other.raw {
                Some(other_id) => id == other_id,
                None => false,
            },
            None => false,
        }
    }
}

impl Buffer {
    //if allocate a buffer without size, we will get size when write data automatically
    pub fn new(desc: &BufferDesc) -> Self {
        let raw = gl_gen_buffers(1)[0];

        let target = desc.target.clone() as es20d::GLenum;
        let usage = desc.usage as u32;

        gl_bind_buffer(target, raw);
        gl_buffer_data(target, desc.size as es20d::GLsizeiptr, ptr::null(), usage);
        gl_bind_buffer(target, 0);

        Buffer {
            desc: desc.clone(),
            raw: Some(raw),
        }
    }

    pub fn new_with_data<T>(name: String, desc: &BufferDesc, data: &[T]) -> Buffer {
        let raw = gl_gen_buffers(1)[0];
        let target = desc.target as u32;
        let usage = desc.usage as u32;

        let real_size = data.len() as usize * mem::size_of::<T>();
        if real_size != desc.size as usize {
            panic!(
                "Buffer::new_with_data: data bytes greater than  GPUMemory size {:?}",
                desc
            );
        }

        gl_bind_buffer(target, raw);
        gl_buffer_data(
            target,
            desc.size as es20d::GLsizeiptr,
            data.as_ptr() as *const es20d::GLvoid,
            usage,
        );
        gl_bind_buffer(target, 0);

        Buffer {
            desc: desc.clone(),
            raw: Some(raw),
        }
    }

    //todo: CPU data unpack to GPU,高版本或许可以使用mapbufferRange来实现高速资源拷贝
    pub fn write_sub_buffer_data<T>(&self, offset: u32, size: u32, data: &[T]) -> Option<&Buffer> {
        if self.desc.size == 0 {
            eprintln!(
                "Buffer::write_data: hasn't been allocate a GPUMemory {:?}",
                self
            );
            return None;
        }

        let mut real_size = size;
        if (offset + size) > self.desc.size {
            eprintln!(
                "Buffer::write_data: override GPUMemory {:?}, do clamp Operation",
                self.desc
            );
            real_size = &self.desc.size - offset;
        }

        let target = self.desc.target as u32;
        self.bind();
        gl_buffer_sub_data(
            target,
            offset as _,
            real_size as _,
            data.as_ptr() as *const es20d::GLvoid,
        );
        self.unbind();
        Some(&self)
    }

    pub fn bind(&self) -> &Self {
        let target = self.desc.target as u32;
        gl_bind_buffer(target, self.raw.unwrap());
        self
    }

    pub fn unbind(&self) -> &Self {
        let target = self.desc.target as u32;
        gl_bind_buffer(target, 0);
        self
    }
}

impl Drop for Buffer {
    fn drop(&mut self) {
        match self.raw {
            Some(id) => {
                gl_delete_buffers(&[id]);
                self.raw = None;
            }
            _ => {}
        }
    }
}
