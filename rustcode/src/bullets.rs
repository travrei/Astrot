use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
struct PlayerBullet {
    #[export]
    speed: f32,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for PlayerBullet {
    fn init(base: Base<Area2D>) -> Self {
        PlayerBullet { speed: 0., base }
    }

    fn process(&mut self, delta: f64) {
        let mut dir = self.base().get_position();

        dir.y -= self.speed * delta as f32;

        self.base_mut().set_position(dir);
    }
}

#[godot_api]
impl PlayerBullet {
    #[func]
    fn on_area_entered(&mut self, _area: Gd<Area2D>) {
        self.base_mut().queue_free();
    }
}
