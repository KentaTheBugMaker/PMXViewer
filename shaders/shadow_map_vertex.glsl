#version 330 core
in vec4 position;
uniform mat4 depth_mvp;
void main() {
    gl_Position = depth_mvp *vec4(-position[0], position[1], position[2], position[3]);
}