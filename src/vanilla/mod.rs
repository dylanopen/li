use crate::*;

mod level;
mod tiles;

pub use level::*;
pub use tiles::*;


pub struct VanillaPlugins;

impl p::Plugin for VanillaPlugins
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .add_plugins(VanillaTilesPlugin)
            .add_plugins(VanillaLevelTownPlugin);
    }
}

