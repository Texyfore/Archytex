use std::marker::PhantomData;

use crate::Gpu;

use bytemuck::{cast_slice, Pod};
use wgpu::util::{BufferInitDescriptor, DeviceExt};
pub use wgpu::BufferUsages;

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
}

impl<T> Buffer<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}
