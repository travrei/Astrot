use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct PlayerBullet {
    #[export]
    speed: f32,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PlayerBullet {
    fn init(base: Base<Area2D>) -> Self {
        PlayerBullet { speed: 0., base }
    }

    fn process(&mut self, delta: f64) {
        let mut dir = self.base().get_position();

        dir.y -= self.speed * delta as f32;

        self.base_mut().set_position(dir);
    }
}

#[godot_api]
impl PlayerBullet {
    #[func]
    fn on_area_entered(&mut self, _area: Gd<Area2D>) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_exit_screen(&mut self) {
        self.base_mut().queue_free();
    }
}

#[derive(GodotClass)]
#[class(base=Area2D)]
struct PlayerBulletLV2 {
    #[export]
    speed: f32,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PlayerBulletLV2 {
    fn init(base: Base<Area2D>) -> Self {
        PlayerBulletLV2 { speed: 0., base }
    }

    fn process(&mut self, delta: f64) {
        let mut dir = self.base().get_position();

        dir.y -= self.speed * delta as f32;

        self.base_mut().set_position(dir);
    }
}

#[godot_api]
impl PlayerBulletLV2 {
    #[func]
    fn on_area_entered(&mut self, _area: Gd<Area2D>) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_exit_screen(&mut self) {
        self.base_mut().queue_free();
    }
}

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PlayerBulletLV3 {
    #[export]
    speed: f32,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PlayerBulletLV3 {
    fn init(base: Base<Area2D>) -> Self {
        PlayerBulletLV3 { speed: 0., base }
    }
    fn process(&mut self, delta: f64) {
        let mut dir = self.base().get_position();

        dir.y -= self.speed * delta as f32;

        self.base_mut().set_position(dir);
    }
}

#[godot_api]
impl PlayerBulletLV3 {
    #[func]
    fn on_exit_screen(&mut self) {
        self.base_mut().queue_free();
    }
}

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct EnemyBullet {
    #[var]
    dir: Vector2,
    speed: f32,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for EnemyBullet {
    fn init(base: Base<Area2D>) -> Self {
        EnemyBullet {
            dir: Vector2::ZERO,
            speed: 100.,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut pos = self.base().get_position();

        pos += self.dir * self.speed * delta as f32;

        self.base_mut().set_position(pos);
    }
}

#[godot_api]
impl EnemyBullet {
    #[func]
    fn exit_screen(&mut self) {
        self.base_mut().queue_free();
    }

    #[func]
    fn on_player_entered(&mut self, mut player: Gd<Player>) {
        player.bind_mut().death();
    }
}
