use std::cmp::max;
use std::cmp::min;

use macroquad::prelude::*;
use macroquad::audio::*;

use crate::game::*;

impl Game {

    pub fn level_bar(v: i32) -> String {
        let mut o = "O".repeat(v as usize);
        let dot = ".".repeat(10-v as usize);
        o.push_str(&dot);
        return o;
    }


    pub fn menu_update(&mut self) {
        if !self.menu_song_started {
            play_sound(&self.assets.menu_song, PlaySoundParams { looped: true, volume: self.music_level as f32 / 10.0});
            self.menu_song_started = true;
        }


        if ( is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) )&& self.menu_selected == 0 {
            self.game_state = GameState::Playing;
            stop_sound(&self.assets.menu_song);
            play_sound(&self.assets.play_song, PlaySoundParams { looped: true, volume: self.music_level as f32 / 10.0});
            play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
        }

        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            self.menu_selected = max(0, self.menu_selected - 1);
            play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
        }

        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            self.menu_selected = min(3, self.menu_selected + 1);
            play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
        }

        if self.menu_selected == 1 {
            if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) { 
                self.music_level = max(0, self.music_level - 1);
                set_sound_volume(&self.assets.menu_song, self.music_level as f32 / 10.0);
                play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
                self.should_save = true;
            }

            if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) { 
                self.music_level = min(10, self.music_level + 1);
                set_sound_volume(&self.assets.menu_song, self.music_level as f32 / 10.0);
                play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
                self.should_save = true;
            }
        }

        if self.menu_selected == 2 {
            if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) { 
                self.effect_level = max(0, self.effect_level - 1);
                play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
                self.should_save = true;
            }

            if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) { 
                self.effect_level = min(10, self.effect_level + 1);
                play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
                self.should_save = true;
            }
        }

        if self.menu_selected == 3 {
            if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) { 
                self.curr_palette_idx -= 1;
                if self.curr_palette_idx < 0 { 
                    self.curr_palette_idx = self.palettes.len() as i32 - 1;
                }
                play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
            }

            if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) { 
                self.curr_palette_idx += 1;
                if self.curr_palette_idx > self.palettes.len() as i32 - 1 { 
                    self.curr_palette_idx = 0;
                }
                play_sound(&self.assets.menu_switch, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0});
            }
            
            self.palette = match self.curr_palette_idx {
                1 => if true { self.palettes[1] } else { self.palettes[0] }
                2 => if true { self.palettes[2] } else { self.palettes[0] }
                _ => self.palettes[0],
            }
        }
    }

    pub fn menu_draw(&mut self) {
        clear_background(self.palette.bg_primary);
        let x_center = DESIGN_WIDTH/2.0;
        draw_texture(&self.assets.menu1, x_center - 40.0, 110.0, self.palette.fg_primary);
        draw_texture(&self.assets.menu2, x_center - 40.0, 110.0, self.palette.fg_secondary);
        draw_text_centered("COLOR   SWITCH ", x_center, 200.0, 130.0, &self.assets.font_monogram);
        draw_text_centered(&format!("Highscore: {} ", self.high_score), x_center, 260.0, 60.0, &self.assets.font_monogram);


        // PLAY
        // MUSIC
        // EFFECT
        // PALETTE

        match self.menu_selected {
            0 => {
                draw_text_centered("> Play < ", x_center, 540.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Music [{}] ", Game::level_bar(self.music_level)), x_center, 600.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Effects [{}] ", Game::level_bar(self.effect_level)), x_center, 660.0, 60.0, &self.assets.font_monogram);
                draw_text_centered("Color palette ", x_center, 720.0, 60.0, &self.assets.font_monogram);
            }
            1 => {
                draw_text_centered(" Play  ", x_center, 540.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("> Music [{}] < ", Game::level_bar(self.music_level)), x_center, 600.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Effects [{}] ", Game::level_bar(self.effect_level)), x_center, 660.0, 60.0, &self.assets.font_monogram);
                draw_text_centered("Color palette ", x_center, 720.0, 60.0, &self.assets.font_monogram);
            }
            2 => {
                draw_text_centered(" Play  ", x_center, 540.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Music [{}] ", Game::level_bar(self.music_level)), x_center, 600.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("> Effects [{}] < ", Game::level_bar(self.effect_level)), x_center, 660.0, 60.0, &self.assets.font_monogram);
                draw_text_centered("Color palette ", x_center, 720.0, 60.0, &self.assets.font_monogram);
            },
            3 => {
                draw_text_centered(" Play  ", x_center, 540.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Music [{}] ", Game::level_bar(self.music_level)), x_center, 600.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Effects [{}] ", Game::level_bar(self.effect_level)), x_center, 660.0, 60.0, &self.assets.font_monogram);
                let text = match self.curr_palette_idx {
                    0 => "Red & Blue",
                    1 => if true { "Orange & Green" } else { "Reach wave 10 to unlock" }
                    2 => if true { "Purple & Yellow" } else { "Reach wave 25 to unlock" }
                    _ => "what"
                };
                draw_text_centered(&format!("> {} <", text), x_center, 720.0, 60.0, &self.assets.font_monogram);
            }
            _ => ()
        }
    }
}