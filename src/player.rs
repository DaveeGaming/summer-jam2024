use macroquad::prelude::*;
use macroquad::audio::*;
use crate::game::*;
use crate::bullet::*;
use crate::colors::*;

pub struct Player {
    pub max_health: i32,
    pub health: i32,
    pub rotation: f32,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub spread: f32,
    pub projectiles: f32,
    pub attack_speed: f32,
    pub move_speed: f32,
    pub bullet_size: f32,
    pub bullet_speed: f32,
    pub damage: f32,
    pub shoot_dx: f32,
    pub shoot_dy: f32,
    pub shoot_t: f32,
    pub melee_t: f32,
    pub melee_range: f32,

    pub heal_from_b: i32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            max_health: 10,
            health: 10,
            rotation: 0.0,
            size: 40.0,
            damage: 1.0,
            spread: 3.0,
            move_speed: 300.0,
            projectiles: 1.0,
            bullet_size: 6.0,
            bullet_speed: 550.0,
            x: DESIGN_WIDTH/2.0,
            y: 700.0,
            dx: 0.0,
            dy: 0.0,
            shoot_dx: 1.0,
            shoot_dy: 0.0,
            attack_speed: 0.1,
            shoot_t: 0.0,
            melee_t: 0.0,
            melee_range: 80.0,

            heal_from_b: 1,
        }
    }
}

impl Player {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size
        }
    }
}

impl Game {
    pub fn draw_healthbar(&self) {
        let color = match self.color_state {
            ColorState::Primary => self.palette.fg_primary,
            ColorState::Secondary => self.palette.fg_secondary
        };

        let mut bg_health = color;
        bg_health.a = 0.6;

        let mut hp = self.player.health;
        let height = 70.0;
        let gap = -8.0;
        let width = height / 2.0;
        let full_size = self.player.max_health as f32 * (width + gap);

        
        for i in 0..self.player.max_health {
            let x = DESIGN_WIDTH / 2.0 + (i as f32 * (width + gap)) - full_size / 2.0;
            let y = DESIGN_HEIGHT - 100.0;

            if hp > 0 {
                draw_texture_ex(&self.assets.hpbar, x, y, color, 
                    DrawTextureParams { dest_size: Some(Vec2 { x: width, y: height }), ..Default::default()});
                hp -= 1;
            } else {
                draw_texture_ex(&self.assets.hpbar, x, y, bg_health, 
                    DrawTextureParams { dest_size: Some(Vec2 { x: width, y: height }), ..Default::default()});
            }
        }

    }

    pub fn player_draw(&self) {
        let color = match self.color_state {
            ColorState::Primary => self.palette.fg_primary,
            ColorState::Secondary => self.palette.fg_secondary
        };

        let center_x = self.player.x + self.player.size/2.0;
        let center_y = self.player.y + self.player.size/2.0;

        let mut melee_color = color.clone();
        melee_color.a = 0.5;


        if self.player.melee_t > 1.5 && self.player.melee_t < 2.0 {
            draw_circle(center_x, center_y, self.player.melee_range, melee_color);
        }

        // draw_rectangle_ex(self.player.x, self.player.y, self.player.size, self.player.size,
        //     DrawRectangleParams {
        //         color: color,
        //         ..Default::default()
        //     }
        // );

        let texture = self.characters[self.selected_char as usize].get_sprite(&self.assets);
        draw_texture_ex(texture, self.player.x, self.player.y, color, 
            DrawTextureParams { dest_size: Some(Vec2 { x: self.player.size, y: self.player.size }), ..Default::default()});

        draw_line(
            self.player.x + self.player.size/2.0, // x center 
            self.player.y + self.player.size/2.0, // y center
            self.player.x + self.player.size/2.0 + self.player.shoot_dx * self.player.size/4.0, // x center + x_dir
            self.player.y + self.player.size/2.0 + self.player.shoot_dy * self.player.size/4.0, // y center + y_dir 
            2.0, WHITE);

            self.draw_healthbar();

    }


    pub fn player_update(&mut self) {
        let dt = get_frame_time();

        let mut dir = Vec2::ZERO;

        if is_key_down(KeyCode::A) {
            dir.x = -1.0;
        }
        if is_key_down(KeyCode::D) {
            dir.x = 1.0;
        }
        if is_key_down(KeyCode::W) {
            dir.y = -1.0;
        }
        if is_key_down(KeyCode::S) {
            dir.y = 1.0;
        }
        dir = dir.normalize_or_zero();

        let mut shoot_dir = Vec2::ZERO;
        let mut shooting = false;
        if is_key_down(KeyCode::Left) {
            shoot_dir.x = -1.0;
            shooting = true;
        }
        if is_key_down(KeyCode::Right) {
            shoot_dir.x = 1.0;
            shooting = true;
        }
        if is_key_down(KeyCode::Up) {
            shoot_dir.y = -1.0;
            shooting = true;
        }
        if is_key_down(KeyCode::Down) {
            shoot_dir.y = 1.0;
            shooting = true;
        }
        shoot_dir = shoot_dir.normalize_or_zero();

        self.player.shoot_dx = shoot_dir.x;
        self.player.shoot_dy = shoot_dir.y;

        self.player.dx = dir.x * self.player.move_speed;
        self.player.dy = dir.y * self.player.move_speed;

        if self.player.shoot_t > 0.0 {
            self.player.shoot_t -= dt;
        }

        if shooting {
            if self.player.shoot_t <= 0.0 {

                play_sound(&self.assets.shoot, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
                // rotate by spread randomly
                let deg_projectile = 2.0;
                let offset = deg_projectile/2.0;

                for i in 0..self.player.projectiles as i32 {
                    
                    let actual_spread = rand::gen_range(-self.player.spread, self.player.spread);
                    let  dir = rotate_vec(Vec2 { x: self.player.shoot_dx, y: self.player.shoot_dy}, actual_spread);
                    let mut dir = rotate_vec(dir, -offset + i as f32*deg_projectile);
                    dir = dir.normalize_or_zero();

                    self.bullets.push(Bullet::new(
                        1, // damage
                        self.player.x + 15.0, // x
                        self.player.y + 15.0, // y
                        dir.x, // dx
                        dir.y, // dy
                        self.player.bullet_size,
                        self.player.bullet_speed,
                        BulletType::Player, // kind
                    ));
                }



                self.player.shoot_t += self.player.attack_speed;
            }

        }

        if self.player.melee_t < 0.0 && (is_key_down(KeyCode::K) || is_key_down(KeyCode::G)) && false {
            self.player.melee_t = 2.0;
        }

        if self.player.melee_t >= 0.0 {
            self.player.melee_t -= get_frame_time();
        }

        self.player.x += self.player.dx * dt;
        self.player.y += self.player.dy * dt;
    }
}