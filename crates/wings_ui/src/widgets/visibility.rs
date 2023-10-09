use bevy::prelude::*;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiVisibility(pub bool);

#[derive(Bundle, Clone, Debug)]
pub struct UiVisibilityBundle {
    child: WidgetBundle,
    widget: UiVisibility,
}

impl Default for UiVisibilityBundle {
    #[inline]
    fn default() -> Self {
        UiVisibilityBundle::from(true)
    }
}

impl UiVisibilityBundle {
    #[inline]
    pub fn from(visible: bool) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    display: if visible { Display::Flex } else { Display::None },
                    ..default()
                },
                ..default()
            },
            widget: UiVisibility(visible),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum UiLayoutVisibility {
    #[default]
    Inherited,
    Visible,
    Hidden,
    Collapsed,
}

#[derive(Bundle, Clone, Debug)]
pub struct UiLayoutVisibilityBundle {
    pub child: WidgetBundle,
    pub visibility: UiLayoutVisibility,
}

impl Default for UiLayoutVisibilityBundle {
    #[inline]
    fn default() -> Self {
        UiLayoutVisibilityBundle::from(UiLayoutVisibility::Inherited)
    }
}

impl UiLayoutVisibilityBundle {
    #[inline]
    pub fn from(visibility: UiLayoutVisibility) -> Self {
        Self {
            child: WidgetBundle {
                style: Style {
                    display: get_computed_display(&visibility),
                    ..default()
                },
                visibility: get_computed_visibility(&visibility),
                ..default()
            },
            visibility,
        }
    }
}

#[inline]
pub(crate) fn get_computed_display(visibility: &UiLayoutVisibility) -> Display {
    match visibility {
        UiLayoutVisibility::Collapsed => Display::None,
        _ => Display::Flex,
    }
}

#[inline]
pub(crate) fn get_computed_visibility(visibility: &UiLayoutVisibility) -> Visibility {
    match visibility {
        UiLayoutVisibility::Visible => Visibility::Visible,
        UiLayoutVisibility::Hidden => Visibility::Hidden,
        _ => Visibility::Inherited,
    }
}
