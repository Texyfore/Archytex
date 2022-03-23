struct Attribs {
    [[location(0)]]
    position: vec3<f32>;
};

struct Instance {
    [[location(1)]]
    mat0: vec4<f32>;

    [[location(2)]]
    mat1: vec4<f32>;

    [[location(3)]]
    mat2: vec4<f32>;

    [[location(4)]]
    mat3: vec4<f32>;

    [[location(5)]]
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
fn vertex(attribs: Attribs, instance: Instance) -> Vertex {
    var transform = mat4x4<f32>(
        instance.mat0,
        instance.mat1,
        instance.mat2,
        instance.mat3,
    );

    var camera_position = (camera.view_to_world * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;
    var origin = (transform * vec4<f32>(0.0, 0.0, 0.0, 1.0)).xyz;
    var scale = distance(camera_position, origin) * 0.01;

    var vertex: Vertex;
    vertex.position = camera.world_to_clip * transform * vec4<f32>(attribs.position * scale, 1.0);
    vertex.color = pow(instance.color, vec3<f32>(2.2));
    return vertex;
}

[[stage(fragment)]]
fn fragment(vertex: Vertex) -> Fragment {
    var fragment: Fragment;
    fragment.color = vec4<f32>(vertex.color, 1.0);
    return fragment;
}