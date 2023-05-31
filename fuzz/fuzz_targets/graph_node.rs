use honggfuzz::fuzz;
use egui::Vec2;
use egui_graphs::{Edge, Node};
use petgraph::stable_graph::StableGraph;

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
        });
    }
}
