use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(Copy, Clone, Debug)]
pub struct UiButtonProps {
    pub width: Val,
    pub height: Val,
    pub color: Color,
    pub is_collapsed: bool,
}

impl Default for UiButtonProps {
    fn default() -> Self {
        Self {
            width: Val::Px(210.0),
            height: Val::Px(90.0),
            color: Color::TEAL,
            is_collapsed: false,
        }
    }
}

#[derive(Bundle, Clone, Debug)]
pub struct UiButton {
    pub child: ButtonBundle,
    pub visibility: UiVisibility,
}

impl Default for UiButton {
    fn default() -> Self {
        UiButton::from(UiButtonProps::default())
    }
}

impl UiButton {
    pub fn from(props: UiButtonProps) -> Self {
        Self {
            child: ButtonBundle {
                style: Style {
                    width: if props.is_collapsed { Val::Px(0.0) } else { props.width },
                    height: if props.is_collapsed { Val::Px(0.0) } else { props.height },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    border: UiRect::all(Val::Px(5.0)),
                    ..default()
                },
                background_color: BackgroundColor::from(props.color),
                border_color: BorderColor(Color::BLACK),
                visibility: if props.is_collapsed { Visibility::Hidden } else { Visibility::Inherited },
                ..default()
            },
            visibility: UiVisibility {
                cached_width: props.width,
                cached_height: props.height,
                is_collapsed: props.is_collapsed,
            },
        }
    }
}
