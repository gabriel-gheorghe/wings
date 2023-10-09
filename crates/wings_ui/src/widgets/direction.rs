use bevy::prelude::*;
use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct ColumnWidget;

#[derive(Copy, Clone, Debug, Default)]
pub struct ColumnProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct ColumnBundle {
    child: WidgetBundle,
    widget: ColumnWidget,
}

impl Default for ColumnBundle {
    #[inline]
    fn default() -> Self {
        ColumnBundle::from(ColumnProps::default())
    }
}

impl ColumnBundle {
    #[inline]
    pub fn from(props: ColumnProps) -> Self {
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
            widget: ColumnWidget::default(),
        }
    }

    #[inline]
    pub fn from_main_axis(
        main_axis_size: MainAxisSize,
        main_axis_alignment: MainAxisAlignment,
    ) -> Self {
        Self::from(ColumnProps { main_axis_size, main_axis_alignment, ..default() })
    }

    #[inline]
    pub fn from_cross_axis(cross_axis_alignment: CrossAxisAlignment) -> Self {
        Self::from(ColumnProps { cross_axis_alignment, ..default() })
    }
}

#[derive(Component, Clone, Debug, Default)]
pub struct RowWidget;

#[derive(Copy, Clone, Debug, Default)]
pub struct RowProps {
    pub main_axis_size: MainAxisSize,
    pub main_axis_alignment: MainAxisAlignment,
    pub cross_axis_alignment: CrossAxisAlignment,
}

#[derive(Bundle, Clone, Debug)]
pub struct RowBundle {
    child: WidgetBundle,
    widget: RowWidget,
}

impl Default for RowBundle {
    #[inline]
    fn default() -> Self {
        RowBundle::from(RowProps::default())
    }
}

impl RowBundle {
    #[inline]
    pub fn from(props: RowProps) -> Self {
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
            widget: RowWidget::default(),
        }
    }

    #[inline]
    pub fn from_main_axis(
        main_axis_size: MainAxisSize,
        main_axis_alignment: MainAxisAlignment,
    ) -> Self {
        Self::from(RowProps { main_axis_size, main_axis_alignment, ..default() })
    }

    #[inline]
    pub fn from_cross_axis(cross_axis_alignment: CrossAxisAlignment) -> Self {
        Self::from(RowProps { cross_axis_alignment, ..default() })
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
