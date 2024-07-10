use bevy::prelude as p;

use crate::*;


pub struct LIDebugPlugin;

impl p::Plugin for LIDebugPlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .add_systems(p::Update, print_fps);
    }
}

fn print_fps(delta_time_res: p::Res<DeltaTime>)
{
    let delta = delta_time_res.0;
    let fps;
    if delta == 0.0
    {
        fps = 1000000.0
    }
    else
    {
        fps = 1.0_f64 / delta;
    }
    println!("delta time = {}; fps = {}", delta, fps);
}

