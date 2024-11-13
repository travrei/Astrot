use godot::{
    classes::{AnimatedSprite2D, Area2D, IPathFollow2D, PathFollow2D},
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=PathFollow2D)]
struct FollowPathEnemy {
    #[export]
    speed: f32,
    point: i8,
    is_dead: bool,
    base: Base<PathFollow2D>,
}

#[godot_api]
impl IPathFollow2D for FollowPathEnemy {
    fn init(base: Base<PathFollow2D>) -> Self {
        FollowPathEnemy {
            speed: 0.,
            point: 2,
            is_dead: false,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        if !self.is_dead {
            self.follow_path(delta);
        }
    }
}

#[godot_api]
impl FollowPathEnemy {
    #[func]
    fn follow_path(&mut self, delta: f64) {
        let progress = self.base().get_progress_ratio();
        let mov = progress + self.speed * delta as f32;

        self.base_mut().set_progress_ratio(mov);
    }

    #[func]
    fn hit(&mut self, _area: Gd<Area2D>) {
        let mut anim = self.base().get_node_as::<AnimatedSprite2D>("Sprite2D");
        self.is_dead = true;
        anim.set_scale(Vector2 { x: 1., y: 1. });
        anim.set_animation("explo");
        let mut player = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let player_points = player.bind().get_points();

        let final_points = player_points + self.point as i16;

        player.bind_mut().set_points(final_points);
    }

    #[func]
    fn on_sprite_2d_animation_finished(&mut self) {
        self.base_mut().queue_free();
    }
}
