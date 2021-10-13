#version 300 es

layout(location = 0) in vec3 pos;
layout(location = 1) in vec4 color;

out vec4 vertColor;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
    gl_Position = projection * inverse(view) * model * vec4(pos, 1.0);
    vertColor = color;
}