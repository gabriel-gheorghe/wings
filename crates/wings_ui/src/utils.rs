use bevy::prelude::*;
use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};
use crate::prelude::UiVisibility;

pub(crate) fn get_computed_display(visibility: &UiVisibility) -> Display {
    match visibility {
        UiVisibility::Collapsed => Display::None,
        _ => Display::Flex,
    }
}

pub(crate) fn get_computed_visibility(visibility: &UiVisibility) -> Visibility {
    match visibility {
        UiVisibility::Visible => Visibility::Visible,
        UiVisibility::Hidden => Visibility::Hidden,
        _ => Visibility::Inherited,
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
