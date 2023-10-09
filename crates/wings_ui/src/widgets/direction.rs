use bevy::prelude::*;
use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiColumn;

#[derive(Copy, Clone, Debug, Default)]
pub struct UiColumnProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiColumnBundle {
    pub child: WidgetBundle,
    internal_tag: UiColumn,
}

impl Default for UiColumnBundle {
    #[inline]
    fn default() -> Self {
        UiColumnBundle::from(UiColumnProps::default())
    }
}

impl UiColumnBundle {
    #[inline]
    pub fn from(props: UiColumnProps) -> Self {
        Self {
            child: WidgetBundle {
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

    #[inline]
    pub fn from_main_axis(
        main_axis_size: MainAxisSize,
        main_axis_alignment: MainAxisAlignment,
    ) -> Self {
        Self::from(UiColumnProps { main_axis_size, main_axis_alignment, ..default() })
    }

    #[inline]
    pub fn from_cross_axis(cross_axis_alignment: CrossAxisAlignment) -> Self {
        Self::from(UiColumnProps { cross_axis_alignment, ..default() })
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct UiRow;

#[derive(Copy, Clone, Debug, Default)]
pub struct UiRowProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiRowBundle {
    pub child: WidgetBundle,
    internal_tag: UiRow,
}

impl Default for UiRowBundle {
    #[inline]
    fn default() -> Self {
        UiRowBundle::from(UiRowProps::default())
    }
}

impl UiRowBundle {
    #[inline]
    pub fn from(props: UiRowProps) -> Self {
        Self {
            child: WidgetBundle {
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

    #[inline]
    pub fn from_main_axis(
        main_axis_size: MainAxisSize,
        main_axis_alignment: MainAxisAlignment,
    ) -> Self {
        Self::from(UiRowProps { main_axis_size, main_axis_alignment, ..default() })
    }

    #[inline]
    pub fn from_cross_axis(cross_axis_alignment: CrossAxisAlignment) -> Self {
        Self::from(UiRowProps { cross_axis_alignment, ..default() })
    }
}

#[inline]
pub(crate) fn to_main_size(value: MainAxisSize) -> Val {
    match value {
        MainAxisSize::Min => Val::Auto,
        MainAxisSize::Max => Val::Percent(100.),
    }
}

#[inline]
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

#[inline]
pub(crate) fn to_align(value: CrossAxisAlignment) -> AlignItems {
    match value {
        CrossAxisAlignment::Start => AlignItems::FlexStart,
        CrossAxisAlignment::End => AlignItems::FlexEnd,
        CrossAxisAlignment::Center => AlignItems::Center,
        CrossAxisAlignment::Baseline => AlignItems::Baseline,
        CrossAxisAlignment::Stretch => AlignItems::Stretch,
    }
}
