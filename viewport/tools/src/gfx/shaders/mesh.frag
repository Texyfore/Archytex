#version 300 es
precision highp float;

in vec3 vertexNormal;
in vec2 vertexUv;
out vec4 fragColor;

void main() {
    fragColor = vec4(vertexUv, 0.0, 1.0);
}