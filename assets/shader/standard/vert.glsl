#version 450
#extension GL_ARB_separate_shader_objects : enable

struct InstanceData {
    uint Albedo;
    uint _u0;
    uint _u1;
    uint _u2;
    vec4 _u3;
    vec4 _u4;
    vec4 _u5;
    mat4 pvm;
};

layout (set=0, binding=0) uniform sampler2D texSampler[1024];
layout (set=1, binding=0) readonly buffer InstanceDataBlock {
    InstanceData instanceData;
} items[1024];

layout(push_constant) uniform Index {
    layout(offset = 0) uint index;
};

layout(location = 0) in vec3 inPosition;
layout(location = 1) in vec2 inUv;

layout(location = 0) out vec2 outUv;

void main() {
    vec4 position = vec4(inPosition.xyz, 1.0);

    mat4 pvm = items[index].instanceData.pvm;
    gl_Position = pvm * position;
    outUv = inUv;
}
