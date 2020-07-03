use tinypath::Path;
use std::collections::HashMap;
use petgraph::{Graph, graph::NodeIndex, dot::{Dot, Config}, Direction};
use crate::Error;

#[derive(Debug)]
pub struct DependencyGraph {
    path_to_node: HashMap<Path, NodeIndex>,
    graph: Graph<Path, ()>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            path_to_node: HashMap::new(),
            graph: Graph::new(),
        }
    }

    pub fn insert_asset(&mut self, path: &Path) {
        if self.path_to_node.contains_key(path) {
            return;
        }

        let path = path.to_owned();
        let index = self.graph.add_node(path.clone());
        self.path_to_node.insert(path.clone(), index);
    }

    pub fn remove_asset(&mut self, path: Path) {
        if let Some(index) = self.path_to_node.remove(&path) {
            self.graph.remove_node(index);
        }
    }

    pub fn add_dependency(&mut self, from: &Path, to: &Path) {
        self.insert_asset(from);
        self.insert_asset(to);

        let from = self.path_to_node.get(from).unwrap();
        let to = self.path_to_node.get(to).unwrap();

        if self.graph.find_edge(*from, *to).is_none() {
            self.graph.add_edge(*from, *to, ());
        }
    }

    pub fn find_dependant_assets(&self, path: &Path) -> Result<Vec<Path>, Error> {
        if let Some(query) = self.path_to_node.get(path) {
            let paths = self.graph.neighbors_directed(*query, Direction::Incoming)
                .map(|node| self.graph.node_weight(node).unwrap().clone())
                .collect();
            
            Ok(paths)
        } else {
            Ok(Vec::new())
        }
    }

    pub fn print_dot(&self) {
        println!("{:?}", Dot::with_config(&self.graph, &[Config::EdgeNoLabel]));
    }
}
