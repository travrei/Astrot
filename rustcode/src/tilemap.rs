use godot::{
    classes::{ITileMapLayer, TileMapLayer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=TileMapLayer)]
pub struct SpaceStation {
    direction: Vector2,
    #[export]
    speed: f32,
    base: Base<TileMapLayer>,
}

#[godot_api]
impl ITileMapLayer for SpaceStation {
    fn init(base: Base<TileMapLayer>) -> Self {
        SpaceStation {
            direction: Vector2::DOWN,
            speed: 0.,
            base,
        }
    }

    fn process(&mut self, delta: f64) {
        let mut position = self.base().get_position();

        position += self.direction * self.speed * delta as f32;

        self.base_mut().set_position(position)
    }
}
