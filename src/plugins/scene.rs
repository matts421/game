use bevy::prelude::*;

pub struct ScenePlugin; 



impl Plugin for ScenePlugin{
    fn build(&self, app: &mut App){
        app
            .insert_resource(ClearColor(Color::srgb(0.1, 0., 0.15)))
            .insert_resource(AmbientLight {
                color: Color::WHITE,
                brightness: 750.0,
                ..Default::default()
            });
    }
}
