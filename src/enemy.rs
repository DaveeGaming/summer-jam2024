use macroquad::prelude::*;
use crate::{player::*, ColorState};

pub trait Enemy {
    fn update(&mut self, p: &Player, s: &ColorState);
    fn draw(&mut self, s: &ColorState);
}


fn dir_to_player(x: f32, y: f32, p: &Player) -> Vec2 {
    let diff = Vec2 { 
        x: p.x - x,
        y: p.y - y,
    };

    return diff.normalize_or_zero();
}

fn distance_to_player(x: f32, y: f32, p: &Player) -> f32 {
    let diff = Vec2 { 
        x: p.x - x,
        y: p.y - y,
    };

    return (diff.x * diff.x + diff.y * diff.y).sqrt();
}
pub enum FollowEnemyType {
    ConstantSpeed(f32), // Constant speed
    ChangeSpeed(f32, f32) // Primary state speed | Secondary state speed
}

pub struct FollowEnemy {
    pub health: i32,
    pub x: f32,
    pub y: f32,
    pub typ: FollowEnemyType
}

impl FollowEnemy {
    pub fn new(health: i32, x: f32, y: f32, typ: FollowEnemyType) -> FollowEnemy{
        FollowEnemy {
            health, x, y, typ,
        }
    }
}


impl Enemy for FollowEnemy {
    fn update(&mut self, p: &Player, s: &ColorState) {
        let dt = get_frame_time();
        let dir = dir_to_player(self.x, self.y, p);

        let speed = match self.typ {
            FollowEnemyType::ConstantSpeed(s) => s,
            FollowEnemyType::ChangeSpeed(s1, s2) => if *s == ColorState::Primary { s1 } else { s2 }
        };

        self.x += dir.x * speed * dt;
        self.y += dir.y * speed * dt;

    }

    fn draw(&mut self, s: &ColorState) {
        draw_rectangle(self.x, self.y, 20.0, 20.0, WHITE); 
    }    
}

pub struct FollowShootEnemy {
    pub health: i32,
    pub x: f32,
    pub y: f32,
}

impl Enemy for FollowShootEnemy {
    fn update(&mut self, p: &Player, s: &ColorState) {
        match s {
            ColorState::Primary => {
                // Chase player
                let dt = get_frame_time();
                let dir = dir_to_player(self.x, self.y, p);
                let speed = 100.0;
                self.x += dir.x * speed * dt;
                self.y += dir.y * speed * dt;
            }
            ColorState::Secondary => {
                // Stop and shoot at player
                let dir = dir_to_player(self.x, self.y, p);
                // TODO: SHOOT XD
            }
        } 
    }

    fn draw(&mut self, s: &ColorState) {
        
    }
}