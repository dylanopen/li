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


pub struct LIConfig
{
    pub player_speed: f32,

}


fn load_li_config(filepath: &'static str) -> LIConfig
{
    // Temporary test value

}

