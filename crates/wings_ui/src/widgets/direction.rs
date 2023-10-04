use bevy::prelude::*;
use crate::widgets::{UiColumn, UiRow, UiVisibility};
use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};
use crate::prelude::UiWidgetBundle;
use crate::utils::{
    get_computed_display, get_computed_visibility, to_align, to_justify, to_main_size,
};

#[derive(Copy, Clone, Debug, Default)]
pub struct UiColumnProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
    pub visibility: UiVisibility,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiColumnBundle {
    pub child: UiWidgetBundle,
    pub visibility: UiVisibility,
    internal_tag: UiColumn,
}

impl Default for UiColumnBundle {
    fn default() -> Self {
        UiColumnBundle::from(UiColumnProps::default())
    }
}

impl UiColumnBundle {
    pub fn from(props: UiColumnProps) -> Self {
        Self {
            child: UiWidgetBundle {
                style: Style {
                    height: to_main_size(props.main_axis_size),
                    display: get_computed_display(&props.visibility),
                    flex_direction: FlexDirection::Column,
                    justify_content: to_justify(props.main_axis_alignment),
                    align_items: to_align(props.cross_axis_alignment),
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

    pub fn from_main_axis(
        main_axis_size: MainAxisSize,
        main_axis_alignment: MainAxisAlignment,
    ) -> Self {
        Self::from(UiColumnProps { main_axis_size, main_axis_alignment, ..default() })
    }

    pub fn from_cross_axis(cross_axis_alignment: CrossAxisAlignment) -> Self {
        Self::from(UiColumnProps { cross_axis_alignment, ..default() })
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
    pub child: UiWidgetBundle,
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
            child: UiWidgetBundle {
                style: Style {
                    width: to_main_size(props.main_axis_size),
                    display: get_computed_display(&props.visibility),
                    flex_direction: FlexDirection::Row,
                    justify_content: to_justify(props.main_axis_alignment),
                    align_items: to_align(props.cross_axis_alignment),
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

    pub fn from_main_axis(
        main_axis_size: MainAxisSize,
        main_axis_alignment: MainAxisAlignment,
    ) -> Self {
        Self::from(UiRowProps { main_axis_size, main_axis_alignment, ..default() })
    }

    pub fn from_cross_axis(cross_axis_alignment: CrossAxisAlignment) -> Self {
        Self::from(UiRowProps { cross_axis_alignment, ..default() })
    }
}
