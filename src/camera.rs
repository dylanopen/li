use bevy::prelude::{self as p, KeyCode, Query, With, Res};
use bevy::input::common_conditions;
use bevy::render::camera::OrthographicProjection;
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
                camera_zoom_in.run_if(common_conditions::input_pressed(KeyCode::Equal)),
                camera_zoom_out.run_if(common_conditions::input_pressed(KeyCode::Minus)),
            ));
    }
}


#[derive(p::Component)]
pub struct GameCamera;


fn spawn_world_camera(
    mut commands: p::Commands,
    config: Res<LIConfig>
) -> ()
{
    let mut world_camera = p::Camera2dBundle::default();
    world_camera.projection.scale = (config.initial_camera_scale) as f32;
    commands.spawn((GameCamera, MoveWithPlayer, world_camera));
}

pub fn zoom_cameras(
    projections: &mut Query<&mut OrthographicProjection, With<GameCamera>>,
    scale_factor: f64,
    config: Res<LIConfig>
) -> ()
{
    for mut projection in projections.iter_mut()
    {
        println!("sf = {}; scale = {}", scale_factor, projection.scale);
        projection.scale *= scale_factor as f32;
        if (projection.scale as f64) < config.camera_scale_min
        {
            projection.scale = config.camera_scale_min as f32;
        }
        if (projection.scale as f64) > config.camera_scale_max
        {
            projection.scale = config.camera_scale_max as f32;
        }
    }
}

pub fn camera_zoom_in(
    mut projections: Query<&mut OrthographicProjection, With<GameCamera>>,
    delta_time_res: Res<DeltaTime>,
    config: Res<LIConfig>
) -> ()
{
    // TODO: zooming in and zooming out will not be completely even due
    // to how percentage changes work (although this may now  be fixed).

    let scale_factor = 1.0 / (1.0 + delta_time_res.0 * config.camera_zoom_speed);
    zoom_cameras(&mut projections, scale_factor, config);
}

pub fn camera_zoom_out(
    mut projections: Query<&mut OrthographicProjection, With<GameCamera>>,
    delta_time_res: Res<DeltaTime>,
    config: Res<LIConfig>
) -> ()
{
    let scale_factor = 1.0 + delta_time_res.0 * config.camera_zoom_speed; // * config.zoom_speed
    zoom_cameras(&mut projections, scale_factor, config);
}

// NOTE: Zooming in requires you to REDUCE the scale. Think of it as
//       making the viewport smaller so what is still visible is larger.
//
//       Zooming out is the opposite - INCREASE the scale so that more
//       of the scene is visible.

