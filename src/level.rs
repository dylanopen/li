use crate::*;

use p::Resource;


#[derive(Debug)]
pub struct Level
{
    pub tileworld: TileWorld,
}


#[derive(Debug, Resource)]
pub struct Levels(pub Vec<Level>);

impl Levels
{
    pub fn new() -> Self
    {
        Levels(Vec::new())
    }
}


pub struct LILevelPlugin;

impl p::Plugin for LILevelPlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .insert_resource(Levels::new());
    }
}



