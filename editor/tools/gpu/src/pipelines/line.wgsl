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
};

struct CameraBlock {
    matrix: mat4x4<f32>;
};

struct TransformBlock {
    matrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[group(1), binding(0)]]
var<uniform> transform: TransformBlock;

[[stage(vertex)]]
fn vs_main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip_position = camera.matrix * transform.matrix * vec4<f32>(in.position, 1.0);
    out.color = in.color;

    return out;
}





struct FragmentOut {
    [[location(0)]]
    color: vec4<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexOut) -> FragmentOut {
    var out: FragmentOut;
    out.color = vec4<f32>(in.color, 1.0);
    return out;
}