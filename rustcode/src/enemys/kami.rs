use godot::{
    classes::{AnimatedSprite2D, Area2D, CharacterBody2D, IArea2D, Timer},
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct KamikazeEnemy {
    #[export]
    speed: f32,
    #[export]
    point: i16,
    #[export]
    followtimer: Gd<Timer>,
    #[export]
    explosion_sound: Gd<AudioStreamPlayer>,
    following: bool,
    is_dead: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for KamikazeEnemy {
    fn init(base: Base<Area2D>) -> Self {
        KamikazeEnemy {
            speed: 0.,
            point: 1,
            followtimer: Timer::new_alloc(),
            following: true,
            is_dead: false,
            explosion_sound: AudioStreamPlayer::new_alloc(),
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

    #[func]
    pub fn on_player_entered(&mut self, _body: Gd<Player>) {
        let mut player = self
            .base()
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
