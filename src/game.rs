use macroquad::prelude::*;


pub struct Player {
    state: bool,
    health: i32,
    rotation: f32,
    x: f32,
    y: f32,
    vel_x: f32,
    vel_y: f32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            state: false,
            health: 10,
            rotation: 0.0,
            x: 50.0,
            y: 50.0,
            vel_x: 0.0,
            vel_y: 0.0,
        }
    }
}

impl Player {
    pub fn update(&mut self) {
        let dt = get_frame_time();
        self.x += self.vel_x * dt;
        self.y += self.vel_y * dt;

        self.vel_x *= 0.99 * dt * 1000.0;
        self.vel_y *= 0.99 * dt * 1000.0;
    }

    pub fn draw(&self) {
        draw_text( format!("x:{}, y:{}", self.x, self.y).as_str() , 10.0, 10.0, 30.0, WHITE);
        draw_rectangle_ex(self.x, self.y, 10.0, 10.0,
            DrawRectangleParams {
                rotation: self.rotation,
                ..Default::default()
            }
        )
    }

    pub fn input(&mut self) {
        let dt = get_frame_time();
        let speed = 500.0;
        let max_speed = 200.0;

        if is_key_down(KeyCode::A){
            self.vel_x = f32::max( -max_speed, self.vel_x - speed * dt) 
        }
        if is_key_down(KeyCode::D){
            self.vel_x = f32::min( max_speed, self.vel_x + speed * dt) 
        }
        if is_key_down(KeyCode::W){
            self.vel_y = f32::max( -max_speed, self.vel_y - speed * dt) 
        }
        if is_key_down(KeyCode::S){
            self.vel_y = f32::min( max_speed, self.vel_y + speed * dt) 
        }
    }
}
