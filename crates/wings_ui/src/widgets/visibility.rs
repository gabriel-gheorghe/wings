use bevy::prelude::*;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct VisibleWidget(pub bool);

#[derive(Bundle, Clone, Debug)]
pub struct VisibleBundle {
    child: WidgetBundle,
    widget: VisibleWidget,
}

impl Default for VisibleBundle {
    #[inline]
    fn default() -> Self {
        VisibleBundle::from(true)
    }
}

impl VisibleBundle {
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
            widget: VisibleWidget(visible),
        }
    }
}

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum LayoutVisibilityWidget {
    #[default]
    Inherited,
    Visible,
    Hidden,
    Collapsed,
}

#[derive(Bundle, Clone, Debug)]
pub struct LayoutVisibilityBundle {
    pub child: WidgetBundle,
    pub visibility: LayoutVisibilityWidget,
}

impl Default for LayoutVisibilityBundle {
    #[inline]
    fn default() -> Self {
        LayoutVisibilityBundle::from(LayoutVisibilityWidget::Inherited)
    }
}

impl LayoutVisibilityBundle {
    #[inline]
    pub fn from(visibility: LayoutVisibilityWidget) -> Self {
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
pub(crate) fn get_computed_display(visibility: &LayoutVisibilityWidget) -> Display {
    match visibility {
        LayoutVisibilityWidget::Collapsed => Display::None,
        _ => Display::Flex,
    }
}

#[inline]
pub(crate) fn get_computed_visibility(visibility: &LayoutVisibilityWidget) -> Visibility {
    match visibility {
        LayoutVisibilityWidget::Visible => Visibility::Visible,
        LayoutVisibilityWidget::Hidden => Visibility::Hidden,
        _ => Visibility::Inherited,
    }
}
