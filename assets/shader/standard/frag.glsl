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

layout(location = 0) in vec2 inUv;
layout(location = 0) out vec4 outColor;

void main() {
    uint albedo_id = items[index].instanceData.Albedo;
    vec3 color = texture(texSampler[albedo_id], inUv.xy).xyz;
    outColor = vec4(color.xyz, 1.0);
}
