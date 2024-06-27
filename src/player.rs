use macroquad::prelude::*;

use crate::colors::ColorPalette;

pub enum ColorState {
    Primary,
    Secondary
}

impl ColorState {
    pub fn next(&self) -> Self {
        match self {
            Self::Secondary => Self::Primary,
            Self::Primary => Self::Secondary,
        }
    }
}

pub struct Player {
    pub state: ColorState,
    health: i32,
    rotation: f32,
    x: f32,
    y: f32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            state: ColorState::Secondary,
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

    pub fn draw(&self, c: &ColorPalette) {
        let color = match self.state {
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

        if is_key_pressed(KeyCode::Space){
            self.state = self.state.next();
        }
    }
}