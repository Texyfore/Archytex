struct VertexIn {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    normal: vec3<f32>;

    [[location(2)]]
    texcoord: vec2<f32>;

    [[location(3)]]
    color: vec4<f32>;
};

struct VertexOut {
    [[builtin(position)]]
    clip_position: vec4<f32>;

    [[location(0)]]
    world_position: vec3<f32>;
    
    [[location(1)]]
    normal: vec3<f32>;

    [[location(2)]]
    texcoord: vec2<f32>;

    [[location(3)]]
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
    out.world_position = in.position;
    out.normal = in.normal;
    out.texcoord = in.texcoord;
    out.color = in.color;

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
    var GRID_THICKNESS = 0.025;
    var GRID_LENGTH = 0.5;

    var color = textureSample(t_diffuse, s_diffuse, in.texcoord) * in.color;
    var color_rgb = color.rgb;
    var color_a = color.a;

    var plane = vec2<f32>(0.0);

    if (abs(in.normal.x) > abs(in.normal.y)) {
        if (abs(in.normal.x) > abs(in.normal.z)) {
            plane = in.world_position.yz;
        }
        else {
            plane = in.world_position.xy;
        }
    } else
    {
        if (abs(in.normal.y) > abs(in.normal.z)) {
            plane = in.world_position.xz;
        } else {
            plane = in.world_position.xy;
        }
    }

    var plane_mod = (plane % GRID_LENGTH) / GRID_LENGTH;
    var stripe_x = plane_mod.x < GRID_THICKNESS || plane_mod.x > (1.0-GRID_THICKNESS);
    var stripe_y = plane_mod.y < GRID_THICKNESS || plane_mod.y > (1.0-GRID_THICKNESS);

    if(stripe_x || stripe_y) {
        color_rgb = vec3<f32>(0.5);
    }

    var light_dir = normalize(vec3<f32>(0.1, 0.2, 0.3));
    var diffuse = clamp(dot(light_dir, in.normal), 0.0, 0.7) + 0.3;
    color_rgb = color_rgb * diffuse;

    var out: FragmentOut;
    out.color = vec4<f32>(color_rgb, color_a);
    return out;
}