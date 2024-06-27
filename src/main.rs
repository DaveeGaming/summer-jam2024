
// FEATURE CREEP CORNER
// Blue/Red shio/enemy/bullet
// Different palettes
// Weapon for diff color ships
// Bullethell kinda stuff
// Fast mouse movement vs asteroid movement vs any other idk


use macroquad::prelude::*;
use crate::game::*;
use crate::constant::*;

mod game;
mod constant;
mod player;
mod enemy;
mod bullet;
mod colors;

use miniquad::window::screen_size;

#[macroquad::main("title")]
async fn main() {
    let mut game = Game::default();
    let canvas = render_target(DESIGN_WIDTH as u32, DESIGN_HEIGHT as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    loop {
        let (screen_w, screen_h) = screen_size();

        let scale = f32::min(screen_w / DESIGN_WIDTH, screen_h / DESIGN_HEIGHT);

        let x_center = (screen_w - DESIGN_WIDTH * scale) / 2.0;
        let y_center = (screen_h - DESIGN_HEIGHT * scale) / 2.0;

        let mut camera = Camera2D::from_display_rect(Rect {
            x: 0.,
            y: 0.,
            w: (DESIGN_WIDTH) as f32,
            h: (DESIGN_HEIGHT) as f32   
        });

        camera.render_target = Some(canvas.clone());
        camera.zoom.y = -camera.zoom.y;
        
        set_camera(&camera);
        game.update();
        game.draw();

        set_default_camera();
        clear_background(BLANK);
        draw_texture_ex(&canvas.texture, x_center, y_center, WHITE, 
            DrawTextureParams {
                dest_size: Some( Vec2 { x: DESIGN_WIDTH * scale, y: DESIGN_HEIGHT * scale  }),
                ..Default::default()
            }
        );
        
        next_frame().await
    }
}
