use std::{sync::Arc, ffi::CString};
use ash::{vk, Device, version::DeviceV1_0};
use crate::Error;
use super::*;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum ShaderStageType {
    Vertex,
    Geometry,
    TessellationControl,
    TessellationEvaluation,
    Fragment,
    Compute,
}

impl Into<vk::ShaderStageFlags> for ShaderStageType {
    fn into(self) -> vk::ShaderStageFlags {
        match self {
            ShaderStageType::Vertex => vk::ShaderStageFlags::VERTEX,
            ShaderStageType::Geometry => vk::ShaderStageFlags::GEOMETRY,
            ShaderStageType::TessellationControl => vk::ShaderStageFlags::TESSELLATION_CONTROL,
            ShaderStageType::TessellationEvaluation => vk::ShaderStageFlags::TESSELLATION_EVALUATION,
            ShaderStageType::Fragment => vk::ShaderStageFlags::FRAGMENT,
            ShaderStageType::Compute => vk::ShaderStageFlags::COMPUTE,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum PipelineBindPoint {
    Graphics,
    Compute,
}

impl Into<vk::PipelineBindPoint> for PipelineBindPoint {
    fn into(self) -> vk::PipelineBindPoint {
        match self {
            PipelineBindPoint::Graphics => vk::PipelineBindPoint::GRAPHICS,
            PipelineBindPoint::Compute => vk::PipelineBindPoint::COMPUTE,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum DescriptorType {
    Sampler,
    CombinedImageSampler,
    SampledImage,
    StorageImage,
    UniformTexelBuffer,
    StorageTexelBuffer,
    UniformBuffer,
    StorageBuffer,
    UniformBufferDynamic,
    StorageBufferDynamic,
    InputAttachment,
    AccelerationStructureNV,
}

impl Into<vk::DescriptorType> for DescriptorType {
    fn into(self) -> vk::DescriptorType {
        match self {
            DescriptorType::Sampler => vk::DescriptorType::SAMPLER,
            DescriptorType::CombinedImageSampler => vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            DescriptorType::SampledImage => vk::DescriptorType::SAMPLED_IMAGE,
            DescriptorType::StorageImage => vk::DescriptorType::STORAGE_IMAGE,
            DescriptorType::UniformTexelBuffer => vk::DescriptorType::UNIFORM_TEXEL_BUFFER,
            DescriptorType::StorageTexelBuffer => vk::DescriptorType::STORAGE_TEXEL_BUFFER,
            DescriptorType::UniformBuffer => vk::DescriptorType::UNIFORM_BUFFER,
            DescriptorType::StorageBuffer => vk::DescriptorType::STORAGE_BUFFER,
            DescriptorType::UniformBufferDynamic => vk::DescriptorType::UNIFORM_BUFFER_DYNAMIC,
            DescriptorType::StorageBufferDynamic => vk::DescriptorType::STORAGE_BUFFER_DYNAMIC,
            DescriptorType::InputAttachment => vk::DescriptorType::INPUT_ATTACHMENT,
            DescriptorType::AccelerationStructureNV => vk::DescriptorType::ACCELERATION_STRUCTURE_NV,
        }
    }
}

#[derive(Debug)]
pub struct VertexAttribute {
    location: u32,
    stride: u32,
    format: vk::Format,
}

#[derive(Debug)]
pub struct DescriptorSetLayoutBinding {
    binding: u32,
    descriptor_type: DescriptorType,
    count: u32,
    stage_flags: vk::ShaderStageFlags,
}

#[derive(Debug)]
pub struct DescriptorSetLayout {
    pub set: u32,
    pub bindings: Vec<DescriptorSetLayoutBinding>,
}

#[derive(Debug)]
pub struct ShaderStage {
    spirv: Vec<u32>,
    stage: ShaderStageType,
    entry: CString,
}

#[derive(Debug)]
pub struct PushConstant {
    offset: u32,
    size: u32,
    stage_flags: vk::ShaderStageFlags,
}

#[derive(Debug)]
pub struct ShaderDescription {
    pipeline_bind_point: PipelineBindPoint,
    stages: Vec<ShaderStage>,
    attributes: Vec<VertexAttribute>,
    descriptor_set_layouts: Vec<DescriptorSetLayout>,
    push_constants: Vec<PushConstant>,
    bind_point: PipelineBindPoint,
}

unsafe fn create_shader_module(device: &Arc<ash::Device>, shader_stage: &ShaderStage) -> Result<vk::ShaderModule, Error> {
    let shader_create_info = vk::ShaderModuleCreateInfo::builder().code(&shader_stage.spirv);
    let shader_module = device.create_shader_module(&shader_create_info, None)?;

    Ok(shader_module)
}

pub struct Pipeline {
    inner: vk::Pipeline,
    device: Arc<Device>,
    pipeline_layout: vk::PipelineLayout,
    descriptor_set_layouts: Vec<vk::DescriptorSetLayout>,
    bind_point: vk::PipelineBindPoint,
}

impl Pipeline {
    pub fn create(shader_description: &ShaderDescription, device: Arc<Device>, width: u32, height: u32, compatible_render_pass: &RenderPass) -> Result<Self, Error> {
        
        let mut shader_stage_create_infos = Vec::new();
        let mut shader_modules = Vec::new();
        for shader_stage in shader_description.stages.iter() {
            let module = unsafe {
                create_shader_module(&device, shader_stage)?
            };

            shader_modules.push(module);

            shader_stage_create_infos.push(vk::PipelineShaderStageCreateInfo {
                module,
                p_name: shader_stage.entry.as_ptr(),
                stage: shader_stage.stage.into(),
                ..Default::default()
            });
        }

        let vertex_input_attribute_descriptions: Vec<vk::VertexInputAttributeDescription> = shader_description.attributes
            .iter()
            .enumerate()
            .map(|(index, attribute)| vk::VertexInputAttributeDescription {
                location: attribute.location,
                binding: index as u32,
                format: attribute.format.into(),
                offset: 0,
            }).collect();
        
        let vertex_input_binding_descriptions: Vec<vk::VertexInputBindingDescription> = shader_description.attributes
            .iter()
            .enumerate()
            .map(|(index, attribute)| vk::VertexInputBindingDescription {
                binding: index as u32,
                stride: attribute.stride,
                input_rate: vk::VertexInputRate::VERTEX,
            }).collect();

        let vertex_input_state_info = vk::PipelineVertexInputStateCreateInfo::builder()
            .vertex_attribute_descriptions(&vertex_input_attribute_descriptions)
            .vertex_binding_descriptions(&vertex_input_binding_descriptions);

        let vertex_input_assembly_state_info = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            ..Default::default()
        };

        let surface_resolution = vk::Extent2D {
            width,
            height,
        };

        let viewports = [vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: surface_resolution.width as f32,
            height: surface_resolution.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        }];

        let scissors = [vk::Rect2D {
            extent: surface_resolution,
            ..Default::default()
        }];
        
        let viewport_state_info = vk::PipelineViewportStateCreateInfo::builder()
            .scissors(&scissors)
            .viewports(&viewports);
        
        let rasterization_info = vk::PipelineRasterizationStateCreateInfo {
            front_face: vk::FrontFace::COUNTER_CLOCKWISE,
            line_width: 1.0,
            polygon_mode: vk::PolygonMode::FILL,
            cull_mode: vk::CullModeFlags::NONE,
            ..Default::default()
        };
        
        let multisample_state_info = vk::PipelineMultisampleStateCreateInfo::builder()
            .rasterization_samples(vk::SampleCountFlags::TYPE_1);

        let noop_stencil_state = vk::StencilOpState {
            fail_op: vk::StencilOp::KEEP,
            pass_op: vk::StencilOp::KEEP,
            depth_fail_op: vk::StencilOp::KEEP,
            compare_op: vk::CompareOp::ALWAYS,
            ..Default::default()
        };
        
        let depth_state_info = vk::PipelineDepthStencilStateCreateInfo {
            depth_test_enable: 1,
            depth_write_enable: 1,
            depth_compare_op: vk::CompareOp::LESS_OR_EQUAL,
            front: noop_stencil_state,
            back: noop_stencil_state,
            max_depth_bounds: 1.0,
            ..Default::default()
        };
        
        let color_blend_attachment_states = [vk::PipelineColorBlendAttachmentState {
            blend_enable: 0,
            src_color_blend_factor: vk::BlendFactor::SRC_COLOR,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_DST_COLOR,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ZERO,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
            color_write_mask: vk::ColorComponentFlags::all(),
        }];
        let color_blend_state = vk::PipelineColorBlendStateCreateInfo::builder()
            .logic_op(vk::LogicOp::CLEAR)
            .attachments(&color_blend_attachment_states);

        let dynamic_state = [vk::DynamicState::VIEWPORT, vk::DynamicState::SCISSOR];
        let dynamic_state_info = vk::PipelineDynamicStateCreateInfo::builder().dynamic_states(&dynamic_state);

        let mut descriptor_set_layouts = Vec::new();
        for descriptor_set_layout in &shader_description.descriptor_set_layouts {
            let mut bindings = Vec::new();
            for binding in &descriptor_set_layout.bindings {
                bindings.push(vk::DescriptorSetLayoutBinding {
                    binding: binding.binding,
                    descriptor_type: binding.descriptor_type.into(),
                    descriptor_count: binding.count,
                    stage_flags: binding.stage_flags.into(),
                    ..Default::default()
                });
            }

            let binding_flags = [
                vk::DescriptorBindingFlags::PARTIALLY_BOUND
                | vk::DescriptorBindingFlags::VARIABLE_DESCRIPTOR_COUNT
                | vk::DescriptorBindingFlags::UPDATE_UNUSED_WHILE_PENDING
            ];

            let mut next = vk::DescriptorSetLayoutBindingFlagsCreateInfo::builder()
                .binding_flags(&binding_flags);

            let descriptor_set_layout_create_info = vk::DescriptorSetLayoutCreateInfo::builder()
                .push_next(&mut next)
                .bindings(&bindings);
            let descriptor_set_layout = unsafe { device.create_descriptor_set_layout(&descriptor_set_layout_create_info, None)? };

            descriptor_set_layouts.push(descriptor_set_layout);
        }

        let mut push_constant_ranges = Vec::new();
        for push_constant in &shader_description.push_constants {
            push_constant_ranges.push(vk::PushConstantRange {
                offset: push_constant.offset,
                size: push_constant.size,
                stage_flags: push_constant.stage_flags.into(),
            });
        }

        let layout_create_info = vk::PipelineLayoutCreateInfo::builder()
            .set_layouts(&descriptor_set_layouts)
            .push_constant_ranges(&push_constant_ranges);
        let pipeline_layout = unsafe {
            device.create_pipeline_layout(&layout_create_info, None)?
        };

        let pipeline_create_info = vk::GraphicsPipelineCreateInfo::builder()
            .stages(&shader_stage_create_infos)
            .vertex_input_state(&vertex_input_state_info)
            .input_assembly_state(&vertex_input_assembly_state_info)
            .viewport_state(&viewport_state_info)
            .rasterization_state(&rasterization_info)
            .multisample_state(&multisample_state_info)
            .depth_stencil_state(&depth_state_info)
            .color_blend_state(&color_blend_state)
            .dynamic_state(&dynamic_state_info)
            .layout(pipeline_layout)
            .render_pass(compatible_render_pass.get_inner());

        let create_result = unsafe { device.create_graphics_pipelines(vk::PipelineCache::null(), &[pipeline_create_info.build()], None) };
        let inner = match create_result {
            Ok(pipelines) => Ok(pipelines[0]),
            Err((_, error)) => Err(error),
        }?;

        for module in shader_modules {
            unsafe {
                device.destroy_shader_module(module, None);
            }
        }

        Ok(Self {
            inner,
            device,
            pipeline_layout,
            descriptor_set_layouts,
            bind_point: shader_description.bind_point.into(),
        })
    }

    pub(crate) fn get_inner(&self) -> vk::Pipeline {
        self.inner
    }
}

impl Drop for Pipeline {
    fn drop(&mut self) {
        unsafe {
            for layout in &self.descriptor_set_layouts {
                self.device.destroy_descriptor_set_layout(*layout, None);
            }

            self.device.destroy_pipeline_layout(self.pipeline_layout, None);
            self.device.destroy_pipeline(self.inner, None);
        }
    }
}
