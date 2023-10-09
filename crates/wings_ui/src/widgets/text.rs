use bevy::prelude::*;
use bevy::text::{DEFAULT_FONT_HANDLE, TextLayoutInfo};
use bevy::ui::ContentSize;
use bevy::ui::widget::TextFlags;
use crate::widgets::WidgetBundle;

#[derive(Component, Clone, Debug, Default)]
pub struct UiText;

#[derive(Clone, Debug)]
pub struct UiTextProps {
    pub text: String,
    pub font: Handle<Font>,
    pub font_size: f32,
    pub color: Color,
}

impl Default for UiTextProps {
    #[inline]
    fn default() -> Self {
        Self {
            text: "Empty".to_string(),
            font: DEFAULT_FONT_HANDLE.typed(),
            font_size: 24.,
            color: Color::WHITE,
        }
    }
}

#[derive(Bundle, Debug)]
pub struct UiTextBundle {
    pub child: WidgetBundle,
    pub text: Text,
    pub text_layout_info: TextLayoutInfo,
    pub text_flags: TextFlags,
    pub calculated_size: ContentSize,
    internal_tag: UiText,
}

impl Default for UiTextBundle {
    fn default() -> Self {
        UiTextBundle::from(UiTextProps::default())
    }
}

impl UiTextBundle {
    #[inline]
    pub fn from(props: UiTextProps) -> Self {
        Self {
            child: WidgetBundle::default(),
            text: Text {
                sections: vec![TextSection {
                    value: props.text.to_string(),
                    style: TextStyle {
                        font: props.font,
                        font_size: props.font_size,
                        color: props.color,
                    },
                }],
                ..default()
            },
            text_layout_info: TextLayoutInfo::default(),
            text_flags: TextFlags::default(),
            calculated_size: ContentSize::default(),
            internal_tag: UiText::default(),
        }
    }

    #[inline]
    pub fn from_text(text: String) -> Self {
        Self::from(UiTextProps {
            text,
            ..default()
        })
    }

    #[inline]
    pub fn from_text_color(text: String, color: Color) -> Self {
        Self::from(UiTextProps {
            text,
            color,
            ..default()
        })
    }
}
