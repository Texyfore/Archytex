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
struct CameraGroup {
    view: mat4x4<f32>;
    projection: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraGroup;

[[stage(vertex)]]
fn main(in: VertexIn) -> VertexOut {
    var out: VertexOut;
    out.clip_position = camera.projection * camera.view * vec4<f32>(in.position, 1.0);
    out.normal = (camera.projection * camera.view * vec4<f32>(in.normal, 1.0)).xyz;
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