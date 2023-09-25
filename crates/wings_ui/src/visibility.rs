use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub struct UiVisibility {
    pub(crate) cached_width: Val,
    pub(crate) cached_height: Val,
    pub(crate) is_collapsed: bool,
}

impl Default for UiVisibility {
    fn default() -> Self {
        Self {
            cached_width: Val::Auto,
            cached_height: Val::Auto,
            is_collapsed: false,
        }
    }
}

impl UiVisibility {
    pub fn from_width(width: Val) -> Self {
        Self {
            cached_width: width,
            ..default()
        }
    }

    pub fn from_height(height: Val) -> Self {
        Self {
            cached_height: height,
            ..default()
        }
    }

    pub fn from_size(width: Val, height: Val) -> Self {
        Self {
            cached_width: width,
            cached_height: height,
            ..default()
        }
    }

    pub fn from_width_and_collapsed(width: Val, is_collapsed: bool) -> Self {
        Self {
            cached_width: width,
            is_collapsed,
            ..default()
        }
    }

    pub fn from_height_and_collapsed(height: Val, is_collapsed: bool) -> Self {
        Self {
            cached_height: height,
            is_collapsed,
            ..default()
        }
    }
}
