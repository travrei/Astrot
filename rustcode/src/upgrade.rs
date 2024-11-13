use godot::{
    classes::{Area2D, IArea2D, Marker2D},
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Upgrade {
    #[export]
    speed: f32,
    #[export]
    spawn_point: Gd<Marker2D>,
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Upgrade {
    fn init(base: Base<Area2D>) -> Self {
        Upgrade {
            speed: 0.,
            spawn_point: Marker2D::new_alloc(),
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut position = self.spawn_point.get_global_position();

        position += Vector2::DOWN * self.speed * delta as f32;

        self.base_mut().set_position(position);
    }
}

#[godot_api]
impl Upgrade {
    #[func]
    fn on_area_entered(&mut self, _area: Gd<Area2D>) {
        let mut player = self
            .base()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let level = player.bind().get_level().to_string();
        let level_str = level.as_str();

        match level_str {
            "First" => player.bind_mut().set_level("Second".into()),
            "Second" => player.bind_mut().set_level("Final".into()),
            "Final" => {}
            _ => {}
        }
    }
}
