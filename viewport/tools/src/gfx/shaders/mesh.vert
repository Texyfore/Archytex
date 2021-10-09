#version 300 es

layout(location = 0) in vec3 pos;
layout(location = 1) in vec3 normal;
layout(location = 2) in vec2 uv;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 vertexNormal;
out vec2 vertexUv;

void main() {
    mat4 normalTransform = model;
    normalTransform[3] = vec4(0.0, 0.0, 0.0, 1.0);

    gl_Position = projection * inverse(view) * model * vec4(pos, 1.0);
    vertexNormal = (normalTransform * vec4(normal, 1.0)).xyz;
    vertexUv = uv;
}