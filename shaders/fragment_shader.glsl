            #version 330 core
            
            uniform sampler2DShadow shadow_map;
            uniform sampler2D tex;
            uniform vec3 ambient_color;
            uniform vec4 diffuse_color;
            uniform vec3 specular_color;
            uniform float specular_intensity;
            uniform vec3 light_loc;
            uniform vec4 model_color;
            in vec3 v_position;
            in vec2 v_tex_coords;
            in vec4 shadow_coord;
            in vec4 model_normal;
            in vec4 v_normal;
            
            out vec4 color;
            void main() {
            float diffuse = max(dot(normalize(v_normal.xyz), normalize(light_loc)), 0.0);
            vec3 camera_dir = normalize(-v_position);
            vec3 half_direction = normalize(normalize(light_loc) + camera_dir);
            float specular = pow(max(dot(half_direction, normalize(v_normal.xyz)), 0.0), specular_intensity);
            vec4 color0=vec4(ambient_color + diffuse * diffuse_color.rgb + specular * specular_color, 1.0);
                vec3 light_color = vec3(1,1,1);
            	float bias = 0.05; // Geometry does not require bias
            	float lum = max(dot(normalize(model_normal.xyz), normalize(light_loc)), 0.0);
            	float visibility = texture(shadow_map, vec3(shadow_coord.xy, (shadow_coord.z-bias)/shadow_coord.w));
            
            color = vec4(max(lum * visibility, 0.05) * model_color.rgb * light_color, 1.0)*texture(tex,v_tex_coords)*color0;

            }
