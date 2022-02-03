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

    [[location(3)]]
    tint: vec4<f32>;

    [[location(4)]]
    world_position: vec3<f32>;

    [[location(5)]]
    camera_position: vec3<f32>;
};

struct CameraBlock {
    world: mat4x4<f32>;
    clip: mat4x4<f32>;
};

struct TransformBlock {
    matrix: mat4x4<f32>;
};

struct TintBlock {
    tint: vec4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraBlock;

[[group(1), binding(0)]]
var<uniform> transform: TransformBlock;

[[group(2), binding(0)]]
var<uniform> tint: TintBlock;

[[stage(vertex)]]
fn vs_main(in: VertexIn) -> VertexOut {
    var world_position = transform.matrix * vec4<f32>(in.position, 1.0);
    var world_normal = transform.matrix * vec4<f32>(in.position + in.normal, 1.0);

    var out: VertexOut;

    out.clip_position = camera.clip * world_position;
    out.normal = normalize((world_normal - world_position).xyz);
    out.texcoord = in.texcoord;
    out.tint = tint.tint;

    out.world_position = world_position.xyz;
    out.camera_position = (camera.world * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;

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
fn fs_main(in: VertexOut) -> FragmentOut {
    var color = textureSample(t_diffuse, s_diffuse, in.texcoord);
    var color_rgb = color.rgb;
    var color_a = color.a;

    var light_dir = normalize(in.camera_position - in.world_position);
    var diffuse = (max(dot(light_dir, in.normal), 0.0) + 0.8) * 0.4;
    color_rgb = color_rgb * diffuse;

    var out: FragmentOut;
    out.color = vec4<f32>(color_rgb + in.tint.xyz * in.tint.w, color_a);
    
    return out;
}