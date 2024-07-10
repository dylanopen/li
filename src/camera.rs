use bevy::prelude::{self as p, KeyCode, Query, With, Res};
use bevy::input::common_conditions;
use bevy::transform::components::Transform;
use bevy::prelude::IntoSystemConfigs;

use crate::*;


pub struct LICameraPlugin;


impl p::Plugin for LICameraPlugin
{
    fn build(&self, app: &mut p::App)
    {
        app
            .add_systems(p::Startup, spawn_world_camera)
            .add_systems(p::Update, (
                move_camera_left.run_if(common_conditions::input_pressed(KeyCode::KeyA)),
                move_camera_right.run_if(common_conditions::input_pressed(KeyCode::KeyD)),
                move_camera_up.run_if(common_conditions::input_pressed(KeyCode::KeyW)),
                move_camera_down.run_if(common_conditions::input_pressed(KeyCode::KeyS)),
            ));
    }
}


fn spawn_world_camera(mut commands: p::Commands)
{
    let mut world_camera = p::Camera2dBundle::default();
    world_camera.projection.scale = 1.0;
    commands.spawn((world_camera, MoveWithPlayer));
}

fn move_camera_left(mut query_camera: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut camera in query_camera.iter_mut()
    {
        camera.translation.x -= config.player_speed;
    }
}

fn move_camera_right(mut query_camera: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut camera in query_camera.iter_mut()
    {
        camera.translation.x += config.player_speed;
    }
}

fn move_camera_up(mut query_camera: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut camera in query_camera.iter_mut()
    {
        camera.translation.y += config.player_speed;
    }
}

fn move_camera_down(mut query_camera: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut camera in query_camera.iter_mut()
    {
        camera.translation.y -= config.player_speed;
    }
}

