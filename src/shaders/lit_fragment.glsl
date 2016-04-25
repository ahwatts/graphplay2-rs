// -*- mode: glsl; c-basic-offset: 4; -*-

#version 410 core

const int MAX_LIGHTS = 10;
struct LightProperties {
    bool enabled;
    vec3 position;
    vec4 color;
    float specular_exp;
};

in vec3 v_normal;
in vec4 v_color;
in vec3 v_eye_dir;
in vec3 v_light_dir[MAX_LIGHTS];
in vec3 v_light_reflect_dir[MAX_LIGHTS];

layout (shared) uniform light_list {
    LightProperties lights[MAX_LIGHTS];
};

out vec4 frag_color;

void main(void) {
    frag_color = vec4(0.0, 0.0, 0.0, 1.0);

    for (int i = 0; i < MAX_LIGHTS; ++i) {
        if (lights[i].enabled) {
            vec3 color_combination = lights[i].color.rgb * v_color.rgb;

            vec3 ambient_color = 0.1 * color_combination;

            float diffuse_coeff = 0.7 * max(0.0, dot(v_normal, v_light_dir[i]));
            vec3 diffuse_color = diffuse_coeff * color_combination;

            vec3 specular_color = vec3(0.0, 0.0, 0.0);
            if (dot(v_normal, v_light_dir[i]) >= 0.0) {
                float spec_coeff = 0.7 * pow(max(0.0, dot(v_light_reflect_dir[i], v_eye_dir)), lights[i].specular_exp);
                specular_color = spec_coeff * color_combination;
            }

            frag_color = clamp(frag_color + vec4(clamp(ambient_color + diffuse_color + specular_color, 0.0, 1.0), v_color.a), 0.0, 1.0);
        }
    }
}
