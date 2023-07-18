#![allow(unused)]

use bevy::prelude::*;

const PLAYER_SPRITE: &str = "player_a_01.png";

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Game".into(),
                resolution: (598., 676.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .run();
}
