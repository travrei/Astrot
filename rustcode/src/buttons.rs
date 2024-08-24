use godot::{
    classes::{Area2D, IArea2D},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Buttons {
    #[base]
    base: Base<Area2D>,
}

#[godot_api]
impl IArea2D for Buttons {
    fn init(base: Base<Area2D>) -> Self {
        Buttons { base }
    }
}
