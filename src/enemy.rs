use macroquad::prelude::*;
use crate::ColorState;
use crate::game::*;


pub enum Enemies {
    FollowEnemy(FollowEnemy),
    FollowShootEnemy(FollowShootEnemy)
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

pub struct FollowShootEnemy {
    pub health: i32,
    pub x: f32,
    pub y: f32,
}