use p::{Res, ResMut};

use crate::*;


pub struct VanillaLevelTownPlugin;

impl p::Plugin for VanillaLevelTownPlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .add_systems(p::Startup, add_town_level);
    }
}


pub fn add_town_level(
    tile_types_res: Res<TileTypes>, // TODO: WHY DOES THIS NOT EXIST??!!
    mut levels: ResMut<Levels>,
) -> ()
{
    let tile_list: Vec<&str> = vec![
        "vanilla/grass", "vanilla/grass", "vanilla/grass",
        "vanilla/grass", "vanilla/grass", "vanilla/grass",
    ];
    let width = 3;
    let tileworld = TileWorld::from_vec(tile_types_res, width, tile_list);
    levels.0.push(Level {
        tileworld,
    }); // TODO: make TownLevel resource
}

