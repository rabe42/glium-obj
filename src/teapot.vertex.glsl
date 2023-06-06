#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform vec3 offset;

void main() {
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    vec3 op = offset + position;
    gl_Position = perspective * modelview * vec4(op, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}
