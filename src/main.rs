use macroquad::prelude::*;

#[macroquad::main("title")]
async fn main() {
    loop {
        draw_text("Hello world!", 100.0, 100.0, 30.0, WHITE);
        next_frame().await;
    }
}
