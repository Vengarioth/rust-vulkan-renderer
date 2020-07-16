use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct NodeIndex(usize);

pub struct DirectedGraph<Node> {
    nodes: Vec<Node>,
    edges: Vec<HashSet<NodeIndex>>,
    reverse_edges: Vec<HashSet<NodeIndex>>,
}

impl<Node> DirectedGraph<Node> {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
            reverse_edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) -> NodeIndex {
        let index = self.nodes.len();
        self.nodes.push(node);
        self.edges.push(HashSet::new());
        self.reverse_edges.push(HashSet::new());
        NodeIndex(index)
    }

    pub fn add_edge(&mut self, from: NodeIndex, to: NodeIndex) {
        self.edges[from.0].insert(to);
        self.reverse_edges[to.0].insert(from);
    }

    pub fn get_node(&self, index: NodeIndex) -> &Node {
        &self.nodes[index.0]
    }

    pub fn replace_node(&mut self, index: NodeIndex, mut node: Node) -> Node {
        use std::mem::swap;

        swap(&mut self.nodes[index.0], &mut node);

        node
    }

    pub fn print_graphviz<F>(&self, node_to_name: F)
        where F: Fn(&Node) -> String {
        println!("digraph G {{");
        for (from, to) in self.edges.iter().enumerate() {
            let from_name = node_to_name(&self.nodes[from]);
            for to in to.iter() {
                let to_name = node_to_name(&self.nodes[to.0]);
                println!("    \"{}\" -> \"{}\"", from_name, to_name);
            }
        }
        println!("}}");
    }

    pub fn topological_sort(&self, roots: &[NodeIndex]) -> Vec<NodeIndex> {
        let mut order = Vec::new();
        let mut visited = HashSet::new();

        for index in roots {
            self.sort_recursive(index, &mut order, &mut visited);
        }

        order
    }

    pub fn sort_recursive(&self, id: &NodeIndex, order: &mut Vec<NodeIndex>, visited: &mut HashSet<NodeIndex>) {
        if visited.contains(id) {
            return;
        }

        self.edges[id.0].iter().for_each(|id| self.sort_recursive(id, order, visited));

        order.push(*id);
        visited.insert(*id);
    }

    pub fn visit(&self, visitor: &mut impl Visitor<Node = Node>) {
        for (from, edges) in self.edges.iter().enumerate() {
            visitor.visit_node(&self, NodeIndex(from));
            for to in edges.iter() {
                visitor.visit_edge(&self, NodeIndex(from), *to);
            }
        }
    }
}

pub trait Visitor {
    type Node;

    fn visit_node(&mut self, graph: &DirectedGraph<Self::Node>, index: NodeIndex);
    fn visit_edge(&mut self, graph: &DirectedGraph<Self::Node>, from: NodeIndex, to: NodeIndex);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_orders_a_graph() {
        let mut graph = DirectedGraph::new();

        let a = graph.add_node(0);
        let b = graph.add_node(1);
        let c = graph.add_node(2);
        let d = graph.add_node(3);

        graph.add_edge(a, b);
        graph.add_edge(b, c);
        graph.add_edge(d, a);

        let order = graph.topological_sort(&[a]);

        assert_eq!(vec![c, b, a], order);
    }

    #[test]
    fn it_replaces_a_node() {
        let mut graph = DirectedGraph::new();

        let a = graph.add_node(0);

        let old = graph.replace_node(a, 1);

        assert_eq!(old, 0);
        assert_eq!(graph.get_node(a), &1);
    }
}
