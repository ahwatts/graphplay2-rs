// -*- mode: glsl; c-basic-offset: 4; -*-

#version 410 core

const int MAX_LIGHTS = 10;
struct LightProperties {
    bool enabled;
    vec3 position;
    vec4 color;
    float specular_exp;
};

in vec3 position;
in vec3 normal;
in vec4 color;

uniform mat4x4 model;
uniform mat3x3 model_inv_trans_3;
layout (shared) uniform view_and_projection {
    mat4x4 view;
    mat4x4 view_inv;
    mat4x4 projection;
};
layout (shared) uniform light_list {
    LightProperties lights[MAX_LIGHTS];
};

out vec3 v_normal;
out vec4 v_color;
out vec3 v_eye_dir;
out vec3 v_light_dir[MAX_LIGHTS];
out vec3 v_light_reflect_dir[MAX_LIGHTS];

void main(void) {
    vec4 wld_vert_position4 = model * vec4(position, 1.0);
    vec3 wld_vert_position = wld_vert_position4.xyz / wld_vert_position4.w;

    vec4 wld_eye_position4 = view_inv * vec4(0.0, 0.0, 0.0, 1.0);
    vec3 wld_eye_position = wld_eye_position4.xyz / wld_eye_position4.w;

    vec3 wld_vert_normal = normalize(model_inv_trans_3 * normal);

    vec3 wld_vert_eye_dir = normalize(wld_eye_position - wld_vert_position);

    gl_Position = projection * view * wld_vert_position4;
    v_color = color;
    v_eye_dir = wld_vert_eye_dir;
    v_normal = wld_vert_normal;
    for (int i = 0; i < MAX_LIGHTS; ++i) {
        if (lights[i].enabled) {
            v_light_dir[i] = normalize(lights[i].position - wld_vert_position);
            v_light_reflect_dir[i] = normalize(reflect(-1 * v_light_dir[i], wld_vert_normal));
        }
    }
}
