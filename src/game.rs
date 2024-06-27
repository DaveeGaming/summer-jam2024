use macroquad::{prelude::*, rand};
use crate::{assets::Assets, colors::ColorPalette, enemy::*};

pub const DESIGN_WIDTH: f32 = 1024.;
pub const DESIGN_HEIGHT: f32 = 576.;

// =========== UTILS ============



#[derive(PartialEq, Eq)]
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




pub struct Game {
    state: ColorState,
    assets: Assets,
    palettes: [ColorPalette; 2],
    palette: ColorPalette,
    wave_current: i32,
    wave_start: bool,
    enemies: Vec<Box<dyn Enemy>>, // Box is for allocating to the heap
    bullets: Vec<PlayerBullet>,
    player: Player,
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
impl Default for Game {
    fn default() -> Self {
        Game {
            state: ColorState::Primary,
            assets: Assets::default(),
            palette: ColorPalette::default(),
            player: Player::default(),
            enemies: Vec::new(),
            bullets: vec![],
            wave_current: 1,
            wave_start: false,


            palettes: [
                ColorPalette::default(),
                ColorPalette::create_from(ORANGE, GREEN)
            ]
        }    
    }
}

impl Game {
    pub fn update(&mut self) {
        self.player_update();

        for enemy in self.enemies.iter_mut() {
            enemy.update(&self.player, &self.state);
        }

        if self.enemies.len() == 0 && self.wave_start {
            self.wave_current += 1;
            self.wave_start = false;
        }

        if is_key_pressed(KeyCode::C) {
            self.palette = self.palettes[ rand::gen_range(0, self.palettes.len()) ]
        }

        if is_key_pressed(KeyCode::Space){
            self.state = self.state.next();
        }

        if is_key_pressed(KeyCode::B) {
            self.enemies.push( 
                Box::new(FollowEnemy::new(10, 50.0, 50.0, FollowEnemyType::ChangeSpeed(100.0, 50.0)))
            );
        }
    }


    pub fn draw(&mut self) {
        let bg_color = match self.state {
            ColorState::Primary =>  self.palette.BG_PRIMARY,
            ColorState::Secondary => self.palette.BG_SECONDARY
        };


        clear_background(bg_color);
        for enemy in self.enemies.iter_mut() {
            enemy.draw(&self.state);
        }
        
        self.player_draw();
        
        let x_center = DESIGN_WIDTH / 2.0;
        let wave_txt = format!("Wave {}", self.wave_current);
        draw_text_centered(&wave_txt, x_center, 20.0, 40.0, &self.assets.font_monogram);
    }





    // ================ PLAYER ====================


    pub fn player_draw(&self) {
        let color = match self.state {
            ColorState::Primary => self.palette.FG_PRIMARY,
            ColorState::Secondary => self.palette.FG_SECONDARY
        };

        let center_x = self.player.x + 15.0;
        let center_y = self.player.y + 15.0;

        for b in &self.bullets {
            b.draw();
        }

        if self.player.melee_t > 0 && self.player.melee_t < 15 {
            draw_circle(center_x, center_y, 45.0, YELLOW);
        }

        draw_rectangle_ex(self.player.x, self.player.y, 30.0, 30.0,
            DrawRectangleParams {
                color: color,
                rotation: self.player.rotation,
                ..Default::default()
            }
        );

        draw_line(self.player.x + 15.0, self.player.y + 15.0, self.player.x + 15.0 + self.player.shoot_dx * 5.0, self.player.y + 15.0 + self.player.shoot_dy * 5.0, 2.0, WHITE);
    }


    pub fn player_update(&mut self) {
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

        if dx != 0.0 { self.player.dx = dx * speed; }
        if dy != 0.0 { self.player.dy = dy * speed; }

        if is_key_down(KeyCode::J) || is_key_down(KeyCode::F) {
            if self.player.shoot_t % 5 == 0 {
                self.bullets.push(PlayerBullet {
                    x: self.player.x + 15.0,
                    y: self.player.y + 15.0,
                    dx: self.player.shoot_dx,
                    dy: self.player.shoot_dy,
                    is_dead: false  
                });
            }

            self.player.shoot_t += 1;
        } else {
            if dx != 0.0 || dy != 0.0 {
                self.player.shoot_dx = dx;
                self.player.shoot_dy = dy; 
            }

            self.player.shoot_t = 0;
        }

        if self.player.melee_t == 0 && (is_key_down(KeyCode::K) || is_key_down(KeyCode::G)) {
            self.player.melee_t = 23;
        }

        if self.player.melee_t > 0 {
            self.player.melee_t -= 1;
        }

        self.player.x += self.player.dx * dt;
        self.player.y += self.player.dy * dt;

        self.player.dx *= 0.85;
        self.player.dy *= 0.85;

        self.bullets.retain_mut(|b| {
            b.update();

            !b.is_dead
        });
    }

}






pub fn dir_to_player(x: f32, y: f32, p: &Player) -> Vec2 {
    let diff = Vec2 { 
        x: p.x - x,
        y: p.y - y,
    };

    return diff.normalize_or_zero();
}

pub fn distance_to_player(x: f32, y: f32, p: &Player) -> f32 {
    let diff = Vec2 { 
        x: p.x - x,
        y: p.y - y,
    };

    return (diff.x * diff.x + diff.y * diff.y).sqrt();
}


pub fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, font: &Font) {
    let center = get_text_center(&text, Some(font), font_size as u16, 1.0, 0.0);
    draw_text(&text, x - center.x, y - center.y, font_size, WHITE);
}