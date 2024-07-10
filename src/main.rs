mod game;
mod debug;
mod res;
mod time;
mod camera;

use game::*;
use debug::*;
use res::*;
use time::*;
use camera::*;

use bevy::prelude as p;


fn main()
{
    p::App::new()
        .add_plugins((
            p::DefaultPlugins,
            LIPlugin,
            LITimePlugin,
            LICameraPlugin,
            LIDebugPlugin,
        ))
        .run();
}
