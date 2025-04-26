use godot::{
    classes::{AnimatedSprite2D, Area2D, IArea2D, Marker2D, Timer},
    prelude::*,
};

use crate::{bullets::EnemyBullet, player::Player};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct RadarEnemy {
    #[export]
    speed: f32,
    #[export]
    point: i16,
    #[export]
    finaltimer: Gd<Timer>,
    #[export]
    bullet_scene: Gd<PackedScene>,
    #[export]
    shoot_sound: Gd<AudioStreamPlayer>,
    #[export]
    explosion_sound: Gd<AudioStreamPlayer>,
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
            shoot_sound: AudioStreamPlayer::new_alloc(),
            explosion_sound: AudioStreamPlayer::new_alloc(),
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
        self.shoot_sound.play();
        self.finaltimer.start();
    }

    #[func]
    fn hit(&mut self, _area: Gd<Area2D>) {
        let mut anim = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        self.is_dead = true;
        anim.set_animation("explo");

        self.base_mut()
            .set_deferred("monitoring", &false.to_variant());
        self.base_mut()
            .set_deferred("monitorable", &false.to_variant());

        self.explosion_sound.play();

        let mut player = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let player_points = player.bind().get_points();

        let final_points = player_points + self.point;

        player.bind_mut().set_points(final_points);
    }

    #[func]
    fn on_sprite_2d_animation_finished(&mut self) {
        self.base_mut().queue_free()
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
    }
}
