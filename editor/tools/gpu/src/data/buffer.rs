use std::marker::PhantomData;

use bytemuck::{cast_slice, Pod};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BufferUsages,
};

use crate::handle::GpuHandle;

pub struct Buffer<T: Pod> {
    pub(crate) inner: wgpu::Buffer,
    len: usize,
    _t: PhantomData<T>,
}

impl<T: Pod> Buffer<T> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl GpuHandle {
    pub fn create_buffer<T: Pod>(&self, content: &[T], usage: BufferUsages) -> Buffer<T> {
        let buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: cast_slice(content),
            usage,
        });

        Buffer {
            inner: buffer,
            len: content.len(),
            _t: PhantomData,
        }
    }
}
