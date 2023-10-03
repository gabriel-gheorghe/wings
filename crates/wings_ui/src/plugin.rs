use bevy::prelude::*;
use bevy_eventlistener::prelude::*;
use crate::events::*;
use crate::systems::*;

pub struct WingsUiPlugin;

impl Plugin for WingsUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                EventListenerPlugin::<UiPointerClick>::default(),
                EventListenerPlugin::<UiPointerDoubleClick>::default(),
                EventListenerPlugin::<UiPointerRelease>::default(),
                EventListenerPlugin::<UiPointerPress>::default(),
                EventListenerPlugin::<UiPointerMove>::default(),
                EventListenerPlugin::<UiPointerOver>::default(),
                EventListenerPlugin::<UiPointerEnter>::default(),
                EventListenerPlugin::<UiPointerExit>::default(),
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
