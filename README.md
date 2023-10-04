# Wings UI
A new way to build User Interfaces on top of BevyUI.
This design is inspired from Flutter.

### Built-in Widgets
1. Scaffold
2. Container
3. SizedBox
4. Center
5. Button
6. Column
7. Row
8. ConstrainedWidth
9. ConstrainedHeight
10. HorizontalDivider
11. VerticalDivider
12. Visibility
13. LayoutVisibility

### Built-in Queries

1. Color
2. Size
3. Visibility
4. LayoutVisibility

#### <u>Column Example</u>

<p>
  <img src="./images/column_example.png" width="400" title="hover text">
</p>

```rust
fn build_ui(mut commands: Commands) {
    commands.spawn(
        UiScaffoldBundle::from(UiScaffoldProps::default()),
    ).with_children(|parent| {
        parent.spawn(UiCenterBundle::default()).with_children(|parent| {
            parent.spawn(
                UiColumnBundle::from(UiColumnProps {
                    main_axis_size: MainAxisSize::Max,
                    main_axis_alignment: MainAxisAlignment::End,
                    cross_axis_alignment: CrossAxisAlignment::End,
                    ..default()
                }),
            ).with_children(|parent| {
                parent.spawn(
                    UiContainerBundle::from(UiContainerProps {
                        color: Color::RED,
                        width: Val::Px(300.),
                        ..default()}),
                );
                parent.spawn(
                    UiContainerBundle::from(UiContainerProps {
                        color: Color::GREEN,
                        width: Val::Px(480.),
                        ..default()}),
                );
                parent.spawn(
                    UiContainerBundle::from(UiContainerProps {
                        color: Color::BLUE,
                        width: Val::Px(200.),
                        ..default()}),
                );
            });
        });
    });
}
```

#### <u>Row Example</u>

<p>
  <img src="./images/row_example.png" width="400" title="hover text">
</p>

```rust
fn build_ui(mut commands: Commands) {
    commands.spawn(
        UiScaffoldBundle::from(UiScaffoldProps::default()),
    ).with_children(|parent| {
        parent.spawn(UiCenterBundle::default()).with_children(|parent| {
            parent.spawn(
                UiRowBundle::from(UiRowProps {
                    main_axis_size: MainAxisSize::Max,
                    main_axis_alignment: MainAxisAlignment::End,
                    cross_axis_alignment: CrossAxisAlignment::End,
                    ..default()
                }),
            ).with_children(|parent| {
                parent.spawn(
                    UiContainerBundle::from(UiContainerProps {
                        color: Color::RED,
                        height: Val::Px(300.),
                        ..default()}),
                );
                parent.spawn(
                    UiContainerBundle::from(UiContainerProps {
                        color: Color::GREEN,
                        height: Val::Px(480.),
                        ..default()}),
                );
                parent.spawn(
                    UiContainerBundle::from(UiContainerProps {
                        color: Color::BLUE,
                        height: Val::Px(200.),
                        ..default()}),
                );
            });
        });
    });
}
```

#### <u>Queries Example</u>

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

    commands.spawn(
        UiScaffoldBundle::default(),
    ).with_children(|parent| {
        parent.spawn(
            UiCenterBundle::default(),
        ).with_children(|parent| {
            parent.spawn(
                UiRowBundle::default(),
            ).with_children(|parent| {
                parent.spawn(UiContainerBundle::from(first_container_props));
                parent.spawn((
                    UiTagCollapsible,
                    UiVisibilityBundle::default(),
                )).with_children(|parent| {
                    parent.spawn(UiRowBundle::default()).with_children(|parent| {
                        parent.spawn(UiSizedBoxBundle::from_width(Val::Px(50.0)));
                        parent.spawn(
                            (
                                ColorTag,
                                UiContainerBundle::default(),
                            ),
                        );
                        parent.spawn(UiSizedBoxBundle::from_width(Val::Px(50.0)));
                        parent.spawn(
                            (
                                ColorTag,
                                UiContainerBundle::default(),
                            ),
                        );
                    });
                });
            });
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
        visibility_query.set(false);
    } else if keyboard_input.just_pressed(KeyCode::X) {
        visibility_query.set(true);
    }
}
```
