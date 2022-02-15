struct Attribs {
    [[location(0)]]
    position: vec3<f32>;

    [[location(1)]]
    color: vec3<f32>;
};

struct Vertex {
    [[builtin(position)]]
    position: vec4<f32>;

    [[location(0)]]
    color: vec3<f32>;
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
    vertex.position.z = vertex.position.z - 0.001;
    vertex.color = attribs.color;

    return vertex;
}

[[stage(fragment)]]
fn fragment(vertex: Vertex) -> Fragment {
    var fragment: Fragment;
    fragment.color = vec4<f32>(vertex.color, 1.0);
    return fragment;
}