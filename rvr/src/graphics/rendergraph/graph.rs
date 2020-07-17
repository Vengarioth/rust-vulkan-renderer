use super::*;
use crate::{
    graphics::*,
    util::DirectedGraph,
};

pub struct Graph {
    create_images: Vec<ImageResource>,
    import_images: Vec<ImageResource>,
    result_images: Vec<ImageHandle>,
    passes: Vec<Pass>,
}

impl Graph {
    pub(crate) fn new(
        create_images: Vec<ImageResource>,
        import_images: Vec<ImageResource>,
        result_images: Vec<ImageHandle>,
        passes: Vec<Pass>,
    ) -> Self {
        Self {
            create_images,
            import_images,
            result_images,
            passes,
        }
    }

    pub fn compile_schedule(&self) -> Schedule {
        use std::collections::{HashMap, HashSet};

        // first, create a hashmap that maps ImageHandle to the pass id that created it, if any
        let mut image_sources = HashMap::new();
        for pass in self.passes.iter() {
            for color_attachment in pass.render_target.color_attachments.iter() {
                image_sources.insert(color_attachment, pass.id);
            }

            if let Some(depth_stencil_attachment) = &pass.render_target.depth_stencil_attachment {
                image_sources.insert(depth_stencil_attachment, pass.id);
            }
        }


        // then create a dependency graph and insert all passes as nodes in the graph
        let mut passes = HashMap::new();
        let mut dependency_graph = DirectedGraph::new();
        for pass in self.passes.iter() {
            let pass_index = dependency_graph.add_node((pass.id, pass.name.to_string()));
            passes.insert(pass.id, pass_index);
        }

        // and fill the graph with edges based on when image versions are needed
        for pass in self.passes.iter() {
            for sample_image in pass.sample_images.iter() {
                if let Some(previous_pass) = image_sources.get(sample_image) {
                    dependency_graph.add_edge(
                        *passes.get(&pass.id).unwrap(),
                        *passes.get(previous_pass).unwrap(),
                    );
                }
            }

            for color_attachment in pass.render_target.color_attachments.iter() {
                if let Some(previous_image) = color_attachment.previous_version() {
                    if let Some(previous_pass) = image_sources.get(&previous_image) {
                        dependency_graph.add_edge(
                            *passes.get(&pass.id).unwrap(),
                            *passes.get(previous_pass).unwrap(),
                        );
                    }
                }
            }

            if let Some(depth_stencil_attachment) = &pass.render_target.depth_stencil_attachment {
                if let Some(previous_image) = depth_stencil_attachment.previous_version() {
                    if let Some(previous_pass) = image_sources.get(&previous_image) {
                        dependency_graph.add_edge(
                            *passes.get(&pass.id).unwrap(),
                            *passes.get(previous_pass).unwrap(),
                        );
                    }
                }
            }
        }

        // then find the passes we have to execute and prune passes we don't need this frame
        // also find all images that were never part of a pass but need to transition layout
        // to be presented
        let mut transition_images = Vec::new();
        let mut root_passes = Vec::new();

        for result_image in self.result_images.iter() {
            if let Some(pass) = image_sources.get(result_image) {
                // image gets presented and was part of a render pass
                root_passes.push(*passes.get(pass).unwrap());
            } else {
                // image gets presented, but was not part of any render pass
                transition_images.push(result_image);
            }
        }

        // the order in which passes get executed
        // we ignore potential optimizations like collapsing passes for now
        let order = dependency_graph.topological_sort(&root_passes);
        
        // now we start building the linear schedule to be executed by the renderer
        let mut created_images = HashSet::new();
        let mut image_layouts = HashMap::new();
        let mut schedule = ScheduleBuilder::new();

        // first set all image layouts to their initial states
        for image in self.create_images.iter() {
            image_layouts.insert(image.id, image.description.initial_layout);
        }
        for image in self.import_images.iter() {
            image_layouts.insert(image.id, image.description.initial_layout);
        }

        // iterate over all passes in execution order
        order.iter().for_each(|index| {
            let (id, _) = dependency_graph.get_node(*index);
            let pass = self.passes.iter().find(|p| p.id == *id).unwrap();

            for color_attachment in pass.render_target.color_attachments.iter() {
                if !created_images.contains(color_attachment) {
                    // TODO create image
                    created_images.insert(color_attachment.clone());
                }
            }

            if let Some(depth_stencil_attachment) = &pass.render_target.depth_stencil_attachment {
                if !created_images.contains(depth_stencil_attachment) {
                    // TODO create image
                    created_images.insert(depth_stencil_attachment.clone());
                }
            }

            // TODO define render pass

            // TODO execute pass
        });

        // insert barriers for images that still need to be transitioned to another layout
        for transition_image in transition_images {
            let from_layout = *image_layouts.get(&transition_image.id).unwrap();
            let to_layout = ImageLayout::Present;

            schedule.add_image_layout_barrier(
                transition_image.id,
                from_layout,
                to_layout,
            );
        }

        schedule.build()
    }
}
