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

    [[location(2)]]
    color: vec4<f32>;
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

[[block]]
struct BrushDetailBlock {
    [[location(0)]]
    highlight: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[group(1), binding(0)]]
var<uniform> transform: TransformBlock;

[[group(2), binding(0)]]
var<uniform> detail: BrushDetailBlock;

[[stage(vertex)]]
fn main(in: VertexIn) -> VertexOut {
    // The vertex normal needs to be rotated, but not moved. This is achieved by
    // creating a special matrix that has its last row set to identity.
    var nmat = transform.matrix;
    nmat[3] = vec4<f32>(0.0, 0.0, 0.0, 1.0);

    var world_position = camera.view * transform.matrix * vec4<f32>(in.position, 1.0);
    var world_normal = normalize((nmat * vec4<f32>(in.normal, 1.0)).xyz);

    var out: VertexOut;
    out.clip_position = camera.projection * world_position;
    out.normal = world_normal;
    out.texcoord = in.texcoord;
    out.color = detail.highlight;

    // WGPU works with a different texture coordinate system, so we need to flip
    // the coordinates vertically.
    out.texcoord.y = 1.0 - out.texcoord.y;

    return out;
}





struct FragmentOut {
    [[location(0)]]
    color: vec4<f32>;
};

[[group(3), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(3), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn main(in: VertexOut) -> FragmentOut {
    // var light_dir = normalize(vec3<f32>(0.1, 0.2, 0.3));
    // var diffuse = clamp(dot(light_dir, in.normal), 0.0, 0.7) + 0.3;
    
    var color = textureSample(t_diffuse, s_diffuse, in.texcoord) * in.color;
    var color_rgb = color.rgb;
    var color_a = color.a;

    var out: FragmentOut;
    out.color = vec4<f32>(color.rgb, color.a);
    return out;
}