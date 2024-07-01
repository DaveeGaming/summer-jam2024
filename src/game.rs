use std::{cmp::{max, min}, default, fmt::format, mem::take, sync::MutexGuard, thread::spawn};

use macroquad::{audio::{play_sound, play_sound_once, set_sound_volume, stop_sound, PlaySoundParams}, prelude::*, rand};
use quad_storage::LocalStorage;
use crate::{assets::Assets, colors::ColorPalette, enemy::*};

pub const DESIGN_WIDTH: f32 = 1600.;
pub const DESIGN_HEIGHT: f32 = 900.;

//TODO: Finish upgrade system
//TODO: SCore screen
//TODO: Web save data for sound level and high score
//TODO: make more upgrades,
//TODO: make a somewhat infinite scaling game
//TODO: make sprites for upgrades (draw or idk some shit)
//TODO: find a menu song and game song, maybe two, so it doesnt get so boring
//TODO: have sound levels in pause menu, quit button, and stats
//TODO: particle system for enemy death, and some enemy attacks
//TODO: have sound effects for dying, shooting,
//TODO: small upgrade hints
//TODO: lazer attack maybe?
//TODO: circle attack with radius check
//TODO: PLAYTEST WITH SOMEONE
//TODO: a bit interactive/visual main menu
//TODO: also a fucking game title lmao


#[derive(PartialEq, Eq, Clone, Copy)]
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


pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    Score,
}

pub enum WaveState {
    Start,
    Spawning
}

pub struct Unlocks {
    pub orangegreen: bool,
    pub purpleyellow: bool,
}

pub struct Wave {
    current: i32,
    state: WaveState,
    enemy_remaining: i32,
    upgrade_picked: bool,
    enemies_set: bool,

    move_player:bool,
    move_player_t: f32,
    move_player_tmax: f32,
    old_x: f32,
    old_y: f32,

    spawn_delay_t: f32,
    spawn_delay_tmax: f32,

    start_spawned: bool,
    upgrades_spawned: bool,
}

impl Wave {
    pub fn default() -> Self {
        Wave { 
            current: 0, 
            state: WaveState::Start, 
            enemy_remaining: 0, 
            enemies_set: false,
            start_spawned: false, 
            spawn_delay_tmax: 5.0,
            spawn_delay_t: 0.0,
            upgrades_spawned: false, 
            upgrade_picked: false,

            move_player: false,
            move_player_t: 0.0,
            move_player_tmax: 0.8,
            old_x: 0.0, old_y: 0.0,
        }
    }
}

pub struct Game {
    pub game_state: GameState,
    pub color_state: ColorState,
    pub unlocks: Unlocks,
    pub high_score: i32,
    pub current_score: i32,
    pub assets: Assets,
    pub should_save: bool,
    pub palettes: [ColorPalette; 3],
    pub palette: ColorPalette,
    pub curr_palette_idx: i32,
    pub enemy_spawn: Vec<SpawnEnemy>,
    pub enemies: Vec<Enemy>, // Box is for allocating to the heap
    pub enemy_list:  [Enemy; 4],
    pub upg_list: [CollectibeKind; 5],
    pub bullets: Vec<Bullet>,
    pub collectibles: Vec<Collectibe>,
    pub circle_attacks: Vec<CircleAttack>,
    pub upgrades: Vec<Collectibe>, 
    pub player: Player,
    pub wave: Wave,
    pub upgrade_count: f32,

    pub menu_selected: i32,
    pub music_level: i32,
    pub effect_level: i32,
    pub menu_song_started: bool,
    pub switch_effect_t: f32,
    pub switch_effect_total: f32,
}

impl Game {
    pub async fn default() -> Self {
        Game {
            color_state: ColorState::Primary,
            assets: Assets::default().await,
            should_save: false,
            palette: ColorPalette::default(),
            curr_palette_idx: 0,
            player: Player::default(),
            enemies: Vec::new(),
            enemy_spawn: Vec::new(),
            bullets: vec![],
            collectibles: Vec::new(),
            circle_attacks: Vec::new(),
            upgrades: Vec::new(),
            menu_song_started: false,
            high_score: 0,
            current_score: 0,
            game_state: GameState::MainMenu,
            unlocks: Unlocks { 
                orangegreen: false,
                purpleyellow: false,
            },

            menu_selected: 0,
            music_level: 10,
            effect_level: 10,

            upgrade_count: 3.0,

            switch_effect_t: 0.0,
            switch_effect_total: 0.01,

            wave: Wave::default(),

            upg_list: [
                CollectibeKind::Maxhp,
                CollectibeKind::Projectile,
                CollectibeKind::Size,
                CollectibeKind::Slowdmg,
                CollectibeKind::Speed,
            ],

            palettes: [
                ColorPalette::default(),
                ColorPalette::create_from(ORANGE, GREEN),
                ColorPalette::create_from(PURPLE, YELLOW)
            ],

            enemy_list: [
                Enemy { health: 5.0, x: 50.0, y: 50.0, size: 40.0, score: 15 , kind: EnemyType::FollowShootEnemy, attack_speed: 1.0, can_collide: true, ..Default::default()},
                Enemy { health: 10.0, x: 50.0, y: 50.0, size: 40.0, score: 30, kind: EnemyType::StaticCircleAttack, can_collide: true, contact_damage: 3, attack_t: 5.0, attack_speed: 5.0, ..Default::default()},
                Enemy { state: ColorState::Primary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
                Enemy { state: ColorState::Secondary,health: 2.0, x: 50.0, y: 50.0, size: 20.0, score: 5, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
            ]
        }    
    }
}

#[derive(PartialEq, Eq)]
pub enum BulletType {
    Player,
    Enemy
}

pub struct Bullet {
    pub damage: i32,
    pub x: f32,
    pub y: f32,
    pub last_x: f32,
    pub last_y: f32,
    pub dx: f32,
    pub dy: f32,
    pub size: f32,
    pub speed: f32,
    pub state: ColorState,
    pub kind: BulletType,
    pub hit: bool
}


impl Bullet {
    pub fn new(damage: i32, x: f32, y: f32, dx: f32, dy: f32, size: f32, speed: f32, kind: BulletType) -> Bullet {
        Bullet {
            x,y,dx,dy,kind, damage, size, speed,
            last_x: x,
            last_y: y,
            hit: false,
            state: ColorState::Primary,
        }
    }

    pub fn update(&mut self) {
        self.last_x = self.x;
        self.last_y = self.y;
        let dt = get_frame_time();

        self.x += self.dx * self.speed * dt;
        self.y += self.dy * self.speed * dt;
    }
}

pub struct CircleAttack {
    pub x: f32,
    pub y: f32,
    pub hit: bool,
    pub radius: f32,
    pub color: ColorState,
}

pub struct SpawnEnemy {
    pub x: f32,
    pub y: f32,
    pub spawn_t: f32,
    pub to_spawn: Enemy,
}

#[derive(Clone, Copy)]
pub enum CollectibeKind {
    StartCube,
    Maxhp,
    Projectile,
    Size,
    Slowdmg,
    Speed,
    Backshot
}

pub struct Collectibe {
    pub x: f32,
    pub y: f32,
    pub size: f32, 
    pub kind: CollectibeKind,
    pub should_exist: bool
}

impl Collectibe {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.x, y: self.y, w: self.size, h: self.size,
        }
    }
}


pub struct Player {
    pub max_health: i32,
    pub health: i32,
    rotation: f32,
    pub size: f32,
    pub x: f32,
    pub y: f32,
    dx: f32,
    dy: f32,
    spread: f32,
    projectiles: f32,
    attack_speed: f32,
    move_speed: f32,
    bullet_size: f32,
    bullet_speed: f32,
    damage: f32,
    shoot_dx: f32,
    shoot_dy: f32,
    shoot_t: f32,
    melee_t: f32,
    melee_range: f32,

    heal_from_b: i32,
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
    pub fn update(&mut self) {
        match self.game_state {
            GameState::MainMenu => self.menu_update(),
            GameState::Playing => self.game_update(),
            _ => ()
        }
    }

    pub fn draw(&mut self) {
        match self.game_state {
            GameState::MainMenu => self.menu_draw(),
            GameState::Playing => self.game_draw(),
            _ => ()
        }
    }




    // =========== MENU STATE ==============
    pub fn save_data(&mut self, s: &mut MutexGuard<LocalStorage>) {

        if self.should_save {
            s.set("highscore", &self.high_score.to_string());
            s.set("orangeyellow", &self.unlocks.orangegreen.to_string());
            s.set("purpleyellow", &self.unlocks.purpleyellow.to_string());
            s.set("sound_volume", &self.music_level.to_string());
            s.set("effect_volume", &self.effect_level.to_string());
        }
    }

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
                1 => if self.unlocks.orangegreen { self.palettes[1] } else { self.palettes[0] }
                2 => if self.unlocks.purpleyellow { self.palettes[2] } else { self.palettes[0] }
                _ => self.palettes[0],
            }
        }
    }

    pub fn menu_draw(&mut self) {
        clear_background(self.palette.BG_PRIMARY);
        let x_center = DESIGN_WIDTH/2.0;
        draw_texture(&self.assets.menu1, x_center - 40.0, 110.0, self.palette.FG_PRIMARY);
        draw_texture(&self.assets.menu2, x_center - 40.0, 110.0, self.palette.FG_SECONDARY);
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
                    1 => if self.unlocks.orangegreen { "Orange & Green" } else { "Reach wave 10 to unlock" }
                    2 => if self.unlocks.purpleyellow { "Purple & Yellow" } else { "Reach wave 25 to unlock" }
                    _ => "what"
                };
                draw_text_centered(&format!("> {} <", text), x_center, 720.0, 60.0, &self.assets.font_monogram);
            }
            _ => ()
        }
    }

    // =========== ENEMY SPAWN ============

    pub fn update_spawning(&mut self, s: &mut SpawnEnemy) {
        s.spawn_t -= get_frame_time();
        if s.spawn_t <= 0.0 {
            let mut enemy = s.to_spawn;
            enemy.x = s.x;
            enemy.y = s.y;
            self.enemies.push(enemy);
        }
    }

    pub fn draw_spawning(&mut self,s: &mut SpawnEnemy) {
        let mut color = match self.color_state {
            ColorState::Primary => self.palette.FG_PRIMARY,
            ColorState::Secondary => self.palette.FG_SECONDARY
        };

        color.a = 0.8;

        let spawn_size = 30.0;
        let spawn_time = 2.0;

        draw_rectangle_lines(s.x, s.y, spawn_size, spawn_size, 6.0, color);

        let curr_size = spawn_size * ((spawn_time - s.spawn_t) / spawn_time);
        draw_rectangle(
            s.x + spawn_size/2.0 - curr_size/2.0, 
            s.y + spawn_size/2.0 - curr_size/2.0, 
            curr_size, curr_size, color)

    }

    // =========== GAME STATE ==============

    // pub fn clean_map_and_move_player(&mut self) {
    //     self.bullets = Vec::new();
    //     self.player.x = DESIGN_WIDTH/2.0 - self.player.size/2.0;
    //     self.player.y = 600.0;
    // }

    pub fn move_player(&mut self) {
        self.bullets = Vec::new();
        self.circle_attacks = Vec::new();
        self.wave.move_player = true;
        self.wave.move_player_t = self.wave.move_player_tmax;
        self.wave.old_x = self.player.x;
        self.wave.old_y = self.player.y;
    }

    pub fn spawn_start_cube(&mut self) {
        let start_size = 100.0;
        let middle_rect = Rect{
            x: DESIGN_WIDTH/2.0 - start_size/2.0,
            y: DESIGN_HEIGHT/2.0 - start_size/2.0,
            w: start_size,
            h: start_size
        };
        self.collectibles.push(
            Collectibe { 
                x: middle_rect.x, 
                y: middle_rect.y, 
                size: middle_rect.w, 
                kind: CollectibeKind::StartCube, 
                should_exist: true }
        );
        self.wave.start_spawned = true;
    }

    pub fn spawn_upgrades(&mut self) {
        let padding = 120.0;
        let upg_size = 100.0;
        
        let total_size = upg_size * self.upgrade_count + padding * (self.upgrade_count - 1.0);
        let start = total_size/2.0;
        
        
        for i in 0..self.upgrade_count as i32 {
            let center_x = DESIGN_WIDTH/2.0;
            let x = center_x - start + i as f32*(upg_size + padding);
            self.upgrades.push(
                Collectibe {
                    x: x,
                    y: DESIGN_HEIGHT/2.0 - upg_size/2.0,
                    size: upg_size,
                    kind: self.upg_list[rand::gen_range(0, self.upg_list.len())],
                    should_exist: true,
                }
            )
        }
        self.move_player();
        self.wave.upgrades_spawned = true;
    }

    pub fn game_update(&mut self) {

        if self.wave.current >= 10 {
            self.unlocks.orangegreen = true;
            self.should_save = true;
        }

        if self.wave.current >= 25 {
            self.unlocks.purpleyellow = true;
            self.should_save = true;
        }

        if self.player.health <= 0 || is_key_pressed(KeyCode::Escape) {
            if self.current_score > self.high_score {
                self.high_score = self.current_score;
                self.should_save = true;
            }

            self.player = Player::default();
            self.wave = Wave::default();
            self.enemies = Vec::new();
            self.bullets = Vec::new();
            self.circle_attacks = Vec::new();
            self.collectibles = Vec::new();
            self.upgrades = Vec::new();
            self.enemy_spawn = Vec::new();
            self.current_score = 0;
            stop_sound(&self.assets.play_song);
            play_sound(&self.assets.menu_song, PlaySoundParams { looped: true, volume: self.music_level as f32 / 10.0 });
            self.game_state = GameState::MainMenu;
        }
        
        if self.wave.move_player {
            self.wave.move_player_t -= get_frame_time();
            
            let dest_x = DESIGN_WIDTH/2.0 - self.player.size/2.0;
            let dest_y = 700.0;
            
            let diff_x = dest_x - self.wave.old_x;
            let diff_y = dest_y - self.wave.old_y;
            
            if self.wave.move_player_t < 0.0 {
                self.wave.move_player = false;
                return;
            }
            
            let lerp_val = (self.wave.move_player_tmax - self.wave.move_player_t) / self.wave.move_player_tmax;
            self.player.x = self.wave.old_x + diff_x * lerp_val;
            self.player.y = self.wave.old_y + diff_y * lerp_val;


            return;
        }

        self.player_update();
        
        let mut spawners = std::mem::take(&mut self.enemy_spawn);
        spawners.retain_mut(|s| {
            self.update_spawning(s);
            
            s.spawn_t > 0.0
        }); 
        self.enemy_spawn = spawners;
        
        let mut enemies = std::mem::take(&mut self.enemies);
        enemies.retain_mut(|e| {
            if e.attack_t >= 0.0 {
                e.attack_t -= get_frame_time();
            }
            
            match e.kind {
                EnemyType::FollowEnemy => self.update_follow_enemy(e),
                EnemyType::FollowShootEnemy => self.update_follow_shoot_enemy(e),
                EnemyType::StaticCircleAttack => self.update_static_circle_enemy(e)
            }
            
            self.enemy_collision(e);
            
            if e.health <= 0.0 {
                self.current_score += e.score;   
                play_sound(&self.assets.dead, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 })
            }
            e.health > 0.0
        });
        self.enemies = enemies;
        

        let mut circles = std::mem::take(&mut self.circle_attacks);
        circles.retain_mut(|c| {
            c.radius += get_frame_time() * 500.0 * (c.radius/200.0);

            if !c.hit {
                // check player distance
                let diffx = (c.x - self.player.x + self.player.size/2.0).abs();
                let diffy = (c.y - self.player.y + self.player.size/2.0).abs();
    
                // player/circle distance
                let dist = (diffx * diffx + diffy * diffy).sqrt() - c.radius;
                
                // player has size, and circle has thickness
                let circle_pad = 2.5;
                let player_pad = self.player.size/2.0;
                
                let dist = dist.abs();
                if dist < circle_pad + player_pad || dist <= circle_pad - player_pad || dist <= player_pad - circle_pad || dist == circle_pad + player_pad {
                    // inside player
                    if self.color_state != c.color {
                        c.hit = true;
                        play_sound(&self.assets.hit, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
                        self.player.health -= 2;
                    } 
                }
            }

            c.radius < 2000.0
        });
        self.circle_attacks = circles;

        let mut upg = std::mem::take(&mut self.upgrades);
        upg.retain_mut(|u| {
            self.update_upgrade(u);

            !self.wave.upgrade_picked
        });
        self.upgrades = upg;


        let mut collectibles = std::mem::take(&mut self.collectibles);
        collectibles.retain_mut(|c| {
            match c.kind {
                CollectibeKind::StartCube => self.update_start_cube(c),
                _ => (),
            }

            c.should_exist
        });
        self.collectibles = collectibles;



        let mut bullets = std::mem::take(&mut self.bullets);
        bullets.retain_mut(|b| {
            b.update();
            self.bullet_collision(b);

            !b.hit
        });
        self.bullets = bullets;

        if is_key_pressed(KeyCode::C) {
            self.palette = self.palettes[ rand::gen_range(0, self.palettes.len()) ]
        }

        if is_key_pressed(KeyCode::Space){
            self.switch_effect_t = self.switch_effect_total;
            // cool circle effect
        }

        if is_key_pressed(KeyCode::B) {
            self.enemies.push( 
                self.enemy_list[1]
            );
        }


        if self.switch_effect_t >= 0.0 {
            self.switch_effect_t -= get_frame_time();
        }
        if self.switch_effect_t <= 0.0 && self.switch_effect_t > -1.0 {
            self.color_state = self.color_state.next();
            self.switch_effect_t = -2.0;
        }

        match self.wave.state {
            WaveState::Start => {
                if self.wave.current == 0 && !self.wave.start_spawned {
                    // ADd start cube
                    self.spawn_start_cube();
                } else {
                    if !self.wave.upgrades_spawned && self.wave.current > 0 {
                        self.spawn_upgrades();
                    }
                }
            },
            WaveState::Spawning => {
                // Wave started, everyting got defeated
                if !self.wave.enemies_set {
                    let enemies_to_spawn = 20 + self.wave.current * 3;
                    self.wave.enemy_remaining = enemies_to_spawn;
                    self.wave.spawn_delay_tmax = 5.0 - self.wave.current as f32 * 0.05;
                    self.wave.enemies_set = true;
                }

                self.wave.spawn_delay_t -= get_frame_time();
                if self.wave.spawn_delay_t > 0.0 && self.enemies.len() == 0 && self.enemy_spawn.len() == 0 {
                    self.wave.spawn_delay_t = 0.0;
                }

                if self.wave.spawn_delay_t <= 0.0 {
                    if self.wave.enemies_set && self.wave.enemy_remaining > 0 {
                        let to_spawn = min(5, self.wave.enemy_remaining);


                        let rad_x = rand::gen_range(50.0, DESIGN_WIDTH - 400.0);
                        let rad_y = rand::gen_range(50.0,  DESIGN_HEIGHT - 350.0);

                        for i in 0..to_spawn {
                            let x = rand::gen_range(0.0, 300.0);
                            let y = rand::gen_range(0.0, 300.0);
                            self.enemy_spawn.push(
                                SpawnEnemy {
                                    x: rad_x + x, y: rad_y + y,
                                    spawn_t: 2.0,
                                    to_spawn: self.enemy_list[ rand::gen_range(0, self.enemy_list.len())],
                                }
                            );
        
                            self.wave.enemy_remaining -= 1;
                        }
                        self.wave.spawn_delay_t = self.wave.spawn_delay_tmax;
                    }
                }



                if self.wave.enemy_remaining == 0 && self.enemy_spawn.len() == 0 && self.enemies.len() == 0 {
                    self.wave.state = WaveState::Start;
                    self.wave.current += 1;
                    self.wave.upgrade_picked = false;
                    self.wave.enemies_set = false;
                    self.wave.spawn_delay_t = 0.0;
                }
                
            }
        }
    }


    pub fn game_draw(&mut self) {
        let bg_color = match self.color_state {
            ColorState::Primary =>  self.palette.BG_PRIMARY,
            ColorState::Secondary => self.palette.BG_SECONDARY
        };

        let bg_color_invert = match self.color_state {
            ColorState::Secondary =>  self.palette.BG_PRIMARY,
            ColorState::Primary => self.palette.BG_SECONDARY
        };


        clear_background(bg_color);


        // draw switch effect before everything else
        if self.switch_effect_t > 0.0 {
            draw_circle(self.player.x, self.player.y, 
                2000.0 * (self.switch_effect_total - self.switch_effect_t) / self.switch_effect_total,
                bg_color_invert);
        }

        let mut enemies = std::mem::take(&mut self.enemies);
        for e in enemies.iter_mut() {
            match e.kind {
                EnemyType::FollowEnemy => self.draw_follow_enemy(e),
                EnemyType::FollowShootEnemy => self.draw_follow_shoot_enemy(e),
                EnemyType::StaticCircleAttack => self.draw_static_circle_enemy(e)
            }
        }
        self.enemies = enemies;
        
        let mut circles = std::mem::take(&mut self.circle_attacks);
        for c in circles.iter_mut() {
            let color = match c.color {
                ColorState::Primary => self.palette.FG_PRIMARY,
                ColorState::Secondary => self.palette.FG_SECONDARY
            };
            draw_circle_lines(c.x, c.y, c.radius, 5.0, color)
        }
        self.circle_attacks = circles;
        
        let mut bullets = std::mem::take(&mut self.bullets);
        for b in bullets.iter_mut() {
            self.bullet_draw(&b);
        }
        self.bullets = bullets;

        let mut spawners = std::mem::take(&mut self.enemy_spawn);
        for s in spawners.iter_mut() {
            self.draw_spawning(s);
        }
        self.enemy_spawn = spawners;

        let collectibles = std::mem::take(&mut self.collectibles);
        for c in collectibles.iter() {
            match c.kind {
                CollectibeKind::StartCube => self.draw_start_cube(c),
                _ => (),
            }
        };
        self.collectibles = collectibles;

        let mut upg = std::mem::take(&mut self.upgrades);
        for u in upg.iter_mut() {
            self.draw_upgrades(u)
        }

        upg.retain(|u| !self.wave.upgrade_picked);
        self.upgrades = upg;

        self.player_draw();
        
        let x_center = DESIGN_WIDTH / 2.0;
        let wave_txt = format!("Wave {} ", self.wave.current);
        draw_text_centered(&wave_txt, x_center, 70.0, 90.0, &self.assets.font_monogram);
        let score = format!("score: {} ", self.current_score);
        draw_text_centered(&score, x_center, 100.0, 40.0, &self.assets.font_monogram);
    }





    // ================ PLAYER ====================


    pub fn draw_healthbar(&self) {
        let color = match self.color_state {
            ColorState::Primary => self.palette.FG_PRIMARY,
            ColorState::Secondary => self.palette.FG_SECONDARY
        };

        let width = 8.0 * self.player.max_health as f32;
        let height = 20.0;
        let frame_thick = 5.0;
        let padding = 6.0;

        let top_right_x = self.player.x + self.player.size/2.0 - (width as f32)/2.0;
        let top_right_y = self.player.y + self.player.size + padding;

        draw_rectangle_lines(
            top_right_x, top_right_y,
            width, height, frame_thick, color
        );

        draw_rectangle(
            top_right_x + padding, 
            top_right_y + padding, 
            (width - 2.0*padding) * self.player.health as f32 / self.player.max_health as f32, 
            height-2.0*padding, color
        )


    }

    pub fn player_draw(&self) {
        let color = match self.color_state {
            ColorState::Primary => self.palette.FG_PRIMARY,
            ColorState::Secondary => self.palette.FG_SECONDARY
        };

        let center_x = self.player.x + self.player.size/2.0;
        let center_y = self.player.y + self.player.size/2.0;

        let mut melee_color = color.clone();
        melee_color.a = 0.5;


        if self.player.melee_t > 1.5 && self.player.melee_t < 2.0 {
            draw_circle(center_x, center_y, self.player.melee_range, melee_color);
        }

        draw_rectangle_ex(self.player.x, self.player.y, self.player.size, self.player.size,
            DrawRectangleParams {
                color: color,
                rotation: self.player.rotation,
                ..Default::default()
            }
        );

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

        if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
            dir.x = -1.0;
        }
        if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
            dir.x = 1.0;
        }
        if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
            dir.y = -1.0;
        }
        if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
            dir.y = 1.0;
        }

        dir = dir.normalize_or_zero();

        self.player.dx = dir.x * self.player.move_speed;
        self.player.dy = dir.y * self.player.move_speed;

        if self.player.shoot_t > 0.0 {
            self.player.shoot_t -= dt;
        }

        if is_key_down(KeyCode::J) || is_key_down(KeyCode::F) {
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

        } else {
            if dir.x != 0.0 || dir.y != 0.0 {
                self.player.shoot_dx = dir.x;
                self.player.shoot_dy = dir.y; 
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

        self.player.dx *= 0.85;
        self.player.dy *= 0.85;

    }

    // ========== BULLET ============


    pub fn bullet_draw(&mut self, b: &Bullet) {
        if b.kind == BulletType::Player {
            draw_circle(b.x, b.y, b.size, WHITE);
        } else {
            let color = match b.state {
                ColorState::Primary => self.palette.FG_PRIMARY,
                ColorState::Secondary => self.palette.FG_SECONDARY,
            };
            draw_circle(b.x, b.y, b.size, color);
        }
    }

    pub fn bullet_collision(&mut self, b: &mut Bullet) {
        if b.x < 0.0 || b.y < 0.0 || b.x > DESIGN_WIDTH || b.y > DESIGN_HEIGHT {
            b.hit = true;
            return;
        }


        match b.kind {
            BulletType::Player => self.bullet_enemy_coll(b),
            BulletType::Enemy => self.bullet_player_coll(b)
        }
    }

    // player bullet collides with enemy
    pub fn bullet_enemy_coll(&mut self, b: &mut Bullet) {
        let mut enemies = std::mem::take(&mut self.enemies);
        for enemy in enemies.iter_mut() {
            let hit = rect_collide(
                Rect{
                    x: b.x - b.size,
                    y: b.y - b.size,
                    w: b.size * 2.0,
                    h: b.size * 2.0,
                },enemy.get_rect()); 
            
            if hit {
                b.hit = hit;
                enemy.health -= self.player.damage;
            }
        }
        self.enemies = enemies;
    }

    // enemy bullet collides with player
    pub fn bullet_player_coll(&mut self, b: &mut Bullet) {
        let hit = rect_collide(
            self.player.get_rect(),
            Rect{
                x: b.x - b.size,
                y: b.y - b.size,
                w: b.size * 2.0,
                h: b.size * 2.0,
            }
        );

        if hit {
            b.hit = hit;
            if b.state == self.color_state {
                self.player.health = min(self.player.max_health, self.player.health + self.player.heal_from_b)
            } else {
                self.player.health -= b.damage;
                play_sound(&self.assets.hit, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
            }
        }
    }
    // =========== ENEMIES =============

    // enemy and player collision
    pub fn enemy_collision(&mut self,e: &mut Enemy) {
        let hit = rect_collide(e.get_rect(), self.player.get_rect());



        if hit && e.can_collide {
            if e.kind == EnemyType::FollowEnemy {
                if e.state == self.color_state {
                self.player.health = min(self.player.max_health, self.player.health + self.player.heal_from_b)
                } else {
                    self.player.health -= 1;
                }
            } else {
                self.player.health -= e.contact_damage;
                play_sound(&self.assets.hit, PlaySoundParams { looped: false, volume: self.effect_level as f32 / 10.0 });
            }
            e.health = 0.0;
        }
    }


    pub fn update_follow_enemy(&mut self,e: &mut Enemy) {
        let dt = get_frame_time();
        let dir = dir_to_player(e.x, e.y, &self.player);
        let speed = 250.0;

        e.x += dir.x * speed * dt;
        e.y += dir.y * speed * dt;
    }

    pub fn draw_follow_enemy(&mut self,e: &mut Enemy) {
        let color = match e.state {
            ColorState::Primary => self.palette.FG_PRIMARY,
            ColorState::Secondary => self.palette.FG_SECONDARY,
        };

        draw_rectangle(e.x, e.y, e.size, e.size, color); 
    }

    pub fn update_follow_shoot_enemy(&mut self, e: &mut Enemy) {
        match self.color_state {
            ColorState::Primary => {
                // Chase player
                let dt = get_frame_time();
                let dir = dir_to_player(e.x, e.y, &self.player);
                let speed = 200.0;
                e.x += dir.x * speed * dt;
                e.y += dir.y * speed * dt;
            }
            ColorState::Secondary => {
                // Stop and shoot at player
                if e.attack_t <= 0.0 {
                    let dir = dir_to_player(e.x, e.y, &self.player);
                    self.bullets.push(
                        Bullet::new(1,e.x + e.size/2.0, e.y + e.size/2.0, dir.x, dir.y, 6.0, 550.0, BulletType::Enemy)
                    );
                    e.attack_t = e.attack_speed;
                }

            }
        } 
    }

    pub fn draw_follow_shoot_enemy(&mut self,e: &mut Enemy) {
        draw_texture(&self.assets.shooter, e.x, e.y, WHITE);
        // draw_rectangle(e.x, e.y, e.size, e.size, WHITE); 
    }

    pub fn update_static_circle_enemy(&mut self,e: &mut Enemy) {
        let state = self.color_state.next();

        e.attack_t -= get_frame_time();
        if e.attack_t <= 0.0 {
            self.circle_attacks.push(
                CircleAttack { 
                    x: e.x + e.size/2.0, 
                    y: e.y + e.size/2.0, 
                    radius: 1.0, 
                    color: state,
                    hit: false,
                }
            );
            e.attack_t = e.attack_speed;
        }
    }

    pub fn draw_static_circle_enemy(&mut self,e: &mut Enemy) {
        draw_texture(&self.assets.tower, e.x, e.y, WHITE);
        // draw_rectangle(e.x, e.y, e.size, e.size, YELLOW); 
    }










    // ========= Collectibles / UPGRADES =============

    pub fn update_start_cube(&mut self, c: &mut Collectibe) {
        if rect_collide(Rect{x: c.x, y: c.y, w: c.size, h: c.size}, self.player.get_rect()) {
            self.wave.state = WaveState::Spawning;
            c.should_exist = false;
            self.wave.start_spawned = false;
        }
    }

    pub fn draw_start_cube(&mut self, c: &Collectibe) {
        draw_texture(&self.assets.start_cube, c.x, c.y, WHITE);
    }

    pub fn update_upgrade(&mut self, c: &mut Collectibe) {
        let hit = rect_collide(self.player.get_rect(), c.get_rect());
        if !hit {
            return; // early return, dotn care didnt ask + l + ratio
        }

        self.wave.state = WaveState::Spawning;
        self.wave.upgrades_spawned = false;
        self.wave.upgrade_picked = true;

    
        match c.kind {
            CollectibeKind::Maxhp => {
                self.player.max_health += 1;
            }
            CollectibeKind::Projectile => {
                let new_dmg = self.player.damage - 1.0;
                self.player.projectiles += 1.0;
                self.player.damage = if new_dmg <= 0.5 { 0.5 } else { new_dmg };
                self.player.spread += 3.0;
            },
            CollectibeKind::Size =>{
                self.player.bullet_size += 0.5;
            }
            CollectibeKind::Speed => {
                self.player.move_speed += 30.0;
            }
            CollectibeKind::Slowdmg => {
                self.player.damage += 1.0;
                self.player.bullet_speed -= 30.0;
            },
            _ => ()
        }
    }

    pub fn draw_upgrades(&mut self, c: &Collectibe) {
        // definitely not start cube or any other, upg logic matches
        // match upg with its texture, for now colored cube
        match c.kind {
            CollectibeKind::Maxhp => {
                draw_texture(&self.assets.maxhp, c.x, c.y, WHITE);
                draw_text_ex("+ maxhp", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            CollectibeKind::Projectile => {
                draw_texture(&self.assets.projectile, c.x, c.y, WHITE);
                draw_text_ex("+ shot", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
                draw_text_ex("+ spread", c.x - 40.0, c.y + 160.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
                draw_text_ex("- dmg", c.x - 40.0, c.y + 190.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
            },
            CollectibeKind::Speed => {
                draw_texture(&self.assets.speed, c.x, c.y, WHITE);
                draw_text_ex("+ speed", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            CollectibeKind::Size => {
                draw_texture(&self.assets.size, c.x, c.y, WHITE);
                draw_text_ex("+ size", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            CollectibeKind::Slowdmg => {
                draw_texture(&self.assets.slowdmg, c.x, c.y, WHITE);
                draw_text_ex("+ dmg", c.x - 40.0, c.y + 130.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                );
                draw_text_ex("- shot speed", c.x - 40.0, c.y + 160.0, 
                    TextParams { 
                        font: Some(&self.assets.font_monogram), 
                        font_size: 50,
                        ..Default::default()
                    }
                )
            },
            _ => ()
        }
    }
}



// =========== UTILS ============

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
    let size = measure_text(&text, Some(&font), font_size as u16, 1.0);
    draw_text(&text, x - size.width/2.0, y, font_size, WHITE);
}

// Takes degrees
pub fn rotate_vec(v: Vec2, d: f32) -> Vec2 {
    let rad = d.to_radians();
    let mut dir = Vec2::ZERO;
    dir.x = v.x * rad.cos() - v.y * rad.sin();
    dir.y = v.x * rad.sin() + v.y * rad.cos();
    return dir
}

pub fn rect_collide(r1: Rect, r2: Rect) -> bool {
    return r1.x< r2.x + r2.w
    && r1.x + r1.w > r2.x
    && r1.y < r2.y + r2.h
    && r1.y + r1.h > r2.y;
}
