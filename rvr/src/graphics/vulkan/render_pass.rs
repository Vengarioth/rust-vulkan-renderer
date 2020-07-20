use std::sync::Arc;
use ash::{vk, Device, version::DeviceV1_0};
use crate::Error;

#[derive(Debug)]
pub struct RenderPassDescription {
    pub attachments: Vec<AttachmentDescription>,
    pub subpasses: Vec<SubpassDescription>,
    pub dependencies: Vec<SubpassDependencyDescription>,
}

#[derive(Debug)]
pub struct AttachmentDescription {
    pub format: vk::Format,
    pub samples: vk::SampleCountFlags,
    pub load_op: vk::AttachmentLoadOp,
    pub store_op: vk::AttachmentStoreOp,
    pub stencil_load_op: vk::AttachmentLoadOp,
    pub stencil_store_op: vk::AttachmentStoreOp,
    pub initial_layout: vk::ImageLayout,
    pub final_layout: vk::ImageLayout,
}

#[derive(Debug)]
pub struct AttachmentReferenceDescription {
    pub attachment: u32,
    pub layout: vk::ImageLayout,
}

#[derive(Debug)]
pub struct SubpassDescription {
    pub color_attachments: Vec<AttachmentReferenceDescription>,
    pub preserve_attachments: Vec<u32>,
    pub depth_stencil_attachment: Option<AttachmentReferenceDescription>,
}

#[derive(Debug)]
pub struct SubpassDependencyDescription {
    pub src_subpass: u32,
    pub dst_subpass: u32,
    pub src_stage_mask: vk::PipelineStageFlags,
    pub dst_stage_mask: vk::PipelineStageFlags,
    pub src_access_mask: vk::AccessFlags,
    pub dst_access_mask: vk::AccessFlags,
}

pub struct RenderPass {
    inner: vk::RenderPass,
    device: Arc<Device>,
}

impl RenderPass {
    pub fn create(device: Arc<Device>, render_pass_description: &RenderPassDescription) -> Result<Self, Error> {

        let attachments: Vec<vk::AttachmentDescription> = render_pass_description.attachments.iter()
            .map(|attachment| {
                vk::AttachmentDescription {
                    format: attachment.format,
                    samples: attachment.samples,
                    load_op: attachment.load_op,
                    store_op: attachment.store_op,
                    stencil_load_op: attachment.stencil_load_op,
                    stencil_store_op: attachment.stencil_store_op,
                    initial_layout: attachment.initial_layout,
                    final_layout: attachment.final_layout,
                    ..Default::default()
                }
            })
            .collect();

        let subpass_attachments: Vec<(Vec<vk::AttachmentReference>, Vec<u32>, Option<vk::AttachmentReference>)> = render_pass_description.subpasses.iter()
            .map(|subpass| {
                let color_attachment_refs: Vec<vk::AttachmentReference> = subpass.color_attachments.iter()
                    .map(|color_attachment| {
                        vk::AttachmentReference {
                            attachment: color_attachment.attachment,
                            layout: color_attachment.layout,
                        }
                    })
                    .collect();

                let preserve_attachments = subpass.preserve_attachments.clone();

                let depth_stencil_attachment = if let Some(depth_stencil_attachment) = &subpass.depth_stencil_attachment {
                    Some(vk::AttachmentReference {
                        attachment: depth_stencil_attachment.attachment,
                        layout: depth_stencil_attachment.layout,
                    })
                } else {
                    None
                };

                (color_attachment_refs, preserve_attachments, depth_stencil_attachment)
            })
            .collect();

        let subpasses: Vec<vk::SubpassDescription> = subpass_attachments.iter()
            .map(|(color_attachment_refs, preserve_attachments, depth_stencil_attachment)| {

                let mut builder = vk::SubpassDescription::builder()
                    .color_attachments(&color_attachment_refs)
                    .preserve_attachments(&preserve_attachments)
                    .pipeline_bind_point(vk::PipelineBindPoint::GRAPHICS);

                if let Some(depth_stencil_attachment) = depth_stencil_attachment {
                    builder = builder.depth_stencil_attachment(depth_stencil_attachment);
                }

                builder.build()
            })
            .collect();

        let dependencies: Vec<vk::SubpassDependency> = render_pass_description.dependencies.iter()
            .map(|dependency| {
                vk::SubpassDependency {
                    src_subpass: dependency.src_subpass,
                    dst_subpass: dependency.dst_subpass,
                    src_stage_mask: dependency.src_stage_mask,
                    dst_stage_mask: dependency.dst_stage_mask,
                    src_access_mask: dependency.src_access_mask,
                    dst_access_mask: dependency.dst_access_mask,
                    ..Default::default()
                }
            })
            .collect();

        let create_info = vk::RenderPassCreateInfo::builder()
            .attachments(&attachments)
            .subpasses(&subpasses)
            .dependencies(&dependencies);

        let inner = unsafe { device.create_render_pass(&create_info, None)? };

        Ok(RenderPass {
            inner,
            device,
        })
    }

    pub(crate) fn get_inner(&self) -> vk::RenderPass {
        self.inner
    }
}

impl Drop for RenderPass {
    fn drop(&mut self) {
        unsafe {
            self.device.destroy_render_pass(self.inner, None);
        }
    }
}
