/*use bevy::asset::LoadedFolder;
use p::{AssetServer, Res, ResMut, Resource};

use crate::*;

pub struct LITexturePlugin;

impl LITexturePlugin
{
    pub fn build(&self, app: &mut p::App) -> ()
    {
        app
            .insert_resource(LITextures::none())
            .add_systems(p::Startup, load_textures);
    }
}


fn load_textures(
    mut textures: ResMut<LITextures>,
    asset_server: Res<AssetServer>
) -> ()
{
    textures.directory = "res".to_string();
    textures.handle = LITextures::load_dir(asset_server, "res");
}


#[derive(Resource)]
pub struct LITextures
{
    pub directory: String,
    pub handle: Option<p::Handle<LoadedFolder>>,
}

impl LITextures
{
    pub fn none() -> Self
    {
        LITextures {
            directory: String::new(),
            handle: None
        }
    }

    pub fn load_dir<S: Into<String>>(
        asset_server: Res<AssetServer>,
        directory: S,
    ) -> Handle
    {
        let directory = directory.into();
        asset_server.load_folder(directory)
    }
}*/


