struct VertexIn {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    normal: vec3<f32>;

    [[location(2)]]
    texcoord: vec2<f32>;
};

struct VertexOut {
    [[builtin(position)]]
    clip_position: vec4<f32>;
    
    [[location(0)]]
    normal: vec3<f32>;

    [[location(1)]]
    texcoord: vec2<f32>;
};

[[block]]
struct CameraBlock {
    view: mat4x4<f32>;
    projection: mat4x4<f32>;
};

[[block]]
struct TransformBlock {
    matrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[group(1), binding(0)]]
var<uniform> transform: TransformBlock;

[[stage(vertex)]]
fn main(in: VertexIn) -> VertexOut {
    var mvp = camera.projection * camera.view * transform.matrix;

    var out: VertexOut;
    out.clip_position = mvp * vec4<f32>(in.position, 1.0);
    out.normal = (mvp * vec4<f32>(in.normal, 1.0)).xyz;
    out.texcoord = in.texcoord;
    return out;
}

struct FragmentOut {
    [[location(0)]]
    color: vec4<f32>;
};

[[stage(fragment)]]
fn main(in: VertexOut) -> FragmentOut {
    var out: FragmentOut;
    out.color = vec4<f32>(1.0);
    return out;
}