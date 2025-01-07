use godot::{
    classes::{Area2D, IArea2D, Timer},
    prelude::*,
};
use rand::{thread_rng, Rng};

use crate::{bullets::EnemyBullet, player::Player};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct FirstBoss {
    #[export]
    speed: f32,
    #[var]
    num_torrets: i32,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for FirstBoss {
    fn init(base: Base<Area2D>) -> Self {
        FirstBoss {
            speed: 0.,
            num_torrets: 4,

            base,
        }
    }

    fn process(&mut self, delta: f64) {
        self.movimentation(delta);
        if self.num_torrets <= 0 {
            self.activate_central();
        }
    }
}

#[godot_api]
impl FirstBoss {
    #[func]
    fn get_player_position(&mut self) -> Vector2 {
        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        return player.get_global_position();
    }

    #[func]
    fn movimentation(&mut self, delta: f64) {
        let player_pos = self.get_player_position();
        let mut pos = self.base().get_global_position();
        let mut dir = Vector2::ZERO;

        if player_pos.x > pos.x + 32. {
            dir.x = 1.;
        } else if player_pos.x < pos.x - 32. {
            dir.x = -1.;
        } else {
            dir.x = 0.;
        }
        pos += dir * self.speed * delta as f32;

        self.base_mut().set_position(pos);
    }

    #[func]
    fn activate_central(&mut self) {}
}

// --- TORRETS ---
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct FBossTorret {
    #[export]
    firerate: Gd<Timer>,
    #[export]
    bullet_scene: Gd<PackedScene>,
    #[export]
    health: f32,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for FBossTorret {
    fn init(base: Base<Area2D>) -> Self {
        FBossTorret {
            bullet_scene: PackedScene::new_gd(),
            firerate: Timer::new_alloc(),
            health: 0.,
            base,
        }
    }
}

#[godot_api]
impl FBossTorret {
    fn get_player_position(&mut self) -> Vector2 {
        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        return player.get_global_position();
    }

    #[func]
    fn shoot(&mut self) {
        let torrent_pos = self.base().get_global_position();

        let player_pos = self.get_player_position();

        let direction = (player_pos - torrent_pos).normalized_or_zero();

        let mut bullet = self.bullet_scene.instantiate_as::<EnemyBullet>();
        self.base_mut()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .add_child(bullet.clone());
        bullet.clone().set_global_position(torrent_pos);
        bullet.bind_mut().set_dir(direction);
    }

    #[func]
    fn hit(&mut self, _area: Gd<Area2D>) {
        self.health -= 1.;

        if self.health <= 0. {
            self.death()
        }
    }

    #[func]
    fn death(&mut self) {
        let mut boss = self
            .base_mut()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<FirstBoss>("FirstBoss");

        let mut torrets = boss.bind().get_num_torrets();
        torrets -= 1;
        boss.bind_mut().set_num_torrets(torrets);

        self.base_mut().queue_free();
    }

    #[func]
    fn time_out(&mut self) {
        self.shoot();
        self.firerate.start();
    }
}

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct CentralTorret {
    #[export]
    firerate: Gd<Timer>,
    #[export]
    bullet_scene: Gd<PackedScene>,
    #[export]
    health: f32,
    is_active: bool,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for CentralTorret {
    fn init(base: Base<Area2D>) -> Self {
        CentralTorret {
            firerate: Timer::new_alloc(),
            bullet_scene: PackedScene::new_gd(),
            health: 0.,
            is_active: false,
            base,
        }
    }
    fn process(&mut self, _delta: f64) {
        if self.get_boss().bind().get_num_torrets() <= 0 {
            self.is_active = true;
        }
        if self.is_active && self.firerate.is_stopped() {
            self.firerate.start();
        }
    }
}

#[godot_api]
impl CentralTorret {
    fn get_boss(&mut self) -> Gd<FirstBoss> {
        self.base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<FirstBoss>("FirstBoss")
    }

    #[func]
    fn shoot(&mut self) {
        let torrent_pos = self.base().get_global_position();

        for i in 0..20 {
            let random_offset = thread_rng().gen_range(-5.0..5.0);
            let angle = (i as f32 / 20.) * 190. + random_offset;
            let radian = angle.to_radians();
            let direction = Vector2::new(radian.cos(), radian.sin());

            let mut bullet = self.bullet_scene.instantiate_as::<EnemyBullet>();
            self.base_mut()
                .get_parent()
                .unwrap()
                .get_parent()
                .unwrap()
                .add_child(bullet.clone());
            bullet.clone().set_global_position(torrent_pos);
            bullet.bind_mut().set_dir(direction);
        }
    }

    #[func]
    fn time_out(&mut self) {
        self.shoot();
        self.firerate.start();
    }
}
