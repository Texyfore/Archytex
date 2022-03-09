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

[[group(1), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(1), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fragment(vertex: Vertex) -> Fragment {
    var color = textureSample(t_diffuse, s_diffuse, vertex.texcoord);
    var color_rgb = color.rgb;
    var color_a = color.a;

    var light_dir = normalize(vertex.camera_position - vertex.world_position);
    var diffuse = (max(dot(light_dir, vertex.normal), 0.0) + 0.8) * 0.4;
    color_rgb = color_rgb * diffuse;

    // Grid
    {
        var cam_to_vert = normalize(vertex.camera_position - vertex.world_position);
        var dist = distance(vertex.camera_position, vertex.world_position);

        var len = f32(vertex.grid_len) / 128.0;
        var x = (((vertex.texcoord.x * 4.0) % len + len) % len) / len;
        var y = (((vertex.texcoord.y * 4.0) % len + len) % len) / len;

        var fade_scale = len * 40.0;
        var fade = (fade_scale - clamp(dist, 0.0, fade_scale)) / fade_scale;
        var flatness = dot(cam_to_vert, vertex.normal);

        var thbase = dist / clamp(flatness * 2.0, 1.0, 2.0) * 0.6;

        var th = thbase / len * 0.01;
        var ith = 1.0 - th;

        if (x < th || x > ith || y < th || y > ith) {
            color_rgb = mix(color_rgb, vec3<f32>(1.0), fade * flatness);
        }
    }

    var fragment: Fragment;
    fragment.color = vec4<f32>(color_rgb + vertex.tint.rgb * vertex.tint.a, color_a);
    return fragment;
}