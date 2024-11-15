use godot::{
    classes::{AnimatedSprite2D, Area2D, CharacterBody2D, ICharacterBody2D, Marker2D},
    prelude::*,
};

use crate::assistent::Assistent;

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
pub struct Player {
    #[export]
    speed: f32,
    #[export]
    #[var]
    points: i16,
    #[export]
    level: Level,
    #[export]
    shoot_point: Gd<Marker2D>,
    #[export]
    bullet_scene: Gd<PackedScene>,
    #[var]
    num_assistent: i8,
    assistents_spawned: i8,

    base: Base<CharacterBody2D>,
}

#[derive(GodotConvert, Var, Export)]
#[godot(via = GString)]
pub enum Level {
    First,
    Second,
    Final,
}

#[godot_api]
impl ICharacterBody2D for Player {
    fn init(base: Base<CharacterBody2D>) -> Self {
        Player {
            speed: 0.,
            points: 0,
            level: Level::First,
            num_assistent: 0,
            assistents_spawned: 0,
            shoot_point: Marker2D::new_alloc(),
            bullet_scene: PackedScene::new_gd(),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let input = Input::singleton();

        self.moviment(delta);
        self.set_player_level();
        self.check_points();
        if input.is_action_just_pressed("Shoot") {
            self.shoot();
        }
        //godot_print!("{}", self.points)
    }
}

#[godot_api]
impl Player {
    #[func]
    fn spawn_assistent(&mut self) {
        let assistent_scene: Gd<PackedScene> = load("res://scenes/player/assistent.tscn");
        let mut assistent = assistent_scene.instantiate_as::<Assistent>();

        let distance = assistent.bind().get_dist() * self.num_assistent as f32;

        assistent.bind_mut().set_dist(distance);
        self.base_mut().get_parent().unwrap().add_child(assistent);
    }

    #[func]
    fn check_points(&mut self) {
        match self.points {
            _ => {}
        }
    }

    #[func]
    fn shoot(&mut self) {
        match self.level {
            Level::First => {
                let spawn_point = self.shoot_point.get_global_position();
                let mut bullet = self.bullet_scene.instantiate_as::<Area2D>();

                bullet.set_position(spawn_point);
                self.base_mut().get_parent().unwrap().add_child(bullet);
            }
            Level::Second => {
                let spawn_point = self.shoot_point.get_global_position();
                self.bullet_scene = load("res://scenes/player/player_bullet_lv_2.tscn");
                let mut bullet = self.bullet_scene.instantiate_as::<Area2D>();

                bullet.set_position(spawn_point);
                self.base_mut().get_parent().unwrap().add_child(bullet);
            }
            Level::Final => {
                let spawn_point = self.shoot_point.get_global_position();
                self.bullet_scene = load("res://scenes/player/player_bullet_lv_3.tscn");
                let mut bullet = self.bullet_scene.instantiate_as::<Area2D>();

                bullet.set_position(spawn_point);
                self.base_mut().get_parent().unwrap().add_child(bullet);
            }
        }
    }

    #[func]
    fn set_player_level(&mut self) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");
        match self.level {
            Level::First => sprite.set_animation("lv1"),
            Level::Second => sprite.set_animation("lv2"),
            Level::Final => sprite.set_animation("lv3"),
        }
    }

    #[func]
    fn moviment(&mut self, delta: f64) {
        let input = Input::singleton();

        let mut dir = Vector2::ZERO;

        dir.x = input.get_axis("Left", "Right");
        dir.y = input.get_axis("Up", "Down");

        if dir != Vector2::ZERO {
            dir.normalized();
        }

        let vel = dir * self.speed * delta as f32;

        self.base_mut().set_velocity(vel);
        self.base_mut().move_and_slide();
    }

    #[func]
    fn death(&mut self) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.set_animation("death");
        self.base_mut().queue_free();
    }
}
