use bevy::prelude::*;


pub struct SignalPlugin;

impl Plugin for SignalPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AddParticle>();
    }
}


pub trait SignalTrait {
    fn new(state: bool) -> Self;
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn is_high(&self) -> bool;
}

#[derive(Resource)]
pub struct AddParticle(bool);

impl SignalTrait for AddParticle {
    
    fn new(state: bool) -> Self {
        AddParticle(state)
    }

    fn set_high(&mut self) {
        self.0 = true;
    }

    fn set_low(&mut self) {
        self.0 = false;    
    }

    fn is_high(&self) -> bool {
        self.0
    }

}

impl Default for AddParticle {
    
    fn default() -> Self {
        AddParticle(false)
    }

}