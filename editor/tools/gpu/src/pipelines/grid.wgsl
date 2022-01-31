struct VertexIn {
    [[location(0)]]
    position: vec3<f32>;
};

struct VertexOut {
    [[builtin(position)]]
    clip_position: vec4<f32>;

    [[location(0)]]
    world_position: vec3<f32>;

    [[location(1)]]
    camera_position: vec3<f32>;
};

struct CameraBlock {
    world: mat4x4<f32>;
    clip: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[stage(vertex)]]
fn vs_main(in: VertexIn) -> VertexOut {
    var camera_position = camera.world * vec4<f32>(0.0, 0.0, 0.0, 1.0);
    var world_position = vec3<f32>(camera_position.x, -0.001, camera_position.z) + in.position * 200.0;

    var out: VertexOut;
    out.clip_position = camera.clip * vec4<f32>(world_position, 1.0);
    out.world_position = world_position;
    out.camera_position = camera_position.xyz;
    return out;
}





struct FragmentOut {
    [[location(0)]]
    color: vec4<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexOut) -> FragmentOut {
    var out: FragmentOut;
    out.color = vec4<f32>(1.0);
    return out;
}