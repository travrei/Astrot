use godot::{
    classes::{Area2D, CharacterBody2D, ICharacterBody2D, Marker2D},
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Assistent {
    #[export]
    speed: f32,
    #[export]
    shoot_point: Gd<Marker2D>,
    #[var]
    dist: f32,
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl ICharacterBody2D for Assistent {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Assistent {
            speed: 0.,
            shoot_point: Marker2D::new_alloc(),
            dist: 24.,
            base,
        }
    }
    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        self.movimentation(delta);
        if input.is_action_just_pressed("Shoot") {
            self.shoot();
        }
    }
}

#[godot_api]
impl Assistent {
    #[func]
    fn movimentation(&mut self, delta: f64) {
        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let player_position = player.get_global_position();
        let positon = self.base().get_global_position();

        if positon.distance_to(player_position) > self.dist {
            let direction = (player_position - positon).normalized();

            let velocity = direction * self.speed * delta as f32;

            self.base_mut().set_velocity(velocity);
            self.base_mut().move_and_slide();
        }
    }
    #[func]
    fn shoot(&mut self) {
        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let spawn_point = self.shoot_point.get_global_position();
        let level = player.bind().get_level().to_string();

        let bullet_path = match level.as_str() {
            "First" => "res://scenes/player/player_bullet.tscn",
            "Second" => "res://scenes/player/player_bullet_lv_2.tscn",
            "Final" => "res://scenes/player/player_bullet_lv_3.tscn",
            _ => return,
        };

        let bullet_scene: Gd<PackedScene> = load(bullet_path);
        let mut bullet = bullet_scene.instantiate_as::<Area2D>();
        bullet.set_position(spawn_point);

        if let Some(mut parent) = self.base_mut().get_parent() {
            parent.add_child(bullet);
        }
    }
}
