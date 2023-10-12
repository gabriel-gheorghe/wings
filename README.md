# Wings UI
A new way to build User Interfaces on top of BevyUI.
This design is inspired from Flutter.

### Built-in Widgets
1. Scaffold
2. Container
3. SizedBox
4. Align
5. Center
6. FlatButton
7. Column
8. Row
9. ConstrainedWidth
10. ConstrainedHeight
11. HorizontalDivider
12. VerticalDivider
13. Visible
14. LayoutVisibility
15. Padding
16. Paragraph
17. GestureDetector

### Built-in Queries

1. Color <i>(background_color)</i>
2. Size <i>(width + height)</i>
3. Visible
4. LayoutVisibility
5. Text <i>(text, font_size, color)</i>

### Known limitations / Work in progress
1. Currently, you cannot create your own widgets
2. For/If statements inside widget tree are missing
3. Missing a lot of useful widgets such as ProgressIndicator, CheckBox, RadioButton, ToggleButton, Dropdown, TextEdit, SelectableText, ScrollArea, ListView, AppBar, Icon, Stack, Grid, Wrap and so on..
4. Animations capability
5. Alignment values have issues. Consider adding a system to process them
6. Events are not implemented properly, consider adding GestureDetector and Focus widgets

#### <u>Padding Example</u>

<p>
  <img src="./images/padding_example.png" width="400" title="hover text">
</p>

```rust
widget_tree!(
    Scaffold {
        child: Container {
            color: Some(Color::BLUE)
            width: val![500. px]
            height: val![500. px]
            child: Padding {
                padding: EdgeInsets::all(val![80. px])
                child: Container {
                    color: Some(Color::YELLOW)
                    width: val![100. %]
                    height: val![100. %]
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
widget_tree!(
    Scaffold {
        child: Center {
            child: Column {
                main_axis_size: MainAxisSize::Max
                main_axis_alignment: MainAxisAlignment::End
                cross_axis_alignment: CrossAxisAlignment::End
                children: [
                    Container {
                        color: Some(Color::RED)
                        width: val![300. px]
                    }
                    Container {
                        color: Some(Color::GREEN)
                        width: val![480. px]
                    }
                    Container {
                        color: Some(Color::BLUE)
                        width: val![200. px]
                    }
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
widget_tree!(
    Scaffold {
        child: Center {
            child: Row {
                main_axis_size: MainAxisSize::Max
                main_axis_alignment: MainAxisAlignment::End
                cross_axis_alignment: CrossAxisAlignment::End
                children: [
                    Container {
                        color: Some(Color::RED)
                        height: val![300. px]
                    }
                    Container {
                        color: Some(Color::GREEN)
                        height: val![480. px]
                    }
                    Container {
                        color: Some(Color::BLUE)
                        height: val![200. px]
                    }
                ]
            }
        }
    }
);
```

#### <u>Queries Example</u>

<p>
  <img src="./images/query_example.gif" width="400" title="hover text">
</p>

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

    widget_tree! {
        Scaffold {
            child: Container {
                decoration: Some(BoxDecoration {
                    color: Color::BLACK,
                    border: Border::all(BorderSide::from_width_color(val![15. px], Color::SILVER)),
                    ..default()
                })
                width: val![500. px]
                height: val![500. px]
                margin: edge_insets_only! {
                    left: val![100. px],
                    top: val![50. px],
                }
                child: Center {
                    child: Column {
                        children: [
                            Container { color: Some(Color::RED) }
                            SizedBox { height: val![10. px] }
                            Visible {
                                tags: [Collapsible]
                                child: Column {
                                    children: [
                                        Container {
                                            tags: [ColorTag]
                                            color: Some(Color::ORANGE)
                                        }
                                        SizedBox { height: val![10. px] }
                                    ]
                                }
                            }
                            Container { color: Some(Color::DARK_GREEN) }
                        ]
                    }
                }
            }
        }
    }
}

fn change_color(
    keyboard_input: Res<Input<KeyCode>>,
    mut color_query: ColorQuery<ColorTag>,
    mut visibility_query: VisibleQuery<Collapsible>,
) {
    if keyboard_input.just_pressed(KeyCode::C) {
        color_query.set_random_color();
    }

    if keyboard_input.just_pressed(KeyCode::V) {
        visibility_query.set_visible(|v| !v);
    }
}
```

#### <u>Gesture Detector Example</u>

Every time when you click on the first Container, all Containers in the Widget Tree will change their color.

```rust
widget_tree! {
    Center {
        child: GestureDetector {
            on_tap: on_tap! {
                |mut color_query: ColorQuery<Container>| {
                    color_query.set_random_color();
                }
            }
            child: Container {
                width: val![500. px]
                child: Align {
                    alignment: Alignment::CENTER_RIGHT
                    child: Container {}
                }
            }
        }
    }
}
```

### Some queries tips for applying a theme:

! When working with WingsUI, Do not access BevyUI components directly because of unexpected behaviour !
Instead, use built-in Queries !

```rust
// This is bad use
fn apply_theme_bad(mut query: Query<(&mut BackgroundColor, With<ContainerWidget>)>) {
    for (mut bg_color, _) in query.iter_mut() {
        bg_color.0 = get_random_color();
    }
}

// You can do this if you want more control over entities, but still not recommended
fn apply_theme_also_bad(mut query: ColorQuery<ContainerWidget>) {
    query.get_mut().for_each_mut(|(_, mut c_color, _)| {
        c_color.0 = get_random_color();
    });
}

// This is the most ergonomic way
fn apply_theme_good(mut query: ColorQuery<ContainerWidget>) {
    query.set_random_color();
}
```