#![allow(unused)]

//mod assets;

use bevy::{
    prelude::*, render::render_resource::Texture, window::PresentMode
};

use bevy_egui::{
    egui::{self, Color32, ImageButton, Stroke}, 
    EguiContexts, 
    EguiPlugin
};

use bevy_rapier2d::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
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
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EguiPlugin)
        .add_systems(Startup, (setup, load_sprites))
        .add_systems(Update, ui_system)
        .run();
}


fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn ui_system(mut contexts: EguiContexts, images: Res<ImagesAssets>) {
    egui::Window::new("Hello World").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|row| {
            if row.add(egui::Button::new("ADD").stroke(Stroke::new(2.0, Color32::GREEN))).clicked() {
                println!("Button0 clicked!");
            }
            if row.add(egui::Button::new("DEL").stroke(Stroke::new(2.0, Color32::YELLOW))).clicked() {
                println!("Button1 clicked!");
            }
        });
    });
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