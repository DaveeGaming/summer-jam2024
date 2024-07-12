use std::cmp::max;
use std::cmp::min;

use macroquad::prelude::*;
use macroquad::audio::*;

use crate::game::*;
use crate::player::Player;

#[derive(PartialEq, Eq)]
pub enum CharacterKind {
    Garry,
    BobBobBob,
    John,
    Mark,
    Locked
}
pub struct Character {
    pub p: Player,
    pub name: String,
    pub health: i32,
    pub damage: i32,
    pub speed: f32,
    pub kind: CharacterKind
}


impl Game {
    pub fn characters_update(&mut self) {
        self.background_update();
        let interact = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
        let up = is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up);
        let down = is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down);
        let left = is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left);
        let right = is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right);

        if left {
            self.selected_char = max(0, self.selected_char - 1);
            self.menu_switch();
        }
        if right {
            self.selected_char = min(self.characters.len() as i32 - 1, self.selected_char + 1);
            self.menu_switch();
        }
    }

    pub fn characters_draw(&mut self) {
        let x_center = DESIGN_WIDTH / 2.0;
        clear_background(BLACK);
        self.background_draw();

        let c = &self.characters[self.selected_char as usize];
        let texture = match c.kind {
            CharacterKind::Garry => &self.assets.garry,
            CharacterKind::BobBobBob => &self.assets.bobbobbob,
            CharacterKind::John => &self.assets.john,
            CharacterKind::Mark => &self.assets.mark,
            CharacterKind::Locked => &self.assets.locked,
        };

        draw_text_centered(&c.name, x_center, 100.0, 30.0, &self.assets.font_monogram);
        draw_texture(texture, x_center - 100.0, 200.0, WHITE);
        draw_text_centered("Easy", x_center, 500.0, 15.0, &self.assets.font_monogram);


        draw_text_ex("Stats", 100.0, 240.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 15, ..Default::default()});
        draw_text_ex("Health: 100", 100.0, 300.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 10, ..Default::default()});
        draw_text_ex("Damage: 1", 100.0, 340.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 10, ..Default::default()});
        draw_text_ex("Speed: 10", 100.0, 380.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 10, ..Default::default()});


        for i in 0..self.characters.len() {
            let c = &self.characters[i];
            
            let texture = match c.kind {
                CharacterKind::Garry => &self.assets.garry,
                CharacterKind::BobBobBob => &self.assets.bobbobbob,
                CharacterKind::John => &self.assets.john,
                CharacterKind::Mark => &self.assets.mark,
                CharacterKind::Locked => &self.assets.locked,
            };

            if i == self.selected_char as usize {
                let color = if c.kind == CharacterKind::Locked { Color::from_hex(0x4f4f4f)} else { WHITE };
                draw_text_ex("( )", 200.0 + (i as f32 * 150.0) - 38.0, 600.0, TextParams { font: Some(&self.assets.font_monogram), font_size: 33, color: color, ..Default::default()});
            }
            draw_texture_ex(texture, 200.0 + (i as f32 * 150.0), 600.0 , WHITE, 
                DrawTextureParams {
                    dest_size: Some(Vec2 { x: 80.0, y: 80.0 }),
                    ..Default::default()
                }
            )
        }
    }
}