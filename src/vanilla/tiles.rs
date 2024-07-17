use p::ResMut;
use vanilla::add_town_level;
use p::IntoSystemConfigs;

use crate::*;


pub struct VanillaTilesPlugin;

impl p::Plugin for VanillaTilesPlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .add_systems(p::Startup, add_tiles.before(add_town_level));
    }
}


fn add_tiles(
    mut tile_types_res: ResMut<TileTypes>,
) -> ()
{
    let vanilla_tile_types = vec![
        TileType::new("vanilla/grass", "Grass", "vanilla/grass.png")
    ];
    tile_types_res.add_vec(vanilla_tile_types);
}

