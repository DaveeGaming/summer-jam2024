
use macroquad::prelude::*;
use miniquad::window::screen_size;

const DESIGN_WIDTH: f32 = 1024.;
const DESIGN_HEIGHT: f32 = 576.;

#[macroquad::main("title")]
async fn main() {
    let canvas = render_target(DESIGN_WIDTH as u32, DESIGN_HEIGHT as u32);

    loop {
        let (screen_w, screen_h) = screen_size();

        let scale = f32::min(screen_w / DESIGN_WIDTH, screen_h / DESIGN_HEIGHT);

        let mut camera = Camera2D::from_display_rect(Rect {
            x: 0.,
            y: 0.,
            w: (DESIGN_WIDTH * scale) as f32,
            h: (DESIGN_HEIGHT * scale) as f32   
        });

        camera.zoom.y = -camera.zoom.y;
        
        set_camera(&camera);

        clear_background(BLUE);
        draw_text("test", 50.0, 50.0, 32.0, GREEN);
        draw_line(0.0, 0.0, DESIGN_WIDTH as f32, DESIGN_HEIGHT as f32, 1.0, GREEN);
        
        set_default_camera();
        clear_background(BLANK);
        draw_texture(&canvas.texture, 0., 0., WHITE);
        
        next_frame().await
    }
}
