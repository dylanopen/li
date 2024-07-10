mod game;
mod debug;
mod res;
mod time;
mod camera;
mod conf;
mod player;

pub use game::*;
pub use debug::*;
pub use conf::*;
pub use time::*;
pub use camera::*;
pub use player::*;

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
            LIPlayerPlugin,
            LIDebugPlugin,
        ))
        .run();
}

