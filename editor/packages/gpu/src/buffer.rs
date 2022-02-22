use std::{marker::PhantomData, mem::size_of};

use crate::Gpu;

use bytemuck::{cast_slice, Pod};
pub use wgpu::BufferUsages;
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferDescriptor,
};

pub struct Buffer<T> {
    pub(crate) buffer: wgpu::Buffer,
    len: usize,
    _t: PhantomData<T>,
}

impl Gpu {
    pub fn create_buffer<T>(&self, content: &[T], usage: BufferUsages) -> Buffer<T>
    where
        T: Pod,
    {
        Buffer {
            buffer: self.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: cast_slice(content),
                usage,
            }),
            len: content.len(),
            _t: PhantomData,
        }
    }

    pub fn create_buffer_uninit<T>(&self, len: usize, usage: BufferUsages) -> Buffer<T>
    where
        T: Pod,
    {
        Buffer {
            buffer: self.device.create_buffer(&BufferDescriptor {
                label: None,
                size: (size_of::<T>() * len) as u64,
                usage,
                mapped_at_creation: false,
            }),
            len,
            _t: PhantomData,
        }
    }

    pub fn write_buffer<T>(&self, buffer: &Buffer<T>, content: &[T])
    where
        T: Pod,
    {
        self.queue
            .write_buffer(&buffer.buffer, 0, cast_slice(content));
    }
}

impl<T> Buffer<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
