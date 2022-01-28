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
    var clear = vec3<f32>(0.1, 0.1, 0.1);
    var tint = vec3<f32>(0.4, 0.6, 1.0);
    var max_dist = 24.0;
    var uv = abs(in.world_position.xz);

    var dist = distance(in.camera_position, in.world_position);
    var cam_dist = clamp(in.camera_position.y / 8.0, 0.0, 1.0);
    var fade = clamp(1.0 - dist / (max_dist * cam_dist), 0.0, 1.0) * cam_dist;

    var color = clear;

    var distfield_x = min(uv.x % 1.0, 1.0 - (uv.x % 1.0));    
    var distfield_y = min(uv.y % 1.0, 1.0 - (uv.y % 1.0));
    var strength_x = pow(1.0 - distfield_x, 32.0);
    var strength_y = pow(1.0 - distfield_y, 32.0);
    var strength = clamp(strength_x + strength_y, 0.0, 1.0);

    color = mix(clear, tint, strength);

    var out: FragmentOut;
    out.color = vec4<f32>(mix(clear, color, fade), 1.0);
    return out;
}