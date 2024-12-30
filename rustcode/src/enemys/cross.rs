use std::f32::consts::PI;

use godot::{
    classes::{AnimatedSprite2D, Area2D, IArea2D},
    prelude::*,
};

use crate::player::Player;

//Cross-Enemy (SinWave)
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct CrossEnemy {
    #[export]
    amplitude: f32,
    #[export]
    frequencia: f32,
    #[export]
    point: i16,
    time: f32,
    is_dead: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for CrossEnemy {
    fn init(base: Base<Area2D>) -> Self {
        CrossEnemy {
            amplitude: 2.,
            frequencia: 2.,
            point: 0,
            time: 0.,
            is_dead: false,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        if !self.is_dead {
            let mut pos = self.base().get_position();

            self.time += delta as f32;

            pos.x += self.amplitude * (PI * self.frequencia * self.time).sin();

            self.base_mut().set_position(pos);
        }
    }
}

#[godot_api]
impl CrossEnemy {
    #[func]
    fn on_player_entered(&mut self, _body: Gd<Player>) {
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
        anim.set_scale(Vector2 { x: 1., y: 1. });
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

        let final_points = player_points + self.point as i16;

        player.bind_mut().set_points(final_points);
    }

    #[func]
    fn on_sprite_2d_animation_finished(&mut self) {
        self.base_mut().queue_free();
    }
}
