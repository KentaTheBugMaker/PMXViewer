#version 330 core
uniform mat4 mvp;
uniform mat4 depth_bias_mvp;
uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform vec4 model_color;
in vec4 position;
in vec4 normal;
in vec2 uv;
out vec4 v_normal;
out vec3 v_position;
out vec2 v_tex_coords;
out vec4 shadow_coord;
out vec4 model_normal;
void main() {

   vec4 position0=vec4(-position.x, position.yzw);
    position0.xyz*=position.w;
    gl_Position =  mvp*position0;
    model_normal = model_matrix *normal;
    v_normal = transpose(inverse(model_matrix*view_matrix)) *normal;
    shadow_coord = depth_bias_mvp *position0;
    v_tex_coords=vec2(uv[0], 1.0-uv[1]);
    v_position=position0.xyz;

    }
