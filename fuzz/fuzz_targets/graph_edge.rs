use honggfuzz::fuzz;
use egui::Vec2;
use egui_graphs::{Edge, Node};
use petgraph::stable_graph::{NodeIndex, StableGraph};

fn main() {
    loop {
        fuzz!(|data: [f32; 512]| {
            // create graph
            let mut graph: StableGraph<Node<()>, Edge<()>> = StableGraph::new();

            // add nodes
            for point in data.chunks(2) {
                //if point.len() == 2 {
                    let (x, y) = (point[0], point[1]);
                    graph.add_node(Node::new(Vec2::new(x, y), ()));
                //}
            }

            // add edges
            let count = 250; //(data.len() + 1) / 2;
            for i in 0..count {
                graph.add_edge(
                    NodeIndex::new(i),
                    NodeIndex::new((i + 1) % count),
                    Edge::new(()),
                );
            }
        });
    }
}
