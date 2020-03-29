#version 330 core

uniform sampler2DShadow shadow_map;
uniform sampler2D tex;
uniform vec3 ambient_color;
uniform vec4 diffuse_color;
uniform vec3 specular_color;
uniform float specular_intensity;
uniform vec3 light_loc;
uniform vec4 model_color;

struct vData{
    vec4 v_normal;
    vec4 shadow_coord;
    vec4 model_normal;
    vec3 v_position;
    vec2 v_tex_coords;
};
in vData frag;
out vec4 color;
void main() {

    vec4 v_normal_f=frag.v_normal;
    vec4 shadow_coord_f=frag.shadow_coord;
    vec4 model_normal_f=frag.model_normal;
    vec3 v_position_f=frag.v_position;
    vec2 v_tex_coords_f=frag.v_tex_coords;
    float diffuse = max(dot(normalize(v_normal_f.xyz), normalize(light_loc)), 0.0);
    vec3 camera_dir = normalize(-v_position_f);
    vec3 half_direction = normalize(normalize(light_loc) + camera_dir);
    float specular = pow(max(dot(half_direction, normalize(v_normal_f.xyz)), 0.0), specular_intensity);
    vec4 color0=vec4(ambient_color + diffuse * diffuse_color.rgb + specular * specular_color, 1.0);
    vec3 light_color = vec3(1, 1, 1);float bias = 0.05;// Geometry does not require bias
    float lum = max(dot(normalize(model_normal_f.xyz), normalize(light_loc)), 0.0);
    float visibility = texture(shadow_map, vec3(shadow_coord_f.xy, (shadow_coord_f.z-bias)/shadow_coord_f.w));
    color = vec4(max(lum * visibility, 0.05) * model_color.rgb * light_color, 1.0)*texture(tex, v_tex_coords_f)*color0;
}
