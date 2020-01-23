#version 450

layout(location = 0) in vec2 position;

out gl_PerVertex {
    vec4 gl_Position;
};

layout(set = 0, binding = 0) uniform Tests {
    float test_float; // 4 bytes
};

//const vec2 positions[3] = vec2[3](
//vec2(0.0, -0.5),
//vec2(0.5, 0.5),
//vec2(-0.5, 0.5)
//);

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
