use bevy::input::InputPlugin;
use bevy::prelude::*;
pub mod components;
pub mod systems;

use systems::*;

fn main() {
    App::new()
        .add_plugin(CorePlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_startup_system(add_vuc)
        .add_startup_system(add_vuc)
        .add_system(greet_people)
        .add_system(hello_world)
        .run();
}
