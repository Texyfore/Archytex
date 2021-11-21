struct VertexIn {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    texcoord: vec2<f32>;

    [[location(2)]]
    color: vec4<f32>;
};

struct VertexOut {
    [[builtin(position)]]
    clip_position: vec4<f32>;

    [[location(0)]]
    texcoord: vec2<f32>;

    [[location(1)]]
    color: vec4<f32>;
};

[[block]]
struct CameraBlock {
    matrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[stage(vertex)]]
fn main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip_position = camera.matrix * vec4<f32>(in.position, 1.0);
    out.texcoord = in.texcoord;
    out.color = in.color;
    return out;
}





struct FragmentOut {
    [[location(0)]]
    color: vec4<f32>;
};

[[group(1), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(1), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn main(in: VertexOut) -> FragmentOut {
    var out: FragmentOut;
    out.color = textureSample(t_diffuse, s_diffuse, in.texcoord) * in.color;
    return out;
}