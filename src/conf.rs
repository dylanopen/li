use std::fs;

use bevy::prelude as p;


pub struct LIConfigPlugin;

impl p::Plugin for LIConfigPlugin
{
    fn build(&self, app: &mut p::App)
    {
        app
            .insert_resource(load_li_config("res/settings.json"));
    }
}


#[derive(p::Resource, serde::Deserialize)]
pub struct LIConfig
{
    pub player_speed: f32,
}


fn load_li_config(filepath: &'static str) -> LIConfig
{
    // Temporary test value

    let json_text = fs::read_to_string(filepath)
        .expect(&format!("error - failed to read config file {} - make sure the json file exists.", filepath));
    let config: LIConfig = serde_json::from_str(&json_text)
        .expect("error - failed to parse config file - make sure it is formatted correctly and contains all fields.");

    config
}

