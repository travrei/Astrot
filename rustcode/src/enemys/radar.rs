use godot::{
    classes::{Area2D, IArea2D, Marker2D, Timer},
    prelude::*,
};

use crate::{bullets::EnemyBullet, player::Player};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct RadarEnemy {
    #[export]
    speed: f32,
    #[export]
    point: i8,
    #[export]
    finaltimer: Gd<Timer>,
    #[export]
    bullet_scene: Gd<PackedScene>,
    dir: Vector2,
    shooted: bool,
    is_dead: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for RadarEnemy {
    fn init(base: Base<Area2D>) -> Self {
        RadarEnemy {
            speed: 0.,
            point: 1,
            finaltimer: Timer::new_alloc(),
            dir: Vector2::ZERO,
            bullet_scene: PackedScene::new_gd(),
            shooted: false,
            is_dead: false,
            base,
        }
    }

    fn ready(&mut self) {
        self.dir.x = 1.;
    }

    fn process(&mut self, delta: f64) {
        if self.shooted {
            self.moviment(delta);
        }
    }
}

#[godot_api]
impl RadarEnemy {
    #[func]
    fn shoot(&mut self, body: Gd<Player>) {
        let player_pos = body.get_position();
        let radar_pos = self.base().get_position();

        let direction = (player_pos - radar_pos).normalized();

        let mut bullet = self.bullet_scene.instantiate_as::<EnemyBullet>();
        bullet.set_position(radar_pos);
        bullet.bind_mut().set_dir(direction);
        self.base_mut().get_parent().unwrap().add_child(bullet);
        self.finaltimer.start();
    }

    #[func]
    fn moviment(&mut self, delta: f64) {
        let mut pos = self.base().get_position();
        if pos.x > 120. {
            self.dir.x = 1.;
        } else if pos.x < 120. {
            self.dir.x = -1.;
        }
        pos += self.speed * self.dir * delta as f32;
        self.base_mut().set_position(pos);
    }

    #[func]
    fn timer_out(&mut self) {
        self.shooted = true;
        godot_print!("TENHO QUE IR EMBORA!")
    }
}
