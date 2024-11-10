use godot::prelude::*;

struct Astrot;

#[gdextension]
unsafe impl ExtensionLibrary for Astrot {}

//Modules
mod bullets;
mod enemys;
mod modules;
mod player;
