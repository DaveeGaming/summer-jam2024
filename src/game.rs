use macroquad::{prelude::*, rand};
use crate::{assets::Assets, colors::ColorPalette, enemy::*, player::*, DESIGN_WIDTH};

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
    player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            state: ColorState::Primary,
            assets: Assets::default(),
            palette: ColorPalette::default(),
            player: Player::default(),
            enemies: Vec::new(),
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
        self.player.update();
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

    pub fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, font: &Font) {
        let center = get_text_center(&text, Some(font), font_size as u16, 1.0, 0.0);
        draw_text(&text, x - center.x, y - center.y, font_size, WHITE);
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
        
        self.player.draw(&self.palette, &self.state);
        
        let x_center = DESIGN_WIDTH / 2.0;
        let wave_txt = format!("Wave {}", self.wave_current);
        Game::draw_text_centered(&wave_txt, x_center, 20.0, 40.0, &self.assets.font_monogram);
    }
}