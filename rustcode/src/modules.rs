use godot::{
    classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D},
    prelude::*,
};
use rand::{seq::SliceRandom, Rng};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Area2D)]
struct Modules {
    points: i8,

    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Modules {
    fn init(base: Base<Area2D>) -> Self {
        Modules { points: 0, base }
    }

    fn ready(&mut self) {
        let mut rng = rand::thread_rng();

        //Aleatorizando a pontuação de 1 a 5
        self.points = rng.gen_range(1..5);

        //load nas texturas e aleatorizando-as
        let mut tex: Vec<GString> = vec!["triang".into(), "trape".into(), "quadr".into()];
        tex.shuffle(&mut rng);

        let texture = tex[0].clone();

        //Aplica textura
        let mut sprite = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        sprite.set_animation(texture.arg());
    }
}

#[godot_api]
impl Modules {
    #[func]
    fn hit(&mut self, _area: Gd<Area2D>) {
        let mut sprite = self
            .base_mut()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        self.base_mut().set_monitoring(false);
        self.base_mut().set_monitorable(false);
        sprite.set_animation("explo");
        sprite.play();
    }
    #[func]
    fn death(&mut self) {
        let mut player = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let player_points = player.bind().get_points();

        let final_points = player_points + self.points as i16;

        player.bind_mut().set_points(final_points);

        self.base_mut().queue_free();
    }
}
