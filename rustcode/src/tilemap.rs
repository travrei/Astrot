use godot::{
    classes::{Area2D, ITileMapLayer, TileMapLayer},
    prelude::*,
};

use crate::enemys::firstboss::FirstBoss;

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct SpaceStation {
    direction: Vector2,
    #[export]
    speed: f32,
    is_moving: bool,
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for SpaceStation {
    fn init(base: Base<TileMapLayer>) -> Self {
        SpaceStation {
            direction: Vector2::DOWN,
            speed: 0.,
            is_moving: true,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        if self.is_moving {
            let mut position = self.base().get_position();

            position += self.direction * self.speed * delta as f32;

            self.base_mut().set_position(position)
        }
    }
}

#[godot_api]
impl SpaceStation {
    #[func]
    fn spawn_bos(&mut self) {
        let boss_scene = load::<PackedScene>("res://scenes/enemys/Boss/first_boss.tscn");

        let mut boss_node = boss_scene.instantiate_as::<FirstBoss>();

        boss_node.set_position(Vector2::new(0., 25.));
        boss_node.set_scale(Vector2::new(1.5, 1.5));
        self.base().get_parent().unwrap().add_child(boss_node);
    }
    #[func]
    fn stop(&mut self, _body: Gd<Node2D>) {
        self.is_moving = false;
        let mut boss_area = self.base().get_node_as::<Area2D>("Boss_area");

        boss_area.set_deferred("monitoring", &false.to_variant());
        boss_area.set_deferred("monitorable", &false.to_variant());

        self.spawn_bos()
    }
}
