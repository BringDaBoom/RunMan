#![allow(unused)]

use bevy::prelude::*;

pub const CLEAR: Color = Color::rgb(0.1,0.1,0.1);

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.4, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Run, Man!".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .run();
}

fn setup_system(mut commands: Commands) {
    //camera
    commands.spawn_bundle(Camera2dBundle::default());

    //add rectangle
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite { color: Color::rgb(0.25,0.25,0.75), 
            custom_size: Some(Vec2::new(150.,150.)), 
            ..Default::default()
         }, 
        ..Default::default()
    });

}