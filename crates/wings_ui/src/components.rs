use bevy::prelude::*;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum UiVisibility {
    #[default]
    Inherited,
    Visible,
    Hidden,
    Collapsed,
}

#[derive(Component, Clone, Debug, Default)]
pub struct UiButton;

#[derive(Component, Clone, Debug, Default)]
pub struct UiContainer;

#[derive(Component, Clone, Debug, Default)]
pub struct UiColumn;

#[derive(Component, Clone, Debug, Default)]
pub struct UiRow;

#[derive(Component, Clone, Debug, Default)]
pub struct UiScreen;
