use bevy::prelude as p;


const LI_INFO: &str = "Little Island v0.0.2";


pub struct LIPlugin;

impl p::Plugin for LIPlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .add_systems(p::Startup, print_version);
    }
}

fn print_version()
{
    println!("Welcome to {}!", LI_INFO);
}

