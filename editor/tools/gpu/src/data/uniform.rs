use bytemuck::{bytes_of, Pod, Zeroable};
use wgpu::{
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor,
    BindGroupLayoutEntry, BindingResource, BindingType, BufferBinding, BufferBindingType,
    BufferUsages, ShaderStages,
};

use crate::handle::GpuHandle;

use super::buffer::Buffer;

pub struct UniformLayout {
    pub(crate) inner: BindGroupLayout,
}

pub struct Uniform<T: Pod> {
    pub(crate) group: BindGroup,
    buffer: Buffer<T>,
}

impl GpuHandle {
    pub fn create_uniform_layout(&self) -> UniformLayout {
        UniformLayout {
            inner: self
                .device
                .create_bind_group_layout(&BindGroupLayoutDescriptor {
                    label: None,
                    entries: &[BindGroupLayoutEntry {
                        binding: 0,
                        visibility: ShaderStages::VERTEX,
                        ty: BindingType::Buffer {
                            ty: BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                }),
        }
    }

    pub fn create_uniform<T: Pod + Zeroable>(&self, layout: &UniformLayout) -> Uniform<T> {
        let buffer = self.create_buffer(
            &[T::zeroed()],
            BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        );
        let group = self.device.create_bind_group(&BindGroupDescriptor {
            label: None,
            layout: &layout.inner,
            entries: &[BindGroupEntry {
                binding: 0,
                resource: BindingResource::Buffer(BufferBinding {
                    buffer: &buffer.inner,
                    offset: 0,
                    size: None,
                }),
            }],
        });

        Uniform { group, buffer }
    }

    pub fn set_uniform<T: Pod>(&self, uniform: &Uniform<T>, content: &T) {
        self.queue
            .write_buffer(&uniform.buffer.inner, 0, bytes_of(content));
    }
}
