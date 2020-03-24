#version 140
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform mat4 rotate;
uniform vec4 diffuse;// マテリアルのdiffuseカラー
uniform vec3 wlightDir;// ワールド座標のディレクショナルライトの向き
uniform mat4 identity;
in vec3 position;// 頂点データ
in vec3 norm;// 法線データ
in vec2 uv;
uniform vec3 ambient;
out vec4 Color;// ピクセルシェーダに渡す色
out vec2 v_tex_coords;
out vec4 Ambient;
void main()
{
    mat4 MVP=identity;
    mat4 MIT =transpose(inverse(rotate*identity));
    gl_Position = rotate*identity * vec4(position, 1.0);// 頂点のmvp変換
    vec3 n = clamp(normalize(mat3(MIT) * norm), 0.0, 1.0);// 法線のm変換
    float nl = dot(n, normalize(-wlightDir));// 法線とライトの内積を算出
    nl=nl*0.5+0.5;
    vec3 c = diffuse.rgb * nl;// 最終色を算出
    c = clamp(c, 0.0, 1.0);// 0.0 ~ 1.0に色を収める
    Color = vec4(c, diffuse.a);
    v_tex_coords=vec2(uv[0], 1.0-uv[1]);
    Ambient=vec4(ambient, 1.0);
}
