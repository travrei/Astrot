use godot::prelude::*;

struct Astrot;

#[gdextension]
unsafe impl ExtensionLibrary for Astrot {}

//Mod
mod bullets;
mod buttons;
mod player;
