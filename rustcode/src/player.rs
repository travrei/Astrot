use godot::{
    classes::{CharacterBody2D, ICharacterBody2D},
    obj::WithBaseField,
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CharacterBody2D)]
struct PlayerShip {
    speed: f32,
    direction: Vector2,
    #[base]
    base: Base<CharacterBody2D>,
}

#[godot_api]
impl PlayerShip {
    #[func]
    fn moviment(&mut self, delta: f64) {
        let mut velocity = self.base().get_velocity();

        let input = Input::singleton();

        self.direction.x = input.get_axis("ui_left".into(), "ui_right".into());
        self.direction.y = input.get_axis("ui_up".into(), "ui_down".into());

        if self.direction != Vector2::ZERO {
            self.direction = self.direction.normalized();
        }

        velocity = self.direction * self.speed * delta as f32;

        self.base_mut().set_velocity(velocity);
        self.base_mut().move_and_slide();
    }
}

#[godot_api]
impl ICharacterBody2D for PlayerShip {
    fn init(base: Base<CharacterBody2D>) -> Self {
        PlayerShip {
            speed: 300.,
            direction: Vector2::ZERO,
            base,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.moviment(delta)
    }
}
