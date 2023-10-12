use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use crate::events::*;
use crate::systems::*;

/// A [`Plugin`](https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html)
/// that defines an interface for Wings User Interface.
pub struct WingsUiPlugin;

impl Plugin for WingsUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EventListenerPlugin::<PointerClick>::default(),
                EventListenerPlugin::<PointerDoubleClick>::default(),
                EventListenerPlugin::<PointerRelease>::default(),
                EventListenerPlugin::<PointerPress>::default(),
                EventListenerPlugin::<PointerMove>::default(),
                EventListenerPlugin::<PointerOver>::default(),
                EventListenerPlugin::<PointerEnter>::default(),
                EventListenerPlugin::<PointerExit>::default(),
            ))
            .add_event::<ApplyConstraintHeight>()
            .add_event::<ApplyConstraintWidth>()
            .add_systems(
                Update,
                (
                    compute_constraint_heights,
                    compute_constraint_widths,
                    buttons_interactions,
                    apply_constraint_height,
                    apply_constraint_width,
                ),
            );
    }
}
