use bevy::{asset::Assets, color::Color, input::common_conditions::input_pressed, prelude::{self as p, KeyCode, Query, Res, ResMut, With}, render::mesh::Mesh, sprite::{ColorMaterial, MaterialMesh2dBundle}, transform::components::Transform, utils::default};
use bevy::prelude::IntoSystemConfigs;

use crate::*;


#[derive(p::Component)]
pub struct MoveWithPlayer;


#[derive(p::Component)]
pub struct Player;


pub struct LIPlayerPlugin;

impl p::Plugin for LIPlayerPlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .add_systems(p::Startup, spawn_player)
            .add_systems(p::Update, (
                move_left.run_if(input_pressed(KeyCode::KeyA)),
                move_right.run_if(input_pressed(KeyCode::KeyD)),
                move_up.run_if(input_pressed(KeyCode::KeyW)),
                move_down.run_if(input_pressed(KeyCode::KeyS)),
            ));
    }
}


fn spawn_player(
    mut commands: p::Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colour_materials: ResMut<Assets<ColorMaterial>>
) -> ()
{
    let shape = Mesh::from(p::Rectangle::new(0.875, 0.875));
    let colour = ColorMaterial::from(Color::srgb(1.0, 1.0, 1.0));

    let mesh_handle = meshes.add(shape);
    let colour_handle = colour_materials.add(colour);
    
    commands.spawn((
        Player,
        MoveWithPlayer,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: colour_handle,
            ..default()
        }
    ));
}

fn move_left(mut query_entities: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut entity in query_entities.iter_mut()
    {
        entity.translation.x -= config.player_speed as f32;
    }
}

fn move_right(mut query_entities: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut entity in query_entities.iter_mut()
    {
        entity.translation.x += config.player_speed as f32;
    }
}

fn move_up(mut query_entities: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut entity in query_entities.iter_mut()
    {
        entity.translation.y += config.player_speed as f32;
    }
}

fn move_down(mut query_entities: Query<&mut Transform, With<MoveWithPlayer>>, config: Res<LIConfig>)
{
    for mut entity in query_entities.iter_mut()
    {
        entity.translation.y -= config.player_speed as f32;
    }
}


