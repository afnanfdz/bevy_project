use super::components::{Name, Person};
use bevy::prelude::*;

pub fn hello_world() {
    println!("Hello!");
}

pub fn add_vuc(mut commands: Commands) {
    commands.spawn((Person, Name("Vuc Skye".to_string())));
}

pub fn greet_people(query: Query<&Name, With<Person>>) {
    for name in query.iter() {
        println!("Hello {}!", name.0);
    }
}
