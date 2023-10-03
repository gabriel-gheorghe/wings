use bevy::prelude::*;
use crate::events::*;
use crate::systems::*;

pub struct WingsUiPlugin;

impl Plugin for WingsUiPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<ApplyConstraintHeight>()
            .add_event::<ApplyConstraintWidth>()
            .add_systems(
                PreUpdate,
                (
                    compute_constraint_heights,
                    compute_constraint_widths,
                ),
            ).add_systems(
                Update,
                (
                    apply_constraint_height,
                    apply_constraint_width,
                ),
            );
    }
}
