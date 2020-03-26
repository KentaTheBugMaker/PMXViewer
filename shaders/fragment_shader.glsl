#version 330 core
uniform sampler2DShadow shadow_map;
uniform sampler2D tex;
uniform vec3 ambient_color;
uniform vec3 specular_color;
uniform vec4 diffuse_color;
uniform vec3 light_loc;
uniform vec4 model_color;
uniform float specular_intensity;
uniform vec2 resolution;
in vec4 shadow_coord;
in vec2 v_tex_coords;
in vec4 model_normal;
in vec4 v_normal;
in vec3 v_position;
out vec4 color;
const float PI = 3.14159265;
const float angle = 60.0;
const float fov = angle * 0.5 * PI / 180.0;
const vec3 lightDir = vec3(-0.577, 0.577, 0.577);
vec4 saturate(vec4 x){
    return clamp(x, 0.0, 1.0);
}
vec3 saturate(vec3 x){
    return clamp(x, 0.0, 1.0);
}
vec4 smoothstepv(vec4 x, vec4 y, float a){
    return vec4(smoothstep(x.x, y.x, a), smoothstep(x.y, y.y, a), smoothstep(x.z, y.z, a), 1.0);
}
const float sphereSize=3.0;
float distFunc(vec3 pos){

    return length(pos) - sphereSize;
    // return length(pos-gl_FragCoord);
}

struct Ray{
    vec3 pos;
    vec3 dir;
};

vec3 getNormal(vec3 pos){
    return normalize(v_normal.xyz);
}

void main(void){
    /*
     vec3 light_color =vec3(1.0,1.0,1.0);
     vec3 camera_dir = normalize(-v_position);
     vec3 half_direction = normalize(normalize(light_loc) + camera_dir);
     float specular = pow(max(dot(half_direction,normalize(v_normal).xyz),0.0),specular_intensity);
     float bias = 0.01; // Geometry does not require bias
     float lum = max(dot(normalize(model_normal.xyz), normalize(light_loc)), 0.0);//diff
     float visibility = texture(shadow_map, vec3(shadow_coord.xy, (shadow_coord.z-bias)/shadow_coord.w));
     vec4 color0 =vec4(ambient_color +max(lum * diffuse_color.rgb,0.05) + specular * specular_color, 1.0);
     vec4 color1=vec4(lum * visibility * model_color.rgb * light_color, 1.0)*1.5;
     */
    vec4 color2=texture(tex, v_tex_coords);
    //vec4 color4 =saturate(smoothstepv(color0,color1,0.4))*color2;

    vec2 p = (gl_FragCoord.xy * 2.0 - resolution) / min(resolution.x, resolution.y);

    // camera
    vec3 cPos = vec3(0.0, 0.0, 3.0);// カメラの位置
    vec3 cDir = vec3(0.0, 0.0, -1.0);// カメラの向き(視線)
    vec3 cUp  = vec3(0.0, 1.0, 0.0);// カメラの上方向
    vec3 cSide = cross(cDir, cUp);// 外積を使って横方向を算出
    float targetDepth = 0.1;// フォーカスする深度
    vec3 ray = normalize(cSide * p.x + cUp * p.y + cDir * targetDepth);
    // marching loop
    float distance = 0.0;// レイとオブジェクト間の最短距離
    float rLen = 0.0;// レイに継ぎ足す長さ
    vec3  rPos = cPos;// レイの先端位置
    for (int i = 0; i < 256; i++){
        distance = distFunc(rPos);
        rLen += distance;
        rPos = cPos + ray * rLen;
    }
    // hit check
    if (abs(distance) < 0.001){

        vec3 normal = getNormal(rPos);
        float diff = clamp(dot(light_loc, normal), 0.1, 1.0);
        color= vec4(vec3(diff), 1.0)*color2;
    }

}