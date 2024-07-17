use bevy::utils::HashMap;
use p::{default, AssetServer, Bundle, Commands, Component, Res, Resource, SpriteBundle};

use crate::*;


pub struct LITilePlugin;


impl p::Plugin for LITilePlugin
{
    fn build(&self, app: &mut p::App)
    {
        app
            .insert_resource(TileTypes(HashMap::new()));
    }
}


#[derive(Debug, Resource)]
pub struct TileTypes(pub HashMap<String, TileType>);

impl TileTypes
{
    pub fn add_vec(&mut self, tile_types: Vec<TileType>)
    {
        for tile_type in tile_types
        {
            self.0.insert(tile_type.id.clone(), tile_type);
        }
    }
}


#[derive(Debug, Clone)]
pub struct TileType
{
    id: String,
    name: String,
    path: String,
}

impl TileType
{
    pub fn new<S: Into<String>>(
        id: S, name: S, filename: S,
    ) -> Self
    {
        let id = id.into();
        let name = name.into();
        let path = format!("res/tile/{}", filename.into());

        TileType {
            id, name, path,
        }
    }

    pub fn from_id(
        tile_types: &Res<TileTypes>,
        id: String,
    ) -> Self
    {
        match tile_types.0.get(&id)
        {
            Some(tile_type) => tile_type.clone(),
            None => panic!("Error: The tile type '{}' is not loaded.", id)
        }
    }
}


#[derive(Debug, Clone, Component)]
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


#[derive(Debug, Bundle)]
struct TileSpriteBundle
{
    tile: Tile,
    sprite_bundle: SpriteBundle,
}


#[derive(Debug)]
pub struct TileWorld
{
    width: u32,
    height: u32,
    tiles: Vec<Tile>,
}

impl TileWorld
{
    pub fn new(width: u32, height: u32) -> Self
    {
        TileWorld {
            width, height,
            tiles: Vec::new()
        }
    }

    pub fn spawn_tile_sprites(
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
                    transform: p::Transform::from_xyz(tile.x as f32, tile.y as f32, 0.0),
                    ..default()
                }
            });
        }
    }

    pub fn from_vec<S: Into<String>>(
        tile_types_res: Res<TileTypes>,
        width: u32, tiles: Vec<S>
    ) -> Self
    {
        let height = tiles.len() as u32 / width;
        let mut level_tiles: Vec<Tile> = Vec::new();

        for (i, type_id) in tiles.into_iter().enumerate()
        {
            let x = i as i32 % width as i32;
            let y = i as i32 / width as i32;
            level_tiles.push(Tile::new(x, y, TileType::from_id(
                &tile_types_res, type_id.into()
            )));
        }

        TileWorld {
            width, height,
            tiles: level_tiles,
        }
    }
}

