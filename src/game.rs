use macroquad::{prelude::*, rand};
use rand::rand;
use crate::{colors::ColorPalette, player::*};


pub struct Game {
    palettes: [ColorPalette; 2],
    palette: ColorPalette,
    player: Player,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            palette: ColorPalette::default(),
            player: Player::default(),


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

        if is_key_pressed(KeyCode::C) {
            self.palette = self.palettes[ rand::gen_range(0, self.palettes.len()) ]
        }
    }

    pub fn draw(&mut self) {
        let bg_color = match self.player.state {
            ColorState::Primary =>  self.palette.BG_PRIMARY,
            ColorState::Secondary => self.palette.BG_SECONDARY
        };

        clear_background(bg_color);

        self.player.draw(&self.palette);
    }
}