use godot::{
    classes::{Area2D, IArea2D, Timer},
    prelude::*,
};

use crate::{bullets::EnemyBullet, player::Player};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct FirstBoss {
    #[export]
    speed: f32,
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
}

// --- TORRETS ---
#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct FBossTorret {
    #[export]
    firerate: Gd<Timer>,
    #[export]
    bullet_scene: Gd<PackedScene>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for FBossTorret {
    fn init(base: Base<Area2D>) -> Self {
        FBossTorret {
            bullet_scene: PackedScene::new_gd(),
            firerate: Timer::new_alloc(),
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
            .add_child(bullet.clone());
        bullet.clone().set_global_position(torrent_pos);
        bullet.bind_mut().set_dir(direction);
    }

    #[func]
    fn time_out(&mut self) {
        self.shoot();
        self.firerate.start();
    }
}
