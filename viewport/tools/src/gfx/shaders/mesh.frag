#version 300 es
precision highp float;

in vec3 vertexNormal;
in vec2 vertexUv;
out vec4 fragColor;

void main() {
    vec3 color = vec3(vertexUv, 1.0);
    
    vec3 lightDir = normalize(vec3(0.0, -0.25, -1.0));
    float diffuse = max(dot(-lightDir, vertexNormal), 0.1);

    fragColor = vec4(diffuse * color, 1.0);
}