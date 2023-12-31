use bevy::prelude::*;
use crate::events::{
    ApplyConstraintHeight, ApplyConstraintWidth,
};
use crate::prelude::{ConstrainedHeightWidget, ConstrainedWidthWidget};

pub(crate) fn compute_constraint_heights(
    mut event: EventWriter<ApplyConstraintHeight>,
    mut constrained_height_q: Query<(Entity, With<ConstrainedHeightWidget>)>,
    style_q: Query<&Style>,
    children_q: Query<&Children>,
) {
    for (target, _) in constrained_height_q.iter_mut() {
        let mut max_height = 0.;
        for child in children_q.iter_descendants(target) {
            let style = style_q.get_component::<Style>(child).unwrap();

            match style.height {
                Val::Px(height) => {
                    if height > max_height {
                        max_height = height;
                    }
                }
                _ => {}
            }
        }
        if max_height != 0. {
            event.send(ApplyConstraintHeight(target, max_height));
        }
    }
}

pub(crate) fn apply_constraint_height(
    mut event: EventReader<ApplyConstraintHeight>,
    mut row_q: Query<&mut Style>,
) {
    for e in event.iter() {
        let mut row = row_q.get_component_mut::<Style>(e.0).unwrap();
        row.height = Val::Px(e.1);
    }
}

pub(crate) fn compute_constraint_widths(
    mut event: EventWriter<ApplyConstraintWidth>,
    mut constrained_width_q: Query<(Entity, With<ConstrainedWidthWidget>)>,
    style_q: Query<&Style>,
    children_q: Query<&Children>,
) {
    for (target, _) in constrained_width_q.iter_mut() {
        let mut max_width = 0.;
        for child in children_q.iter_descendants(target) {
            let style = style_q.get_component::<Style>(child).unwrap();

            match style.width {
                Val::Px(width) => {
                    if width > max_width {
                        max_width = width;
                    }
                }
                _ => {}
            }
        }
        if max_width != 0. {
            event.send(ApplyConstraintWidth(target, max_width));
        }
    }
}

pub(crate) fn apply_constraint_width(
    mut event: EventReader<ApplyConstraintWidth>,
    mut row_q: Query<&mut Style>,
) {
    for e in event.iter() {
        let mut row = row_q.get_component_mut::<Style>(e.0).unwrap();
        row.width = Val::Px(e.1);
    }
}
