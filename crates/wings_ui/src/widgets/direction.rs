use bevy::prelude::*;
use crate::enums::{UiCrossAxisAlignment, UiMainAxisAlignment, UiMainAxisSize};
use crate::widgets::UiWidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiColumn;

#[derive(Copy, Clone, Debug, Default)]
pub struct UiColumnProps {
    pub main_axis_size: UiMainAxisSize,
    pub main_axis_alignment: UiMainAxisAlignment,
    pub cross_axis_alignment: UiCrossAxisAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiColumnBundle {
    pub child: UiWidgetBundle,
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

    #[inline]
    pub fn from_main_axis(
        main_axis_size: UiMainAxisSize,
        main_axis_alignment: UiMainAxisAlignment,
    ) -> Self {
        Self::from(UiColumnProps { main_axis_size, main_axis_alignment, ..default() })
    }

    #[inline]
    pub fn from_cross_axis(cross_axis_alignment: UiCrossAxisAlignment) -> Self {
        Self::from(UiColumnProps { cross_axis_alignment, ..default() })
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct UiRow;

#[derive(Copy, Clone, Debug, Default)]
pub struct UiRowProps {
    pub main_axis_size: UiMainAxisSize,
    pub main_axis_alignment: UiMainAxisAlignment,
    pub cross_axis_alignment: UiCrossAxisAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiRowBundle {
    pub child: UiWidgetBundle,
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

    #[inline]
    pub fn from_main_axis(
        main_axis_size: UiMainAxisSize,
        main_axis_alignment: UiMainAxisAlignment,
    ) -> Self {
        Self::from(UiRowProps { main_axis_size, main_axis_alignment, ..default() })
    }

    #[inline]
    pub fn from_cross_axis(cross_axis_alignment: UiCrossAxisAlignment) -> Self {
        Self::from(UiRowProps { cross_axis_alignment, ..default() })
    }
}

#[inline]
pub(crate) fn to_main_size(value: UiMainAxisSize) -> Val {
    match value {
        UiMainAxisSize::Min => Val::Auto,
        UiMainAxisSize::Max => Val::Percent(100.),
    }
}

#[inline]
pub(crate) fn to_justify(value: UiMainAxisAlignment) -> JustifyContent {
    match value {
        UiMainAxisAlignment::Start => JustifyContent::FlexStart,
        UiMainAxisAlignment::End => JustifyContent::FlexEnd,
        UiMainAxisAlignment::Center => JustifyContent::Center,
        UiMainAxisAlignment::SpaceBetween => JustifyContent::SpaceBetween,
        UiMainAxisAlignment::SpaceEvenly => JustifyContent::SpaceEvenly,
        UiMainAxisAlignment::SpaceAround => JustifyContent::SpaceAround,
    }
}

#[inline]
pub(crate) fn to_align(value: UiCrossAxisAlignment) -> AlignItems {
    match value {
        UiCrossAxisAlignment::Start => AlignItems::FlexStart,
        UiCrossAxisAlignment::End => AlignItems::FlexEnd,
        UiCrossAxisAlignment::Center => AlignItems::Center,
        UiCrossAxisAlignment::Baseline => AlignItems::Baseline,
        UiCrossAxisAlignment::Stretch => AlignItems::Stretch,
    }
}
