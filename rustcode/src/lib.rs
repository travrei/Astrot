use godot::prelude::*;

struct Astrot;

#[gdextension]
unsafe impl ExtensionLibrary for Astrot {}

//Modules
mod assistent;
mod bullets;
mod degub;
mod enemys;
mod modules;
mod player;
mod upgrade;
