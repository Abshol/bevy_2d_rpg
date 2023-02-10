use bevy::prelude::*;
use bevy_inspector_egui::{WorldInspectorPlugin, RegisterInspectable};

use crate::{player::{Player, EncounterTracker}, combat::CombatStats};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        if cfg!(debug_assertions) {
            app
            .add_plugin(WorldInspectorPlugin::new())
            .register_type::<EncounterTracker>()
            .register_inspectable::<CombatStats>()
            .register_inspectable::<Player>();
        }
    }
}