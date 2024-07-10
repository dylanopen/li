use bevy::prelude::{self as p, KeyCode, Query, With};
use bevy::input::common_conditions;
use bevy::transform::components::Transform;
use bevy::prelude::IntoSystemConfigs;


pub struct LICameraPlugin;


impl p::Plugin for LICameraPlugin
{
    fn build(&self, app: &mut p::App)
    {
        app
            .add_systems(p::Startup, spawn_world_camera)
            .add_systems(p::Update,
                move_camera_left.run_if(common_conditions::input_pressed(KeyCode::KeyA)),
            );
    }
}


#[derive(p::Component)]
struct WorldCameraMarker;


fn spawn_world_camera(mut commands: p::Commands)
{
    let world_camera = p::Camera2dBundle::default();
    commands.spawn((world_camera, WorldCameraMarker));
}

fn move_camera_left(mut query_camera: Query<&mut Transform, With<WorldCameraMarker>>)
{
    let mut camera = query_camera.single_mut();
    camera.translation.x -= 1.0;
    println!("{}", camera.translation.x);
}

