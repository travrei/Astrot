use godot::{
    classes::{Area2D, CharacterBody2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct PlayerBullet {
    speed: f32,

    base: Base<Area2D>,
}

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct EnemyBullet {
    speed: f32,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PlayerBullet {
    fn init(base: Base<Area2D>) -> Self {
        PlayerBullet { speed: 500., base }
    }
    fn physics_process(&mut self, delta: f64) {
        let mut pos = self.base().get_position();

        pos.y -= self.speed * delta as f32;

        self.base_mut().set_position(pos)
    }
}
#[godot_api]
impl PlayerBullet {
    #[func]
    fn _on_visible_on_screen_notifier_2d_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}

#[godot_api]
impl IArea2D for EnemyBullet {
    fn init(base: Base<Area2D>) -> Self {
        EnemyBullet { speed: 5000., base }
    }
}
