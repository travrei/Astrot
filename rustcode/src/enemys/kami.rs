use godot::{
    classes::{Area2D, CharacterBody2D, IArea2D, Timer},
    prelude::*,
};

use crate::player::Player;

use super::follow;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct KamikazeEnemy {
    #[export]
    speed: f32,
    #[export]
    points: i8,
    #[export]
    followtimer: Gd<Timer>,
    following: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for KamikazeEnemy {
    fn init(base: Base<Area2D>) -> Self {
        KamikazeEnemy {
            speed: 0.,
            points: 1,
            followtimer: Timer::new_alloc(),
            following: true,
            base,
        }
    }
    fn process(&mut self, delta: f64) {
        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        if self.following {
            self.follow_player(player);
        } else if !self.following {
            self.downwego(delta);
        }
    }
}

#[godot_api]
impl KamikazeEnemy {
    #[func]
    fn follow_player(&mut self, player: Gd<Player>) {
        let player_pos = player.get_position();

        let mut pos = self.base().get_position();

        pos.x = player_pos.x;

        self.base_mut().set_position(pos);
    }

    #[func]
    fn downwego(&mut self, delta: f64) {
        let mut pos = self.base().get_position();

        pos.y += self.speed * delta as f32;

        self.base_mut().set_position(pos);
    }

    #[func]
    fn timer_out(&mut self) {
        self.following = false
    }
}

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct SpawnerKami {
    #[export]
    amount: i32,
    #[export]
    enemy: Gd<PackedScene>,
    #[export]
    timer: Gd<Timer>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for SpawnerKami {
    fn init(base: Base<Area2D>) -> Self {
        SpawnerKami {
            amount: 0,
            enemy: PackedScene::new_gd(),
            timer: Timer::new_alloc(),
            base,
        }
    }
}

#[godot_api]
impl SpawnerKami {
    #[func]
    fn spawn(&mut self) {
        let enemys = self.enemy.instantiate_as::<KamikazeEnemy>();
        self.base_mut()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .add_child(enemys);
    }
    #[func]
    fn timer_out(&mut self) {
        if self.amount > 0 {
            self.spawn();
            self.timer.start();
            self.amount -= 1;
        }
    }
    #[func]
    fn on_player_entered(&mut self, _player: Gd<CharacterBody2D>) {
        self.timer.start();
    }
}
