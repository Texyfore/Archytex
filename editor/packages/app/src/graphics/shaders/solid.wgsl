struct Attribs {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    normal: vec3<f32>;

    [[location(2)]]
    texcoord: vec2<f32>;

    [[location(3)]]
    tvertext: vec4<f32>;
};

struct Vertex {
    [[builtin(position)]]
    position: vec4<f32>;

    [[location(0)]]
    normal: vec3<f32>;

    [[location(1)]]
    texcoord: vec2<f32>;

    [[location(2)]]
    tvertext: vec4<f32>;

    [[location(3)]]
    camera_position: vec3<f32>;

    [[location(4)]]
    world_position: vec3<f32>;
};

struct Fragment {
    [[location(0)]]
    color: vec4<f32>;
};

struct Camera {
    world_to_clip: mat4x4<f32>;
    view_to_world: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: Camera;

[[stage(vertex)]]
fn vertex(attribs: Attribs) -> Vertex {
    var vertex: Vertex;
    
    vertex.position = camera.world_to_clip * vec4<f32>(attribs.position, 1.0);
    vertex.normal = attribs.normal;
    vertex.texcoord = attribs.texcoord;
    vertex.tvertext = attribs.tvertext;
    
    vertex.camera_position = (camera.view_to_world * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;
    vertex.world_position = attribs.position;

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

    var fragment: Fragment;
    fragment.color = vec4<f32>(color_rgb + vertex.tvertext.xyz * vertex.tvertext.w, color_a);
    return fragment;
}