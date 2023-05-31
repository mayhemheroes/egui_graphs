use honggfuzz::fuzz;
use egui::Vec2;
use egui_graphs::Node;

fn main() {
    loop {
        fuzz!(|data: [f32; 512]| {
            for point in data.chunks(2) {
                //if point.len() == 2 {
                    let (x, y) = (point[0], point[1]);
                    let _ = Node::new(Vec2::new(x, y), ());
                //}
            }
        });
    }
}
