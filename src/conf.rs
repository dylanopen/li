use std::fs;

use bevy::{prelude::{self as p, Query, Res, With}, window::{PrimaryWindow, Window}};



#[derive(p::Resource, serde::Deserialize)]
pub struct LIConfig
{
    pub player_speed: f64,
    pub camera_zoom_speed: f64,
    pub camera_scale_min: f64,
    pub camera_scale_max: f64,
    pub initial_camera_scale: f64,
    pub window_title: String,
}



pub struct LIConfigPlugin;

impl p::Plugin for LIConfigPlugin
{
    fn build(&self, app: &mut p::App)
    {
        app
            .insert_resource(load_li_config("res/settings.json"))
            .add_systems(p::Startup, set_window_title);
    }
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

fn set_window_title(
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
    config: Res<LIConfig>
) -> ()
{
    if let Ok(mut window) = window_query.get_single_mut()
    {
        window.title = config.window_title.clone();
    } 
}

