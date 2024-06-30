use std::{cmp::{max, min}, default, fmt::format, mem::take, thread::spawn};

use macroquad::{prelude::*, rand};
use crate::{assets::Assets, colors::ColorPalette, enemy::*};

pub const DESIGN_WIDTH: f32 = 1600.;
pub const DESIGN_HEIGHT: f32 = 900.;

//TODO: Finish upgrade system
//TODO: separate upgrade vec and make a bool for chosen


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


pub enum GameState {
    MainMenu,
    Playing,
    Paused,
    Score,
}

pub struct Game {
    game_state: GameState,
    color_state: ColorState,
    high_score: i32,
    current_score: i32,
    start_added: bool,
    upg_added: bool,    
    assets: Assets,
    palettes: [ColorPalette; 2],
    palette: ColorPalette,
    wave_current: i32,
    wave_start: bool,
    enemy_spawn: Vec<SpawnEnemy>,
    enemies: Vec<Enemy>, // Box is for allocating to the heap
    enemy_list:  [Enemy; 2],
    bullets: Vec<Bullet>,
    collectibles: Vec<Collectibe>,
    upgrades: Vec<Collectibe>, 
    player: Player,
    upgrade_count: f32,

    menu_selected: i32,
    music_level: i32,
    effect_level: i32,

    switch_effect_t: f32,
    switch_effect_total: f32,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            color_state: ColorState::Primary,
            assets: Assets::default(),
            palette: ColorPalette::default(),
            start_added: false,
            upg_added: false,
            player: Player::default(),
            enemies: Vec::new(),
            enemy_spawn: Vec::new(),
            bullets: vec![],
            collectibles: Vec::new(),
            upgrades: Vec::new(),
            wave_current: 0,
            wave_start: false,
            high_score: 0,
            current_score: 0,
            game_state: GameState::MainMenu,

            menu_selected: 0,
            music_level: 10,
            effect_level: 10,

            upgrade_count: 3.0,

            switch_effect_t: 0.0,
            switch_effect_total: 0.1,

            palettes: [
                ColorPalette::default(),
                ColorPalette::create_from(ORANGE, GREEN)
            ],

            enemy_list: [
                Enemy { health: 10, x: 50.0, y: 50.0, size: 40.0, kind: EnemyType::FollowShootEnemy, attack_speed: 1.0, can_collide: true, ..Default::default()},
                Enemy { health: 10, x: 50.0, y: 50.0, size: 40.0, kind: EnemyType::FollowEnemy, can_collide: true, ..Default::default()},
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
    pub state: ColorState,
    pub kind: BulletType,
    pub hit: bool
}


impl Bullet {
    pub fn new(damage: i32, x: f32, y: f32, dx: f32, dy: f32, kind: BulletType) -> Bullet {
        Bullet {
            x,y,dx,dy,kind, damage,
            size: 6.0,
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
        let speed = 550.0;

        self.x += self.dx * speed * dt;
        self.y += self.dy * speed * dt;
    }
}

pub struct SpawnEnemy {
    pub x: f32,
    pub y: f32,
    pub spawn_t: f32,
    pub to_spawn: Enemy,
}

pub enum CollectibeKind {
    StartCube,
    DamageUp,
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
    attack_speed: f32,
    shoot_dx: f32,
    shoot_dy: f32,
    shoot_t: f32,
    melee_t: u32,

    heal_from_b: i32,
}

impl Default for Player {
    fn default() -> Player {
        Player {
            max_health: 10,
            health: 10,
            rotation: 0.0,
            size: 40.0,
            spread: 3.0,
            x: DESIGN_WIDTH/2.0,
            y: DESIGN_HEIGHT/4.0,
            dx: 0.0,
            dy: 0.0,
            shoot_dx: 1.0,
            shoot_dy: 0.0,
            attack_speed: 0.1,
            shoot_t: 0.0,
            melee_t: 0,

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

    pub fn level_bar(v: i32) -> String {
        let mut o = "O".repeat(v as usize);
        let dot = ".".repeat(10-v as usize);
        o.push_str(&dot);
        return o;
    }


    pub fn menu_update(&mut self) {
        if ( is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) )&& self.menu_selected == 0 {
            self.game_state = GameState::Playing;
        }


        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            self.menu_selected = max(0, self.menu_selected - 1)
        }

        if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            self.menu_selected = min(2, self.menu_selected + 1)
        }

        if self.menu_selected == 1 {
            if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) { 
                self.music_level = max(0, self.music_level - 1)
            }

            if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) { 
                self.music_level = min(10, self.music_level + 1)
            }
        }

        if self.menu_selected == 2 {
            if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) { 
                self.effect_level = max(0, self.effect_level - 1)
            }

            if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) { 
                self.effect_level = min(10, self.effect_level + 1)
            }
        }
    }

    pub fn menu_draw(&mut self) {
        clear_background(self.palette.BG_PRIMARY);
        let x_center = DESIGN_WIDTH/2.0;
        draw_text_centered("GAME TITLE ", x_center, 200.0, 130.0, &self.assets.font_monogram);
        draw_text_centered(&format!("Highscore: {} ", self.high_score), x_center, 260.0, 60.0, &self.assets.font_monogram);


        // PLAY
        // MUSIC
        // EFFECT

        match self.menu_selected {
            0 => {
                draw_text_centered("> Play < ", x_center, 540.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Music [{}] ", Game::level_bar(self.music_level)), x_center, 600.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Effects [{}] ", Game::level_bar(self.effect_level)), x_center, 660.0, 60.0, &self.assets.font_monogram);
            }
            1 => {
                draw_text_centered(" Play  ", x_center, 540.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("> Music [{}] < ", Game::level_bar(self.music_level)), x_center, 600.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Effects [{}] ", Game::level_bar(self.effect_level)), x_center, 660.0, 60.0, &self.assets.font_monogram);
            }
            2 => {
                draw_text_centered(" Play  ", x_center, 540.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("Music [{}] ", Game::level_bar(self.music_level)), x_center, 600.0, 60.0, &self.assets.font_monogram);
                draw_text_centered(&format!("> Effects [{}] < ", Game::level_bar(self.effect_level)), x_center, 660.0, 60.0, &self.assets.font_monogram);
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

    pub fn game_update(&mut self) {
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
                EnemyType::FollowShootEnemy => self.update_follow_shoot_enemy(e)
            }

            self.enemy_collision(e);

            e.health > 0
        });
        self.enemies = enemies;

        if self.enemies.len() == 0 && self.enemy_spawn.len() == 0 && self.wave_start {
            self.wave_current += 1;
            self.wave_start = false;

            // Spawn current wave
            self.enemy_spawn.push(
                SpawnEnemy {
                    x: 50.0,
                    y: 50.0,
                    spawn_t: 2.0,
                    to_spawn: self.enemy_list[0],
                }
            );
        }

        // Interact with arena middle
        if self.enemies.len() == 0 && self.enemy_spawn.len() == 0 && !self.wave_start {
            // start of the game, show controls and middle collectibe
            if self.wave_current == 0 {
                // ADd start cube
                if !self.start_added {
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
                    self.start_added = true;
                }
            } else {
                if !self.upg_added {
                    // Add upgrades
                    self.upg_added = true;
                    let padding = 70.0;
                    let upg_size = 100.0;
    
                    let total_size = upg_size * self.upgrade_count + padding * (self.upgrade_count - 1.0);
                    let start = total_size/2.0;


                    for i in 0..self.upgrade_count as i32 {
                        let center_x = DESIGN_WIDTH/2.0;
                        let x = center_x - start + i as f32*(upg_size + padding);
                        self.collectibles.push(
                            Collectibe {
                                x: x,
                                y: DESIGN_HEIGHT/2.0 - upg_size/2.0,
                                size: upg_size,
                                kind: CollectibeKind::DamageUp,
                                should_exist: true,
                            }
                        )
                    }
                }


            }
        }

        let mut collectibles = std::mem::take(&mut self.collectibles);
        collectibles.retain_mut(|c| {
            match c.kind {
                CollectibeKind::StartCube => self.update_start_cube(c),
                _ => self.update_upgrade(c),
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
                EnemyType::FollowShootEnemy => self.draw_follow_shoot_enemy(e)
            }
        }
        self.enemies = enemies;
        
        
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
                _ => self.draw_upgrades(c),
            }

            if self.wave_start {
                
            }
        };
        self.collectibles = collectibles;


        self.player_draw();
        
        let x_center = DESIGN_WIDTH / 2.0;
        let wave_txt = format!("Wave {} ", self.wave_current);
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


        if self.player.melee_t > 0 && self.player.melee_t < 15 {
            draw_circle(center_x, center_y, 45.0, YELLOW);
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
        let speed = 300.0;

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

        self.player.dx = dir.x * speed;
        self.player.dy = dir.y * speed;

        if self.player.shoot_t > 0.0 {
            self.player.shoot_t -= dt;
        }

        if is_key_down(KeyCode::J) || is_key_down(KeyCode::F) {
            if self.player.shoot_t <= 0.0 {

                // rotate by spread randomly
                let actual_spread = rand::gen_range(-self.player.spread, self.player.spread);
                let mut dir = rotate_vec(Vec2 { x: self.player.shoot_dx, y: self.player.shoot_dy}, actual_spread);
                dir = dir.normalize_or_zero();

                self.bullets.push(Bullet::new(
                    1, // damage
                    self.player.x + 15.0, // x
                    self.player.y + 15.0, // y
                    dir.x, // dx
                    dir.y, // dy
                    BulletType::Player, // kind
                ));

                self.player.shoot_t += self.player.attack_speed;
            }

        } else {
            if dir.x != 0.0 || dir.y != 0.0 {
                self.player.shoot_dx = dir.x;
                self.player.shoot_dy = dir.y; 
            }
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
                enemy.health -= 1;
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
            }
        }
    }
    // =========== ENEMIES =============

    // enemy and player collision
    pub fn enemy_collision(&mut self,e: &mut Enemy) {
        let hit = rect_collide(e.get_rect(), self.player.get_rect());

        if hit && e.can_collide {
            e.health = 0;
            self.player.health -= e.contact_damage;
        }
    }


    pub fn update_follow_enemy(&mut self,e: &mut Enemy) {
        let dt = get_frame_time();
        let dir = dir_to_player(e.x, e.y, &self.player);
        let speed = 150.0;

        e.x += dir.x * speed * dt;
        e.y += dir.y * speed * dt;
    }

    pub fn draw_follow_enemy(&mut self,e: &mut Enemy) {
        draw_rectangle(e.x, e.y, e.size, e.size, WHITE); 
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
                        Bullet::new(1,e.x + e.size/2.0, e.y + e.size/2.0, dir.x, dir.y, BulletType::Enemy)
                    );
                    e.attack_t = e.attack_speed;
                }

            }
        } 
    }

    pub fn draw_follow_shoot_enemy(&mut self,e: &mut Enemy) {
        draw_rectangle(e.x, e.y, e.size, e.size, YELLOW); 
    }



    // ========= Collectibles / UPGRADES =============

    pub fn update_start_cube(&mut self, c: &mut Collectibe) {
        if rect_collide(Rect{x: c.x, y: c.y, w: c.size, h: c.size}, self.player.get_rect()) {
            self.wave_start = true;
            c.should_exist = false;
            self.start_added = false;
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

        self.wave_start = true;

    
        match c.kind {
            CollectibeKind::DamageUp => {

            }
            _ => ()
        }
    }

    pub fn draw_upgrades(&mut self, c: &Collectibe) {
        // definitely not start cube or any other, upg logic matches
        // match upg with its texture, for now colored cube
        match c.kind {
            CollectibeKind::DamageUp => draw_rectangle(c.x, c.y, c.size, c.size, WHITE),
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
