struct Attribs {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    normal: vec3<f32>;

    [[location(2)]]
    texcoord: vec2<f32>;

    [[location(3)]]
    tint: vec4<f32>;
};

struct Vertex {
    [[builtin(position)]]
    position: vec4<f32>;

    [[location(0)]]
    normal: vec3<f32>;

    [[location(1)]]
    texcoord: vec2<f32>;

    [[location(2)]]
    tint: vec4<f32>;

    [[location(3)]]
    camera_position: vec3<f32>;

    [[location(4)]]
    world_position: vec3<f32>;

    [[location(5)]]
    grid_len: i32;
};

struct Fragment {
    [[location(0)]]
    color: vec4<f32>;
};

struct Camera {
    world_to_clip: mat4x4<f32>;
    view_to_world: mat4x4<f32>;
};

struct GridParams {
    len: i32;
    pad1: i32;
    pad2: i32;
    pad3: i32;
};

[[group(0), binding(0)]]
var<uniform> camera: Camera;

[[group(2), binding(0)]]
var<uniform> grid: GridParams;


[[stage(vertex)]]
fn vertex(attribs: Attribs) -> Vertex {
    var vertex: Vertex;
    
    vertex.position = camera.world_to_clip * vec4<f32>(attribs.position, 1.0);
    vertex.normal = attribs.normal;
    vertex.texcoord = attribs.texcoord;
    vertex.tint = attribs.tint;
    
    vertex.camera_position = (camera.view_to_world * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;
    vertex.world_position = attribs.position;
    vertex.grid_len = grid.len;

    return vertex;
}

fn mkgrid(
    pos: vec3<f32>,
    nor: vec3<f32>,
    uv: vec2<f32>,
    cam: vec3<f32>,
    glen: i32,
) -> f32 {
    var cam_to_vert = normalize(cam - pos);
    var dist = distance(cam, pos);

    var len = f32(glen) / 128.0;
    var x = ((uv.x % len + len) % len) / len;
    var y = ((uv.y % len + len) % len) / len;

    var fade_scale = len * 60.0;
    var fade = (fade_scale - clamp(dist, 0.0, fade_scale)) / fade_scale;
    var flatness = pow(dot(cam_to_vert, nor), 1.5);

    var thbase = dist * 0.6;
    var th = thbase / len * 0.005;
    var ith = 1.0 - th;

    if (x < th || x > ith || y < th || y > ith) {
        return fade * flatness;
    }else{
        return 0.0;
    }
}

[[group(1), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(1), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fragment(vertex: Vertex) -> Fragment {
    var color = textureSample(t_diffuse, s_diffuse, vertex.texcoord);
    var color_rgb = color.rgb;
    var color_a = color.a;

    var g = mkgrid(
        vertex.world_position,
        vertex.normal,
        vertex.texcoord,
        vertex.camera_position,
        vertex.grid_len
    );

    color_rgb = color_rgb + vec3<f32>(g * 0.5);

    var light_dir = normalize(vertex.camera_position - vertex.world_position);
    var diffuse = (max(dot(light_dir, vertex.normal), 0.0) + 0.8) * 0.4;
    color_rgb = color_rgb * diffuse;

    var dist = distance(vertex.world_position, vertex.camera_position);
    var mixval = pow(clamp(dist / 200.0, 0.0, 1.0), 4.0);

    var final_rgb = color_rgb + vertex.tint.rgb * vertex.tint.a;
    final_rgb = mix(final_rgb, vec3<f32>(0.537, 0.847, 1.0), mixval);

    var fragment: Fragment;
    fragment.color = vec4<f32>(final_rgb, color_a);
    return fragment;
}