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
        ..default()
    };

    commands.spawn(UiScreenBundle::from(UiScreenProps {
        centered: true,
        ..default()
    })).with_children(|parent| {
        parent.spawn(UiRowBundle::default()).with_children(|parent| {
            parent.spawn(UiContainerBundle::from(first_container_props));
            parent.spawn((UiTagCollapsible, UiHorizontalDividerBundle::from_width(Val::Px(50.0))));
            parent.spawn((UiTagCollapsible, ColorTag, UiContainerBundle::default()));
            parent.spawn((UiTagCollapsible, UiHorizontalDividerBundle::from_width(Val::Px(50.0))));
            parent.spawn((UiTagCollapsible, ColorTag, UiContainerBundle::default()));
        });
    });
}

fn change_color(
    keyboard_input: Res<Input<KeyCode>>,
    mut color_query: UiColorQuery<ColorTag>,
    mut visibility_query: UiVisibilityQuery<UiTagCollapsible>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        color_query.set_random();
    }

    if keyboard_input.just_pressed(KeyCode::Z) {
        visibility_query.set(UiVisibility::Collapsed);
    } else if keyboard_input.just_pressed(KeyCode::X) {
        visibility_query.set(UiVisibility::Inherited);
    }
}
```