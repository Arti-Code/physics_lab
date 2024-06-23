use bevy::prelude::*;
use bevy_egui::{
    egui::{
        self, Color32, ImageButton, RichText, Stroke, TopBottomPanel, Vec2 as UIVec2
    }, 
    EguiContexts, 
    EguiPlugin,
};
use crate::signal::*;

pub struct GUIPlugin;

impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, update);
    }
}


fn update(mut contexts: EguiContexts, mut add_particle: ResMut<AddParticle>) {
    egui::TopBottomPanel::top("top_panel").show(contexts.ctx_mut(), |ui| {
        ui.horizontal(|row| {
            if row.add(egui::Button::new(RichText::new("ADD").strong())
                .stroke(Stroke::new(2.0, Color32::GREEN))
                .min_size(UIVec2::new(32., 32.)))
                .clicked() {
                    add_particle.set_high();
                    //println!("Button0 clicked!");
            }
            if row.add(egui::Button::new(RichText::new("DEL").strong())
                .stroke(Stroke::new(2.0, Color32::RED))
                .min_size(UIVec2::new(32., 32.)))
                .clicked() {
                    println!("Button1 clicked!");
            }
        });
    });
}