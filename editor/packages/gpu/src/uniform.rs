use std::marker::PhantomData;

use bytemuck::{bytes_of, Pod};
use wgpu::{
    util::{BufferInitDescriptor, DeviceExt},
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindingResource, Buffer, BufferBinding,
    BufferUsages,
};

use crate::{Gpu, Res};

pub struct Uniform<T> {
    pub(crate) group: BindGroup,
    buffer: Buffer,
    _t: PhantomData<T>,
}

impl Gpu {
    pub fn create_uniform<T>(&self, content: &T) -> Res<Uniform<T>>
    where
        T: Pod,
    {
        let buffer = self.device.create_buffer_init(&BufferInitDescriptor {
            label: None,
            contents: bytes_of(content),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });

        let group = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &self.uniform_layout,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        Res::new(Uniform {
            group,
            buffer,
            _t: PhantomData,
        })
    }

    pub fn set_uniform<T>(&self, uniform: &Uniform<T>, content: &T)
    where
        T: Pod,
    {
        self.queue
            .write_buffer(&uniform.buffer, 0, bytes_of(content));
    }
}
