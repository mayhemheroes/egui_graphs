use honggfuzz::fuzz;
use egui::Vec2;
use egui_graphs::{Edge, Node};
use petgraph::stable_graph::{NodeIndex, StableGraph};

fn main() {
    loop {
        fuzz!(|data: [f32; 128]| {
            // create graph
            let mut graph: StableGraph<Node<()>, Edge<()>> = StableGraph::new();

            // add nodes
            for point in data.chunks(2) {
                let (x, y) = (point[0], point[1]);
                graph.add_node(Node::new(Vec2::new(x, y), ()));
            }

            // add edges
            let count = 64;
            for i in 0..count {
                for j in 0..count {
                    if i != j {
                        graph.add_edge(
                            NodeIndex::new(i),
                            NodeIndex::new(j),
                            Edge::new(()),
                        );
                    }
                }
            }
        });
    }
}
