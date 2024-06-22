#![allow(unused)]

mod gui;
mod my_images;
mod particle;
mod wall;

use std::ops::RangeInclusive;

use bevy::{
    prelude::*, 
    render::render_resource::Texture, 
    window::PresentMode,
};
use bevy_egui::EguiPlugin;

use crate::gui::GUIPlugin;
use crate::my_images::MyImagesPlugin;
use crate::particle::ParticlePlugin;
use crate::wall::WallPlugin;
use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(Diameter { w: -400.0..=400.0, h: -300.0..=300.0 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PHYSICS LAB".into(),
                name: Some("physics lab".into()),
                resolution: (800., 600.).into(),
                present_mode: PresentMode::AutoVsync,
                enabled_buttons: bevy::window::EnabledButtons {
                    maximize: false,
                    ..Default::default()
                },
                visible: true,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin)
        .add_plugins(GUIPlugin)    
        .add_plugins(MyImagesPlugin)
        .add_plugins(WallPlugin)    
        .add_plugins(ParticlePlugin)
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(Resource)]
struct Diameter {
    pub w: RangeInclusive<f32>,
    pub h: RangeInclusive<f32>,
}