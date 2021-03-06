#version 150

in vec3 pos;
in vec3 normal;
in vec2 uv;

out vec3 v_normal;
out vec3 v_position;
out vec2 v_uv;

uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

void main() {
  mat4 modelview = view * model;

  v_uv = uv;
  v_normal = transpose(inverse(mat3(modelview))) * normal;

  gl_Position = perspective * modelview * vec4(pos, 1.0);
  v_position = gl_Position.xyz / gl_Position.w;
}
