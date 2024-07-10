mod game;
mod debug;
mod res;
mod time;
mod camera;
mod conf;

pub use game::*;
pub use debug::*;
pub use conf::*;
pub use time::*;
pub use camera::*;

use bevy::prelude as p;


fn main()
{
    p::App::new()
        .add_plugins((
            p::DefaultPlugins,
            LIPlugin,
            LIConfigPlugin,
            LITimePlugin,
            LICameraPlugin,
            LIDebugPlugin,
        ))
        .run();
}

