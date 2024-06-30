use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum EnemyType {
    FollowEnemy,
    FollowShootEnemy,
}

#[derive(Clone, Copy)]
pub struct Enemy {
    pub health: i32,
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub kind: EnemyType,
    pub attack_speed: f32,
    pub can_collide: bool,
    pub contact_damage: i32,
    pub attack_t: f32,
}

impl Default for Enemy {
    fn default() -> Self {
        Self {
            health: 10,
            x: 50.0,
            y: 50.0,
            size: 40.0,
            can_collide: false,
            kind: EnemyType::FollowEnemy,
            attack_speed: 0.0,
            attack_t: 0.0,
            contact_damage: 2,
        }
    }
}

impl Enemy {
    pub fn get_rect(&self) -> Rect {
        Rect {
            x: self.x,
            y: self.y,
            w: self.size,
            h: self.size,
        }
    }
}
