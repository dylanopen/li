use bevy::prelude as p;


#[derive(p::Resource)]
pub struct DeltaTime(pub f64);


pub struct LITimePlugin;

impl p::Plugin for LITimePlugin
{
    fn build(&self, app: &mut p::App) -> ()
    {
        app
            .insert_resource(DeltaTime(0.0))
            .add_systems(p::Update, update_delta);
    }
}

fn update_delta(query_time: p::Res<p::Time>, mut res_delta_time: p::ResMut<DeltaTime>)
{
    res_delta_time.0 = query_time.delta_seconds_f64();
}
 
