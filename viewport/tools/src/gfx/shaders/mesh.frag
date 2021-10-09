#version 300 es
precision highp float;

const vec3 LIGHT_DIR = normalize(vec3(0.0, -0.25, -1.0));

in vec3 vertexNormal;
in vec2 vertexUv;
out vec4 fragColor;

uniform sampler2D tex;

void main() {
    vec4 color = texture(tex, vertexUv);

    float diffuse = max(dot(-LIGHT_DIR, vertexNormal), 0.1);
    vec3 rgb = diffuse * color.rgb;

    fragColor = vec4(rgb, 1.0);
}