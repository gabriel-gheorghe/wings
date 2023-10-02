use bevy::prelude::*;
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
