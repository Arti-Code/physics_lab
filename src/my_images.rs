use bevy::prelude::*;

pub struct MyImagesPlugin;

impl Plugin for MyImagesPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Startup, load_sprites);
    }
}


#[derive(Resource)]
pub struct ImagesAssets {
    pub particle32: Handle<Image>, 
    pub particle: Handle<Image>,
}

fn load_sprites(
    mut commands: Commands,
    server: Res<AssetServer>
) {
    let handle: Handle<Image> = server.load("particle32.png");
    let texture: Handle<Image> = server.load("particle32.png");
    commands.insert_resource(ImagesAssets {
        particle32: handle,
        particle: texture,
    });
}