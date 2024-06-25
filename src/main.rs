use macroquad::prelude::*;
use crate::game::*;

mod game;

#[macroquad::main("title")]
async fn main() {

    let mut p = Player::default();
    loop {
        p.input();
        p.update();
        p.draw();

        next_frame().await;
    }
}
