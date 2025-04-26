use godot::{
    classes::{AnimatedSprite2D, Area2D, IArea2D},
    prelude::*,
};
use rand::{thread_rng, Rng};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct GellyEnemy {
    #[export]
    speed: f32,
    dir: Vector2,
    is_dead: bool,
    #[export]
    explosion_sound: Gd<AudioStreamPlayer>,
    #[export]
    point: i16,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for GellyEnemy {
    fn ready(&mut self) {
        let mut rng = thread_rng();

        self.dir = Vector2 {
            x: rng.gen_range(-0.5..=0.5),
            y: 1.,
        };
    }

    fn init(base: Base<Area2D>) -> Self {
        GellyEnemy {
            speed: 0.,
            dir: Vector2::ZERO,
            explosion_sound: AudioStreamPlayer::new_alloc(),
            is_dead: false,
            point: 1,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        if !self.is_dead {
            self.moviment(delta);
        }
    }
}

#[godot_api]
impl GellyEnemy {
    #[func]
    fn moviment(&mut self, delta: f64) {
        let mut pos = self.base().get_position();

        pos += self.dir * self.speed * delta as f32;

        self.base_mut().set_position(pos);
    }

    #[func]
    pub fn on_player_entered(&mut self, _body: Gd<Player>) {
        let mut player = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        player.bind_mut().death();
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

        self.explosion_sound.play();

        player.bind_mut().set_points(final_points);
    }
    #[func]
    fn on_sprite_2d_animation_finished(&mut self) {
        self.base_mut().queue_free()
    }
}
