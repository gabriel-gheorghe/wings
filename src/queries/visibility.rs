use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use crate::visibility::UiVisibility;

#[derive(SystemParam)]
pub struct UiVisibilityQuery<'w, 's, T: Component>(
    Query<'w, 's,
        (
            &'static mut Style,
            &'static mut Visibility,
            &'static mut UiVisibility,
            With<T>,
        )
    >,
);

impl <'w, 's, T: Component> UiVisibilityQuery<'w, 's, T> {
    pub fn inherit(&mut self) {
        self.0.for_each_mut(|(mut c_style, mut c_visibility, mut ui_visibility, _)| {
            if ui_visibility.is_collapsed {
                c_style.width = ui_visibility.cached_width;
                c_style.height = ui_visibility.cached_height;
                ui_visibility.is_collapsed = false;
            }

            *c_visibility = Visibility::Inherited;
        });
    }

    pub fn show(&mut self) {
        self.0.for_each_mut(|(mut c_style, mut c_visibility, mut ui_visibility, _)| {
            if ui_visibility.is_collapsed {
                c_style.width = ui_visibility.cached_width;
                c_style.height = ui_visibility.cached_height;
                ui_visibility.is_collapsed = false;
            }

            *c_visibility = Visibility::Visible;
        });
    }

    pub fn hide(&mut self) {
        self.0.for_each_mut(|(mut c_style, mut c_visibility, mut ui_visibility, _)| {
            if ui_visibility.is_collapsed {
                c_style.width = ui_visibility.cached_width;
                c_style.height = ui_visibility.cached_height;
                ui_visibility.is_collapsed = false;
            }

            *c_visibility = Visibility::Hidden;
        });
    }

    pub fn collapse(&mut self) {
        self.0.for_each_mut(|(mut c_style, mut c_visibility, mut ui_visibility, _)| {
            if ui_visibility.is_collapsed { return; }

            ui_visibility.cached_width = c_style.width;
            ui_visibility.cached_height = c_style.height;
            ui_visibility.is_collapsed = true;

            c_style.width = Val::Px(0.0);
            c_style.height = Val::Px(0.0);
            *c_visibility = Visibility::Hidden;
        });
    }
}
