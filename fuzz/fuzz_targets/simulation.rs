use honggfuzz::fuzz;
use egui::Vec2;
use egui_graphs::{Edge, Node};
use petgraph::stable_graph::{NodeIndex, StableGraph};
use fdg_sim::{ForceGraph, force::fruchterman_reingold, Simulation, SimulationParameters, ForceGraphHelper};

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

            // Create force graph
            let mut force_graph = ForceGraph::with_capacity(graph.node_count(), graph.edge_count());
            graph.node_indices().for_each(|idx| {
                let idx = idx.index();
                force_graph.add_force_node(format!("{}", idx).as_str(), ());
            });
            graph.edge_indices().for_each(|idx| {
                let (source, target) = graph.edge_endpoints(idx).unwrap();
                force_graph.add_edge(source, target, ());
            });

            // Create simulation parameters
            let mut params = SimulationParameters::default();
            let force = fruchterman_reingold(100., 0.5);
            params.set_force(force);
            
            // Create simulation
            let _: Simulation<(), ()> = Simulation::from_graph(force_graph, params);
        });
    }
}
