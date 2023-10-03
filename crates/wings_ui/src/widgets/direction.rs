use bevy::prelude::*;
use crate::widgets::{UiColumn, UiRow, UiVisibility};
use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};
use crate::utils::{get_computed_display, get_computed_visibility};

#[derive(Copy, Clone, Debug, Default)]
pub struct UiColumnProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub visibility: UiVisibility,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiColumnBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
    internal_tag: UiColumn,
}

impl Default for UiColumnBundle {
    fn default() -> Self {
        UiColumnBundle::from(UiColumnProps::default())
    }
}

impl UiColumnBundle {
    fn from(props: UiColumnProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
            internal_tag: UiColumn::default(),
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiColumnProps { visibility, ..default() })
    }
}

#[derive(Copy, Clone, Debug, Default)]
pub struct UiRowProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub visibility: UiVisibility,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiRowBundle {
    pub child: NodeBundle,
    pub visibility: UiVisibility,
    internal_tag: UiRow,
}

impl Default for UiRowBundle {
    fn default() -> Self {
        UiRowBundle::from(UiRowProps::default())
    }
}

impl UiRowBundle {
    pub fn from(props: UiRowProps) -> Self {
        Self {
            child: NodeBundle {
                style: Style {
                    display: get_computed_display(&props.visibility),
                    flex_direction: FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                visibility: get_computed_visibility(&props.visibility),
                ..default()
            },
            visibility: props.visibility,
            internal_tag: UiRow::default(),
        }
    }

    pub fn from_visibility(visibility: UiVisibility) -> Self {
        Self::from(UiRowProps { visibility, ..default() })
    }
}
