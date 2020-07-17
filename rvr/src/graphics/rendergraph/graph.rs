use super::*;
use crate::util::{DirectedGraph, NodeIndex};

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

    pub fn compile_schedule(&self) {
        use std::collections::{HashMap, HashSet};

        // first, create a hashmap that maps ImageHandle to pass.id
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
        
        let mut created_images = HashSet::new();

        // dependency_graph.print_graphviz(|(_, name)| name.to_string());
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

            // TODO execute pass
        });

        // TODO any images left in need of transitioning to another layout?
    }
}
