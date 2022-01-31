struct VertexIn {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    color: vec3<f32>;
};

struct VertexOut {
    [[builtin(position)]]
    clip_position: vec4<f32>;

    [[location(0)]]
    color: vec3<f32>;

    [[location(1)]]
    world_position: vec3<f32>;

    [[location(2)]]
    camera_position: vec3<f32>;
};

struct CameraBlock {
    world: mat4x4<f32>;
    clip: mat4x4<f32>;
};

struct InfoBlock {
    step: i32;
    snap_flags: i32;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[group(1), binding(0)]]
var<uniform> info: InfoBlock;

[[stage(vertex)]]
fn vs_main(in: VertexIn) -> VertexOut {
    var camera_position = camera.world * vec4<f32>(0.0, 0.0, 0.0, 1.0);
    
    var step = f32(info.step) / 100.0;
    var x = floor(camera_position.x / step) * step;
    var z = floor(camera_position.z / step) * step;

    var world_position = vec3<f32>(x, 0.0, z) + in.position;

    var out: VertexOut;
    out.color = in.color;
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
    var cam_pos = vec3<f32>(in.camera_position.x, 0.0, in.camera_position.z);

    var full = vec3<f32>(1.0);
    var clear = vec3<f32>(0.1);
    var dist = clamp(distance(in.world_position, in.camera_position) / 30.0, 0.0, 1.0);
    var color = mix(full, clear, dist);

    var out: FragmentOut;
    out.color = vec4<f32>(color, 1.0);
    return out;
}