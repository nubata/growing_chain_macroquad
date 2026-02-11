use macroquad::prelude::*;

mod model;

#[macroquad::main("GrowingChain")]
async fn main() {
    let mut model = model::Model::new();

    loop {
        // Update model.
        model.update();

        // Draw model.
        model.draw();

        // Refresh screen.
        next_frame().await;
    }
}
