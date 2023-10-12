use bevy::prelude::*;
use wings_ui::plugin::WingsUiPlugin;

/// A [`Plugin`](https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html)
/// that defines an interface for all Wings plugins such as:
/// [`WingsUiPlugin`](WingsUiPlugin)
pub struct WingsPlugin;

impl Plugin for WingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(WingsUiPlugin);
    }
}
