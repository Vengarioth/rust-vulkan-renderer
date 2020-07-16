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
        use std::collections::HashMap;

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
            let pass_index = dependency_graph.add_node(pass.name.to_string());
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

        let roots: Vec<NodeIndex> = self.result_images.iter()
            .map(|image| image_sources.get(image).unwrap())
            .map(|pass| *passes.get(pass).unwrap())
            .collect();

        dependency_graph.print_graphviz(|n| n.to_string());
        let order = dependency_graph.topological_sort(&roots);


        order.iter().for_each(|index| {
            let name = dependency_graph.get_node(*index).to_string();
            println!("{}", name);
        });
    }
}
