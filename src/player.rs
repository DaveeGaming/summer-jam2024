use macroquad::prelude::*;

use crate::{colors::ColorPalette, ColorState};


pub struct Player {
    health: i32,
    rotation: f32,
    pub x: f32,
    pub y: f32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            health: 10,
            rotation: 0.0,
            x: 50.0,
            y: 50.0,
        }
    }
}

impl Player {
    pub fn update(&mut self) {
        let dt = get_frame_time();
    }

    pub fn draw(&self, c: &ColorPalette, s: &ColorState) {
        let color = match s {
            ColorState::Primary => c.FG_PRIMARY,
            ColorState::Secondary => c.FG_SECONDARY
        };

        draw_rectangle_ex(self.x, self.y, 30.0, 30.0,
            DrawRectangleParams {
                color: color,
                rotation: self.rotation,
                ..Default::default()
            }
        )
    }

    pub fn input(&mut self) {
        let dt = get_frame_time();
        let speed = 200.0;

        if is_key_down(KeyCode::A){
            self.x -= speed * dt;
        }
        if is_key_down(KeyCode::D){
            self.x += speed * dt;
        }
        if is_key_down(KeyCode::W){
            self.y -= speed * dt;
        }
        if is_key_down(KeyCode::S){
            self.y += speed * dt;
        }
    }
}