# Wings UI
A new way to build User Interfaces on top of BevyUI.

#### Example

```rust
use bevy::prelude::*;
use wings::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WingsPlugin)
        .add_systems(Startup, startup)
        .add_systems(Update, change_color)
        .run();
}

#[derive(Component, Debug)]
pub struct ColorTag;

fn startup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let first_container_props = UiContainerProps {
        width: Val::Px(50.0),
        height: Val::Px(50.0),
        color: Color::RED,
        is_collapsed: false,
    };

    commands.spawn(UiScreen::from(UiScreenProps{
        centered: true,
        ..default()
    })).with_children(|parent| {
        parent.spawn(UiRow::default()).with_children(|parent| {
            parent.spawn(UiContainer::from(first_container_props));
            parent.spawn((UiTagCollapsible, UiHorizontalDivider::from_width(Val::Px(50.0))));
            parent.spawn((UiTagCollapsible, ColorTag, UiContainer::default()));
            parent.spawn((UiTagCollapsible, UiHorizontalDivider::from_width(Val::Px(50.0))));
            parent.spawn((UiTagCollapsible, ColorTag, UiContainer::default()));
        });
    });
}

fn change_color(
    keyboard_input: Res<Input<KeyCode>>,
    mut color_query: UiColorQuery<ColorTag>,
    mut visibility_query: UiVisibilityQuery<UiTagCollapsible>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        color_query.set_random_color();
    }

    if keyboard_input.just_pressed(KeyCode::Z) {
        visibility_query.collapse();
    } else if keyboard_input.just_pressed(KeyCode::X) {
        visibility_query.inherit();
    }
}
```