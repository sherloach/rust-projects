#![allow(unused)]

use std::f32::consts::E;

use bevy::{prelude::*, window::PrimaryWindow};

// -- Asset Constants
const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const ENEMY_SPRITE: &str = "enemy_a_01.png";

// -- Resources
#[derive(Resource)]
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
}

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
        .add_startup_system(setup)
        .run();
}

fn setup(
    mut commands: Commands,
    assest_server: Res<AssetServer>,
    query: Query<&Window, With<PrimaryWindow>>,
) {
    // camera
    commands.spawn(Camera2dBundle::default());

    // capture window size
    let Ok(primary) = query.get_single() else {
        return;
    };

    let (win_w, win_h) = (primary.width(), primary.height());

    // add WinSize resource
    // let win_size = WinSize { w: win_w, h: win_h };
    // commands.insert_resource(win_size);

    // add GameTextures resourse
    let game_textures = GameTextures {
        player: assest_server.load(PLAYER_SPRITE),
    };

    // commands.insert_resource(game_textures);
    let bottom = -win_h / 2.;
    commands.spawn(SpriteBundle {
        texture: game_textures.player,
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. + 5., 10.),
            ..Default::default()
        },
        ..Default::default()
    });
}
