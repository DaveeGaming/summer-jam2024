use macroquad::prelude::*;

use crate::{colors::ColorPalette, ColorState, DESIGN_HEIGHT, DESIGN_WIDTH};



pub struct Player {
    pub health: i32,
    rotation: f32,
    pub x: f32,
    pub y: f32,
    dx: f32,
    dy: f32,
    shoot_dx: f32,
    shoot_dy: f32,
    shoot_t: u32,
    melee_t: u32
}

impl Default for Player {
    fn default() -> Player {
        Player {
            health: 10,
            rotation: 0.0,
            x: 50.0,
            y: 50.0,
            dx: 0.0,
            dy: 0.0,
            shoot_dx: 1.0,
            shoot_dy: 0.0,
            shoot_t: 0,
            melee_t: 0
        }
    }
}

impl Player {
    pub fn draw(&self, c: &ColorPalette, s: &ColorState) {
        let color = match s {
            ColorState::Primary => c.FG_PRIMARY,
            ColorState::Secondary => c.FG_SECONDARY
        };

        let center_x = self.x + 15.0;
        let center_y = self.y + 15.0;

        if self.melee_t > 0 && self.melee_t < 15 {
            draw_circle(center_x, center_y, 45.0, YELLOW);
        }

        draw_rectangle_ex(self.x, self.y, 30.0, 30.0,
            DrawRectangleParams {
                color: color,
                rotation: self.rotation,
                ..Default::default()
            }
        );

        draw_line(self.x + 15.0, self.y + 15.0, self.x + 15.0 + self.shoot_dx * 5.0, self.y + 15.0 + self.shoot_dy * 5.0, 2.0, WHITE);
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();
        let speed = 220.0;

        let mut dx = 0.0;
        let mut dy = 0.0;

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dx = -1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dx = 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dy = -1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dy = 1.0;
        }

        if dx != 0.0 { self.dx = dx * speed; }
        if dy != 0.0 { self.dy = dy * speed; }

        if is_key_down(KeyCode::J) || is_key_down(KeyCode::F) {
            if self.shoot_t % 5 == 0 {
                
            }

            self.shoot_t += 1;
        } else {
            if dx != 0.0 || dy != 0.0 {
                self.shoot_dx = dx;
                self.shoot_dy = dy; 
            }

            self.shoot_t = 0;
        }

        if self.melee_t == 0 && (is_key_down(KeyCode::K) || is_key_down(KeyCode::G)) {
            self.melee_t = 23;
        }

        if self.melee_t > 0 {
            self.melee_t -= 1;
        }

        self.x += self.dx * dt;
        self.y += self.dy * dt;

        self.dx *= 0.85;
        self.dy *= 0.85;
    }
}
