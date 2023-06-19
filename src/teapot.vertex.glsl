#version 150

in vec3 position;
in vec3 normal;

out vec3 v_normal;
out vec3 v_position;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;
uniform vec3 offset;

// We need to rotate first and move the object afterwards.
void main() {
    mat4 modelview = view * model;
    v_normal = transpose(inverse(mat3(modelview))) * normal;
    vec3 op = offset + position;
    vec4 rot_pos = perspective * modelview * vec4(position, 1.0); // It seems that at this point something is going wrong.
    gl_Position = rot_pos + vec4(offset, 1.0);
    v_position = gl_Position.xyz / gl_Position.w;
}
