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
    
    [[location(1)]]
    normal: vec3<f32>;

    [[location(2)]]
    texcoord: vec2<f32>;
};

[[block]]
struct CameraBlock {
    matrix: mat4x4<f32>;
};

[[block]]
struct TransformBlock {
    matrix: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[group(2), binding(0)]]
var<uniform> transform: CameraBlock;

[[stage(vertex)]]
fn main(in: VertexIn) -> VertexOut {
    var world_position = transform.matrix * vec4<f32>(in.position, 1.0);
    var world_normal = transform.matrix * vec4<f32>(in.position + in.normal, 1.0);

    var out: VertexOut;
    out.clip_position = camera.matrix * world_position;
    out.normal = normalize((world_normal - world_position).xyz);
    out.texcoord = in.texcoord;

    // WGPU works with a different texture coordinate system, so we need to flip
    // the coordinates vertically.
    out.texcoord.y = 1.0 - out.texcoord.y;

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
    var color = textureSample(t_diffuse, s_diffuse, in.texcoord) * in.color;
    var color_rgb = color.rgb;
    var color_a = color.a;

    var light_dir = normalize(vec3<f32>(0.1, 0.2, 0.3));
    var diffuse = clamp(dot(light_dir, in.normal), 0.0, 0.7) + 0.3;
    color_rgb = color_rgb * diffuse;

    var out: FragmentOut;
    out.color = vec4<f32>(color_rgb, color_a);
    return out;
}