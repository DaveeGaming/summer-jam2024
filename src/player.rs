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
    dx: f32,
    dy: f32,
    shoot_dx: f32,
    shoot_dy: f32,
    bullets: Vec<PlayerBullet>,
    shoot_t: u32,
    melee_t: u32
}

impl Default for Player {
    fn default() -> Player {
        Player {
            state: ColorState::Secondary,
            health: 10,
            rotation: 0.0,
            x: 50.0,
            y: 50.0,
            dx: 0.0,
            dy: 0.0,
            shoot_dx: 1.0,
            shoot_dy: 0.0,
            bullets: vec![],
            shoot_t: 0,
            melee_t: 0
        }
    }
}

impl Player {
    pub fn draw(&self, c: &ColorPalette) {
        let color = match self.state {
            ColorState::Primary => c.FG_PRIMARY,
            ColorState::Secondary => c.FG_SECONDARY
        };

        let center_x = self.x + 15.0;
        let center_y = self.y + 15.0;

        for b in &self.bullets {
            b.draw();
        }

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
                self.bullets.push(PlayerBullet {
                    x: self.x + 15.0,
                    y: self.y + 15.0,
                    dx: self.shoot_dx,
                    dy: self.shoot_dy,
                    is_dead: false  
                });
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

        if is_key_pressed(KeyCode::Space){
            self.state = self.state.next();
        }

        self.bullets.retain_mut(|b| {
            b.update();

            !b.is_dead
        });
    }
}

pub struct PlayerBullet {
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub is_dead: bool
}

impl PlayerBullet {
    pub fn update(&mut self) {
        let dt = get_frame_time();
        let speed = 550.0;

        self.x += self.dx * speed * dt;
        self.y += self.dy * speed * dt;
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, 8.0, 8.0, WHITE);
    }
}