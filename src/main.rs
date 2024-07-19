
// FEATURE CREEP CORNER
// Weapon for diff color ships
// Bullethell kinda stuff


//TODO: Change shooting to be on the mouse
//TODO: Crosshair color for palettes
//TODO: Better difficulty curve
//TODO: Melee attack
//TODO: Particle system
//TODO: Abilities like shield and stuff
//TODO: instead of upgrade, heal or reroll
//TODO: Multiple difficulty levels, 

use macroquad::prelude::*;
use crate::game::*;

mod game;
mod enemy;
mod collision;
mod player;
mod collection;
mod bullet;
mod colors;
mod characters;
mod assets;
mod options;
mod wave;
mod menu;
mod collectibe;

use miniquad::window::screen_size;

#[macroquad::main("title")]
async fn main() {
    let mut game = Game::default().await;
    let time = miniquad::date::now();

    // Browser storage handling
    let storage = &mut quad_storage::STORAGE.lock().unwrap();    

    
    let highscore = storage.get("highscore");
    if highscore.is_none() {
        storage.set("highscore", &0.to_string());
    } else {
        let highscore = highscore.unwrap();
        game.high_score = highscore.parse::<i32>().unwrap();
    }

    

    let sound = storage.get("sound_volume");
    if sound.is_none() {
        storage.set("sound_volume", &3.to_string());
    } else {
        let sound = sound.unwrap();
        game.music_level = sound.parse::<i32>().unwrap();
    }

    let effect = storage.get("effect_volume");
    if effect.is_none() {
        storage.set("effect_volume", &3.to_string());
    } else {
        let effect = effect.unwrap();
        game.effect_level = effect.parse::<i32>().unwrap();
    }



    let orangeyellow = storage.get("orangeyellow");
    if orangeyellow.is_none() {
        storage.set("orangeyellow", &false.to_string());
    } else {
        let orangeyellow = orangeyellow.unwrap();
        game.unlocks.orangegreen = orangeyellow.parse::<bool>().unwrap();
    }
    
    let purpleyellow = storage.get("purpleyellow");
    if purpleyellow.is_none() {
        storage.set("purpleyellow", &false.to_string());
    } else {
        let purpleyellow = purpleyellow.unwrap();
        game.unlocks.purpleyellow = purpleyellow.parse::<bool>().unwrap();
    }
    macroquad::rand::srand(time as u64);
    let canvas = render_target(DESIGN_WIDTH as u32, DESIGN_HEIGHT as u32);
    canvas.texture.set_filter(FilterMode::Nearest);
    loop {

        let (screen_w, screen_h) = screen_size();
        
        let scale = f32::min(screen_w / DESIGN_WIDTH, screen_h / DESIGN_HEIGHT);
        
        let x_center = (screen_w - DESIGN_WIDTH * scale) / 2.0;
        let y_center = (screen_h - DESIGN_HEIGHT * scale) / 2.0;
        
        let mut camera = Camera2D::from_display_rect(Rect {
            x: 0.,
            y: 0.,
            w: (DESIGN_WIDTH) as f32,
            h: (DESIGN_HEIGHT) as f32   
        });

        camera.render_target = Some(canvas.clone());
        camera.zoom.y = -camera.zoom.y;
        
        set_camera(&camera);
        game.update();
        game.draw();
        game.save_data(storage);

        set_default_camera();
        clear_background(BLANK);
        draw_texture_ex(&canvas.texture, x_center, y_center, WHITE, 
            DrawTextureParams {
                dest_size: Some( Vec2 { x: DESIGN_WIDTH * scale, y: DESIGN_HEIGHT * scale  }),
                ..Default::default()
            }
        );
        
        next_frame().await
    }
}
