use bevy::prelude::*;

pub mod alignment;
pub mod button;
pub mod constrained;
pub mod container;
pub mod direction;
pub mod divider;
pub mod scaffold;

#[derive(Component, Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum UiVisibility {
    #[default]
    Inherited,
    Visible,
    Hidden,
    Collapsed,
}

#[derive(Component, Clone, Debug, Default)]
pub struct UiCenter;

#[derive(Component, Clone, Debug, Default)]
pub struct UiConstrainedHeight;

#[derive(Component, Clone, Debug, Default)]
pub struct UiConstrainedWidth;

#[derive(Component, Clone, Debug, Default)]
pub struct UiButton;

#[derive(Component, Clone, Debug, Default)]
pub struct UiContainer;

#[derive(Component, Clone, Debug, Default)]
pub struct UiColumn;

#[derive(Component, Clone, Debug, Default)]
pub struct UiRow;

#[derive(Component, Clone, Debug, Default)]
pub struct UiHorizontalDivider;

#[derive(Component, Clone, Debug, Default)]
pub struct UiVerticalDivider;

#[derive(Component, Clone, Debug, Default)]
pub struct UiScaffold;
