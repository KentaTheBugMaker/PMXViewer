#version 330 core
uniform mat4 mvp;
uniform mat4 depth_bias_mvp;
uniform mat4 model_matrix;
uniform mat4 view_matrix;
uniform vec4 model_color;
in vec4 position;
in vec4 normal;
in vec2 uv;
struct vData{
 vec4 v_normal;
 vec4 shadow_coord;
 vec4 model_normal;
 vec3 v_position;
 vec2 v_tex_coords;
};
out vData vertex;
void main() {

 vec4 position0=vec4(-position.x, position.yzw);
 position0.xyz*=position.w;
 gl_Position =  mvp*position0;
 vertex.model_normal = model_matrix *normal;
 vertex.v_normal = transpose(inverse(model_matrix*view_matrix)) *normal;
 vertex.shadow_coord = depth_bias_mvp *position0;
 vertex.v_tex_coords=vec2(uv[0], 1.0-uv[1]);
 vertex.v_position=position0.xyz;

}
