use godot::{
    classes::{ILabel, Label},
    obj::WithBaseField,
    prelude::*,
};

use crate::player::Player;

#[derive(GodotClass)]
#[class(base=Label)]
pub struct PlayerPoints {
    base: Base<Label>,
}

#[godot_api]
impl ILabel for PlayerPoints {
    fn init(base: Base<Label>) -> Self {
        PlayerPoints { base }
    }

    fn process(&mut self, _delta: f64) {
        let player = self
            .base()
            .get_parent()
            .unwrap()
            .get_parent()
            .unwrap()
            .get_node_as::<Player>("Player");

        let points = player.bind().get_points();
        let text = points.to_string();
        let gtext = text.to_godot();

        //self.base_mut().set_text(&gtext);
    }
}
