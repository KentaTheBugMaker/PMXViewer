#version 330 core
layout(triangles) in;
layout(line_strip, max_vertices=4) out;
struct vData{
    vec4 v_normal;
    vec4 shadow_coord;
    vec4 model_normal;
    vec3 v_position;
    vec2 v_tex_coords;
};
//From Vertex Shader
in vData vertex[];
//To Fragment Shader
out vData frag;
void main() {
    int i;
    for (i=0;i<gl_in.length();i++){
        frag.v_normal=vertex[i].v_normal;
        frag.model_normal=vertex[i].model_normal;
        frag.shadow_coord=vertex[i].shadow_coord;
        frag.v_position=vertex[i].v_position;
        frag.v_tex_coords=vertex[i].v_tex_coords;
        gl_Position=gl_in[i].gl_Position;
        EmitVertex();
    }
    gl_Position=gl_in[0].gl_Position;
    EmitVertex();
    EndPrimitive();
}
