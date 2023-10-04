use bevy::prelude::*;
use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};
use crate::prelude::UiWidgetBundle;
use crate::widgets::{UiColumn, UiRow};

#[derive(Copy, Clone, Debug, Default)]
pub struct UiColumnProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiColumnBundle {
    pub child: UiWidgetBundle,
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
                    flex_direction: FlexDirection::Column,
                    justify_content: to_justify(props.main_axis_alignment),
                    align_items: to_align(props.cross_axis_alignment),
                    ..default()
                },
                ..default()
            },
            internal_tag: UiColumn::default(),
        }
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
}

#[derive(Bundle, Clone, Debug)]
pub struct UiRowBundle {
    pub child: UiWidgetBundle,
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
                    flex_direction: FlexDirection::Row,
                    justify_content: to_justify(props.main_axis_alignment),
                    align_items: to_align(props.cross_axis_alignment),
                    ..default()
                },
                ..default()
            },
            internal_tag: UiRow::default(),
        }
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

pub(crate) fn to_main_size(value: MainAxisSize) -> Val {
    match value {
        MainAxisSize::Min => Val::Auto,
        MainAxisSize::Max => Val::Percent(100.),
    }
}

pub(crate) fn to_justify(value: MainAxisAlignment) -> JustifyContent {
    match value {
        MainAxisAlignment::Start => JustifyContent::FlexStart,
        MainAxisAlignment::End => JustifyContent::FlexEnd,
        MainAxisAlignment::Center => JustifyContent::Center,
        MainAxisAlignment::SpaceBetween => JustifyContent::SpaceBetween,
        MainAxisAlignment::SpaceEvenly => JustifyContent::SpaceEvenly,
        MainAxisAlignment::SpaceAround => JustifyContent::SpaceAround,
    }
}

pub(crate) fn to_align(value: CrossAxisAlignment) -> AlignItems {
    match value {
        CrossAxisAlignment::Start => AlignItems::FlexStart,
        CrossAxisAlignment::End => AlignItems::FlexEnd,
        CrossAxisAlignment::Center => AlignItems::Center,
        CrossAxisAlignment::Baseline => AlignItems::Baseline,
        CrossAxisAlignment::Stretch => AlignItems::Stretch,
    }
}
