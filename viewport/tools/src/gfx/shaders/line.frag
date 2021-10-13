#version 300 es
precision highp float;

in vec4 vertColor;
out vec4 fragColor;

void main() {
    fragColor = vertColor;
}