use p::{default, AssetServer, Bundle, Commands, Component, Res, Resource, SpriteBundle};

use crate::*;


pub struct LITilePlugin;


impl p::Plugin for LITilePlugin
{
    fn build(&self, app: &mut p::App)
    {
        
    }
}


#[derive(Clone)]
pub struct TileType
{
    name: String,
    path: String,
}

impl TileType
{
    pub fn new<S: Into<String>>(
        name: S, filename: S,
    ) -> Self
    {
        let name = name.into();
        let path = format!("res/tile/{}", filename.into());

        TileType {
            name, path,
        }
    }
}


#[derive(Clone, Component)]
pub struct Tile
{
    x: i32,
    y: i32,
    tiletype: TileType,
}

impl Tile
{
    pub fn new(x: i32, y: i32, tiletype: TileType)
        -> Self
    {
        Tile {
            x, y,
            tiletype,
        }
    }
}


#[derive(Bundle)]
struct TileSpriteBundle
{
    tile: Tile,
    sprite_bundle: SpriteBundle,
}


#[derive(Resource)]
pub struct Level
{
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl Level
{
    pub fn create_sprites(
        &self,
        mut commands: Commands,
        asset_server: Res<AssetServer>
    ) -> ()
    {
        for tile in &self.tiles
        {
            let texture = asset_server.load(tile.tiletype.path.clone());
            commands.spawn(TileSpriteBundle {
                tile: tile.clone(),
                sprite_bundle: SpriteBundle {
                    texture,
                    ..default()
                }
            });
        }
    }
}

