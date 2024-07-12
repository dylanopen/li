use bevy::{asset::Assets, color::Color, prelude::{self as p, Commands, ResMut}, render::mesh::Mesh, sprite::{ColorMaterial, MaterialMesh2dBundle}, transform::components::Transform, utils::default};

use crate::*;


pub struct LIDebugPlugin;

impl p::Plugin for LIDebugPlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .add_systems(p::Startup, spawn_test_square)
            .add_systems(p::Update, print_fps);
    }
}

fn print_fps(delta_time_res: p::Res<DeltaTime>)
{
    let delta = delta_time_res.0;
    let fps;
    if delta == 0.0
    {
        fps = 1000000.0;
    }
    else
    {
        fps = 1.0_f64 / delta;
    }
    println!("delta time = {}; fps = {}", delta, fps);
}

fn spawn_test_square(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut colour_materials: ResMut<Assets<ColorMaterial>>,
) -> ()
{
    let shape = Mesh::from(p::Rectangle::new(1.0, 1.0));
    let colour = ColorMaterial::from(Color::srgb(0.9, 0.6, 0.25));

    let mesh_handle = meshes.add(shape);
    let colour_handle = colour_materials.add(colour);
    
    commands.spawn(
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: colour_handle,
            transform: Transform::from_xyz(1.5, 1.5, 1.5),
            ..default()
        }
    );

}

