use godot::{
    classes::{Area2D, CharacterBody2D, IArea2D, IMarker2D, Marker2D},
    prelude::*,
};

use crate::player::Player;

//O SPAWNER
#[derive(GodotClass)]
#[class(base=Marker2D)]
pub struct SpawnerUpgrade {
    #[export]
    upgrade: Gd<PackedScene>,
    #[export]
    next_spawn_threshold: i16,
    base: Base<Marker2D>,
}

#[godot_api]
impl IMarker2D for SpawnerUpgrade {
    fn init(base: Base<Marker2D>) -> Self {
        SpawnerUpgrade {
            upgrade: PackedScene::new_gd(),
            next_spawn_threshold: 2,
            base,
        }
    }
    fn process(&mut self, _delta: f64) {
        self.spawn_upgrade()
    }
}

#[godot_api]
impl SpawnerUpgrade {
    #[func]
    fn instanciate_upgrade(&mut self) {
        let upgrade = self.upgrade.instantiate_as::<Upgrade>();
        self.base_mut().get_parent().unwrap().add_child(upgrade);
    }

    #[func]
    fn spawn_upgrade(&mut self) {
        let player_points = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player")
            .bind()
            .get_points();

        if player_points >= self.next_spawn_threshold {
            self.instanciate_upgrade();
            self.next_spawn_threshold += self.next_spawn_threshold;
        }
    }
}

//UPGRADE QUE VAI MUDAR O LEVEL DO PLAYER
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Upgrade {
    #[export]
    speed: f32,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Upgrade {
    fn init(base: Base<Area2D>) -> Self {
        Upgrade { speed: 0., base }
    }

    fn ready(&mut self) {
        let position = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<SpawnerUpgrade>("Spawn_Point")
            .get_global_position();

        self.base_mut().set_position(position);
    }

    fn process(&mut self, delta: f64) {
        let mut position = self.base().get_position();

        position += Vector2::DOWN * self.speed * delta as f32;

        self.base_mut().set_position(position);
    }
}

#[godot_api]
impl Upgrade {
    #[func]
    fn on_body_entered(&mut self, _body: Gd<CharacterBody2D>) {
        let mut player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let counter = player.bind_mut().get_upgrade_counter();

        player.bind_mut().set_upgrade_counter(counter + 1);

        if counter >= 1 {
            let level = player.bind().get_level().to_string();
            let level_str = level.as_str();
            match level_str {
                "First" => player.bind_mut().set_level("Second".into()),
                "Second" => player.bind_mut().set_level("Final".into()),
                "Final" => {
                    let num_assistent = player.bind_mut().get_num_assistent();
                    if num_assistent < 2 {
                        player.bind_mut().set_num_assistent(num_assistent + 1);
                        player.bind_mut().spawn_assistent()
                    } else {
                        let mut player_points = player.bind().get_points();

                        player_points += 2;

                        player.bind_mut().set_points(player_points);
                    }
                }
                _ => {}
            }
            player.bind_mut().set_upgrade_counter(0);
        }

        self.base_mut().queue_free();
    }
}
