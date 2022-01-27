struct VertexIn {
    [[location(0)]]
    position: vec3<f32>;
};

struct InstanceIn {
    [[location(1)]]
    mat0: vec4<f32>;

    [[location(2)]]
    mat1: vec4<f32>;

    [[location(3)]]
    mat2: vec4<f32>;

    [[location(4)]]
    mat3: vec4<f32>;

    [[location(5)]]
    color: vec4<f32>;
};

struct VertexOut {
    [[builtin(position)]]
    clip_position: vec4<f32>;

    [[location(0)]]
    color: vec4<f32>;
};

struct CameraBlock {
    world: mat4x4<f32>;
    clip: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[stage(vertex)]]
fn vs_main(in: VertexIn, instance: InstanceIn) -> VertexOut {
    var transform_matrix = mat4x4<f32>(
        instance.mat0,
        instance.mat1,
        instance.mat2,
        instance.mat3,
    );

    var world_origin = transform_matrix * vec4<f32>(0.0, 0.0, 0.0, 1.0);
    var camera_position = camera.world * vec4<f32>(0.0, 0.0, 0.0, 1.0);
    var scale = length(world_origin - camera_position) * 0.006;

    var out: VertexOut;
    out.clip_position = camera.clip * transform_matrix * vec4<f32>(in.position * scale, 1.0);
    out.color = instance.color;
    return out;
}





struct FragmentOut {
    [[location(0)]]
    color: vec4<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexOut) -> FragmentOut {
    var out: FragmentOut;
    out.color = vec4<f32>(in.color.xyz, 1.0);
    return out;
}