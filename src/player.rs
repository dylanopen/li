use bevy::{asset::Assets, color::Color, prelude::{self as p, ResMut}, render::mesh::Mesh, sprite::{ColorMaterial, MaterialMesh2dBundle}, utils::default};

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
            .add_systems(p::Startup, spawn_player);
    }
}


fn spawn_player(
    mut commands: p::Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colour_materials: ResMut<Assets<ColorMaterial>>
) -> ()
{
    let shape = Mesh::from(p::Rectangle::new(0.875, 0.875));
    let colour = ColorMaterial::from(Color::srgb(0.9, 0.6, 0.25));

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


