# Wings UI
A new way to build User Interfaces on top of BevyUI.
This design is inspired from Flutter.

### Built-in Widgets
1. Scaffold
2. Container
3. SizedBox
4. Align
5. Center
6. Button
7. Column
8. Row
9. ConstrainedWidth
10. ConstrainedHeight
11. HorizontalDivider
12. VerticalDivider
13. Visibility
14. LayoutVisibility
15. Padding

### Built-in Queries

1. Color
2. Size
3. Visibility
4. LayoutVisibility

#### <u>Padding Example</u>

<p>
  <img src="./images/padding_example.png" width="400" title="hover text">
</p>

```rust
widget_tree!(&mut commands,
    Scaffold {
        :Container {
            color: Color::BLUE,
            width: Val::Px(500.),
            height: Val::Px(500.),
            :Padding {
                padding: EdgeInsets::all(Val::Px(80.)),
                :Container {
                    color: Color::YELLOW,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                }
            }
        }
    }
);
```

#### <u>Column Example</u>

<p>
  <img src="./images/column_example.png" width="400" title="hover text">
</p>

```rust
widget_tree!(&mut commands,
    Scaffold {
        :Center {
            :Column {
                main_axis_size: MainAxisSize::Max,
                main_axis_alignment: MainAxisAlignment::End,
                cross_axis_alignment: CrossAxisAlignment::End,
                :[
                    Container {
                        color: Color::RED,
                        width: Val::Px(300.),
                    },
                    Container {
                        color: Color::GREEN,
                        width: Val::Px(480.),
                    },
                    Container {
                        color: Color::BLUE,
                        width: Val::Px(200.),
                    },
                ]
            }
        }
    }
);
```

#### <u>Row Example</u>

<p>
  <img src="./images/row_example.png" width="400" title="hover text">
</p>

```rust
widget_tree!(&mut commands,
    Scaffold {
        :Center {
            :Row {
                main_axis_size: MainAxisSize::Max,
                main_axis_alignment: MainAxisAlignment::End,
                cross_axis_alignment: CrossAxisAlignment::End,
                :[
                    Container {
                        color: Color::RED,
                        height: Val::Px(300.),
                    },
                    Container {
                        color: Color::GREEN,
                        height: Val::Px(480.),
                    },
                    Container {
                        color: Color::BLUE,
                        height: Val::Px(200.),
                    },
                ]
            }
        }
    }
);
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
                    UiCollapsible,
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
    mut visibility_query: UiVisibilityQuery<UiCollapsible>,
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

<i>Desired design - Work in Progress</i>

```rust
widget_tree!(&mut commands,
    in {
        let first_container = Container {
            width: Val::Px(50.0),
            height: Val::Px(50.0),
            color: Color::RED,
        };
    }
    
    Scaffold {
        :Center {
            :Row {
                :[
                    first_container,
                    :Visibility use Collapsible {
                        :Row {
                            :[
                                for i in 0..2 {
                                    SizedBox {
                                        width: Val::Px(50.),
                                    },
                                    Container use ColorTag,
                                }
                            ]
                        }
                    }
                ]
            }
        }
    }
);
```
