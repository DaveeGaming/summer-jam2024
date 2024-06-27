use macroquad::{prelude::*, rand};
use rand::rand;
use crate::{colors::ColorPalette, player::*, enemy::*};

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
    palettes: [ColorPalette; 2],
    palette: ColorPalette,
    enemies: Vec<Box<dyn Enemy>>, // Box is for allocating to the heap
    player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            state: ColorState::Primary,
            palette: ColorPalette::default(),
            player: Player::default(),
            enemies: Vec::new(),


            palettes: [
                ColorPalette::default(),
                ColorPalette::create_from(ORANGE, GREEN)
            ]
        }    
    }
}

impl Game {
    pub fn update(&mut self) {
        self.player.input();
        self.player.update();
        for enemy in self.enemies.iter_mut() {
            enemy.update(&self.player, &self.state);
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
            enemy.draw();
        }

        self.player.draw(&self.palette, &self.state);

    }
}