use bevy::prelude::*;
use wings_ui::plugin::WingsUiPlugin;

pub struct WingsPlugin;

impl Plugin for WingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WingsUiPlugin);
    }
}
