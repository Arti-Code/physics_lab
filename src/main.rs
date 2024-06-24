#![allow(unused)]

mod gui;
mod my_images;
mod particle;
mod wall;
mod signal;
mod bound;

use std::ops::RangeInclusive;

use bevy::core_pipeline::bloom::{BloomPrefilterSettings, BloomSettings};
use bevy::core_pipeline::tonemapping::Tonemapping;
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
use crate::signal::*;
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
        .add_plugins(SignalPlugin)
        .add_plugins(EguiPlugin)
        .add_plugins(GUIPlugin)    
        .add_plugins(MyImagesPlugin)
        .add_plugins(WallPlugin)    
        .add_plugins(ParticlePlugin)
        .add_systems(Startup, setup)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true,
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface,
            ..default()
        },
        BloomSettings {
            intensity: 0.5,
            low_frequency_boost: 0.1,
            high_pass_frequency: 0.6,
            prefilter_settings: BloomPrefilterSettings {
                threshold: 0.0,
                threshold_softness: 0.7,
            },
            composite_mode: bevy::core_pipeline::bloom::BloomCompositeMode::Additive,
            low_frequency_boost_curvature: 1.0,
            ..Default::default()
        },
    ));
    //commands.spawn(cam);
}

#[derive(Resource)]
struct Diameter {
    pub w: RangeInclusive<f32>,
    pub h: RangeInclusive<f32>,
}