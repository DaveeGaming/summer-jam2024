use macroquad::time::get_frame_time;

use crate::Game;

pub struct Bullet<'g> {
    game: &'g mut Game<'g>, 
    pub kind: BulletKind,
    pub x: f32,
    pub y: f32,
    pub dx: f32,
    pub dy: f32,
    pub is_alive: bool
}

pub enum BulletKind {
    PlayerBullet,
    EnemyBullet
}

pub fn update_player(Bullet {x, y, dx, dy, ..}: &mut Bullet) {

}

pub fn update_enemy(Bullet {x, y, dx, dy, ..}: &mut Bullet) {

}

impl Bullet<'_> {
    pub fn update(&mut self) {
        let dt = get_frame_time();

        match self.kind {
            BulletKind::PlayerBullet => update_player(self),
            BulletKind::EnemyBullet => update_enemy(self)
        }

        self.x += self.dx * dt;
        self.y += self.dy * dt; 
    }
}