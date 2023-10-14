# ✈️ Wings UI
### A new way to build User Interfaces on top of <u>BevyUI</u>.
The design is inspired from <u>Flutter</u>.
This is the most ergonomic <b>ECS</b> Data Driven UI Framework.

---
### 🦅‍ Bevy Version Support

| bevy | wings |
|------|-------|
| 0.11 | 0.1   |

---
### 🛩 Built-in Widgets
1. `Scaffold`
2. `Container`
3. `SizedBox`
4. `Align`
5. `Center`
6. `FlatButton`
7. `Column`
8. `Row`
9. `ConstrainedWidth`
10. `ConstrainedHeight`
11. `HorizontalDivider`
12. `VerticalDivider`
13. `Visible`
14. `LayoutVisibility`
15. `Padding`
16. `Paragraph`
17. `GestureDetector` A widget that detects gestures.

---
### 🛩 Built-in Queries

1. `ColorQuery` (`background_color`)
2. `SizeQuery` (`width` & `height`)
3. `VisibleQuery`
4. `LayoutVisibilityQuery`
5. `TextQuery` (`text` & `font_size` & `color`)

---
### 🛩 Gesture Events
1. `OnTap` | `on_tap! { || {} }` Fires when a tap with a pointer button has occurred.
2. `OnTapDown` | `on_tap_down! { || {} }` Fires when a pointer that might cause a tap with a button has contacted the screen at a particular location.
3. `OnTapUp` | `on_tap_up! { || {} }` Fires when a pointer that will trigger a tap with a button has stopped contacting the screen at a particular location.
4. `OnMove` | `on_move! { || {} }` Fires when a pointer is moving over the widget.
5. `OnContact` | `on_contact! { || {} }` Fires when a pointer crosses into the bounds of the target entity.
6. `OnLeave` | `on_leave! { || {} }` Fires when a pointer crosses out of the bounds of the target entity.

---
### 🛩 UI Macros
1. `str!["Play"]` is equivalent to `"Play".to_string()`
2. `val![100. px]` is equivalent to `Val::Px(100.)`
3. `val![100. %]` is equivalent to `Val::Percent(100.)`
4. `color![TEAL]` is equivalent to `Some(Color::TEAL)`
5. `color![^TEAL]` is equivalent to `Color::TEAL`
6. `color![Color::default()]` is equivalent to `Some(Color::default())`
7. `color![^Color::default()]` is equivalent to `Color::default()`
8. `color![r: 0.75, g: 0.5., b: 0.25]` is equivalent to `Some(Color::rgb(0.75, 0.5, 0.25))`
9. `color![^r: 0.75, g: 0.5, b: 0.25]` is equivalent to `Color::rgb(0.75, 0.5, 0.25)`
10. `color![r: 0.75, g: 0.5, b: 0.25, a: 1.]` is equivalent to `Some(Color::rgba(0.75, 0.5, 0.25, 1.))`
11. `color![^r: 0.75, g: 0.5, b: 0.25, a: 1.]` is equivalent to `Color::rgba(0.75, 0.5, 0.25, 1.)`
12. `color![r8: 117, g8: 63, b8: 223]` is equivalent to `Some(Color::rgb_u8(117, 63, 223))`
13. `color![^r8: 117, g8: 63, b8: 223]` is equivalent to `Color::rgb_u8(117, 63, 223)`
14. `color![r8: 117, g8: 63, b8: 223, a8: 255]` is equivalent to `Some(Color::rgba_u8(117, 63, 223, 255))`
15. `color![^r8: 117, g8: 63, b8: 223, a8: 255]` is equivalent to `Color::rgba_u8(117, 63, 223, 255)`
16. `color!["#00FF00FF"]` is equivalent to `Some(Color::hex("#00FF00FF").unwrap_or(Color::rgba(0., 0., 0., 0.)))`
17. `color![^"#00FF00FF"]` is equivalent to `Color::hex("#00FF00FF").unwrap_or(Color::rgba(0., 0., 0., 0.))`
18. `color![hex my_string]` is equivalent to `Some(Color::hex(my_string).unwrap_or(Color::rgba(0., 0., 0., 0.)))`
19. `color![^hex my_string]` is equivalent to `Color::hex(my_string).unwrap_or(Color::rgba(0., 0., 0., 0.))`
20. `edge_insets_only![left: 10., right: 15., top: 20., bottom: 25.]` is equivalent to `EdgeInsets::from_ltrb(10., 15., 20., 25.)` You can keep only what side(s) you need, everything else will be set to 0.
21. `edge_insets_symmetric![vertical: 10.]` is equivalent to `EdgeInsets::symmetric_vertical(10.)`
22. `edge_insets_symmetric![horizontal: 10.]` is equivalent to `EdgeInsets::symmetric_horizontal(10.)`
23. `edge_insets_symmetric![vertical: 10., horizontal: 15.]` is equivalent to `EdgeInsets::symmetric_vh(10., 15.)`
24. `edge_insets_symmetric![horizontal: 15., vertical: 10.]` is equivalent to `EdgeInsets::symmetric_vh(10., 15.)`

---
### 🛩 Known limitations / Work in progress
1. Currently, you cannot create your own widgets. Consider adding `compose_widget!` proc macro.
2. Expressions such as `for`/`while`/`loop`/`if-else`/`match` inside `widget_tree!` are missing.
3. Missing a lot of useful widgets such as `ProgressIndicator`, `CheckBox`, `RadioButton`, `ToggleButton`, `Dropdown`, `TextEdit`, `SelectableText`, `ScrollArea`, `ListView`, `AppBar`, `Icon`, `Stack`, `Grid`, `Wrap`, `Drawer`, `ColorPicker`, `FilePicker` and so on..
4. Animation capability is still in design phase. `Alignment` must be fixed before.
5. `Alignment` values have issues. Consider adding a system to process them.
6. All 4 borders inside `BoxDecoration` must have the same color in order to render the border.

---
## [ ✈️ -1- ] <u>Padding Example</u>

<p>
  <img src="./images/padding_example.png" width="400" title="hover text">
</p>

```rust
widget_tree!(
    Scaffold {
        child: Container {
            color: color![BLUE]
            width: val![500. px]
            height: val![500. px]
            child: Padding {
                padding: EdgeInsets::all(val![80. px])
                child: Container {
                    color: color![YELLOW]
                    width: val![100. %]
                    height: val![100. %]
                }
            }
        }
    }
);
```

---
## [ ✈️ -2- ] <u>Column Example</u>

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
                        color: color![RED]
                        width: val![300. px]
                    }
                    Container {
                        color: color![GREEN]
                        width: val![480. px]
                    }
                    Container {
                        color: color![BLUE]
                        width: val![200. px]
                    }
                ]
            }
        }
    }
);
```

---
## [ ✈️ -3- ] <u>Row Example</u>

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
                        color: color![RED]
                        height: val![300. px]
                    }
                    Container {
                        color: color![GREEN]
                        height: val![480. px]
                    }
                    Container {
                        color: color![BLUE]
                        height: val![200. px]
                    }
                ]
            }
        }
    }
);
```

---
## [ ✈️ -4- ] <u>Queries Example</u>

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
                            Container { color: color![RED] }
                            SizedBox { height: val![10. px] }
                            Visible {
                                tags: [Collapsible]
                                child: Column {
                                    children: [
                                        Container {
                                            tags: [ColorTag]
                                            color: color![ORANGE]
                                        }
                                        SizedBox { height: val![10. px] }
                                    ]
                                }
                            }
                            Container { color: color![DARK_GREEN] }
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

---
## [ ✈️ -5- ] <u>Gesture Detector Example</u>

Every time when you click on the first `Container`, all Containers in the Widget Tree will change their color.
Mouse enter will trigger `on_contact` and mouse exit will trigger `on_leave`.

```rust
widget_tree! {
    Center {
        child: GestureDetector {
            on_tap: on_tap! {
                |mut query: ColorQuery<Container>| {
                    query.set_random_color();
                }
            }
            on_contact: on_contact! { || println!("Contact") }
            on_leave: on_leave! { || println!("Leave") }
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

---
### 🛫 Some query tips for applying a theme:

⚠️ When working with WingsUI, Do not access BevyUI components directly because of unexpected behaviour !
Instead, use built-in Queries ⚠️

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