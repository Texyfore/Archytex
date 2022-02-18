struct Attribs {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    texcoord: vec2<f32>;
};

struct Vertex {
    [[builtin(position)]]
    position: vec4<f32>;

    [[location(0)]]
    normal: vec3<f32>;

    [[location(1)]]
    texcoord: vec2<f32>;

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
    var camera_position = (camera.view_to_world * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;

    var vertex: Vertex;
    vertex.position = camera.world_to_clip * vec4<f32>(attribs.position, 1.0);
    vertex.normal = vec3<f32>(0.0, 1.0, 0.0);
    vertex.texcoord = attribs.texcoord;
    vertex.camera_position = camera_position;
    vertex.world_position = attribs.position;
    return vertex;
}

[[group(1), binding(0)]]
var t_diffuse: texture_2d<f32>;

[[group(1), binding(1)]]
var s_diffuse: sampler;

[[stage(fragment)]]
fn fragment(vertex: Vertex) -> Fragment {
    var dist = distance(vertex.world_position.xz, vec2<f32>(0.0));
    var mixval = pow(clamp(dist / 200.0, 0.0, 1.0), 4.0);

    var color = textureSample(t_diffuse, s_diffuse, vertex.texcoord);
    var color_rgb = color.rgb;
    var color_a = color.a;
    color_rgb = mix(color_rgb, vec3<f32>(0.537, 0.847, 1.0), mixval);

    var fragment: Fragment;
    fragment.color = vec4<f32>(color_rgb, color_a);
    return fragment;
}