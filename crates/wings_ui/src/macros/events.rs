#[allow(unused_macros)]

/// Macro used to create a system event for when a tap with a pointer button has occurred.
#[macro_export]
macro_rules! on_tap {
    ($callback:expr) => {{
        define_ui_types_components!();
        OnTap::run($callback)
    }};
}

/// Macro used to create a system event for when a pointer that might cause a tap with a button
/// has contacted the screen at a particular location.
#[macro_export]
macro_rules! on_tap_down {
    ($callback:expr) => {{
        define_ui_types_components!();
        OnTapDown::run($callback)
    }};
}

/// Macro used to create a system event for when a pointer that will trigger a tap with a button
/// has stopped contacting the screen at a particular location.
#[macro_export]
macro_rules! on_tap_up {
    ($callback:expr) => {{
        define_ui_types_components!();
        OnTapUp::run($callback)
    }};
}

/// Macro used to create a system event for when a pointer is moving over the widget.
#[macro_export]
macro_rules! on_move {
    ($callback:expr) => {{
        define_ui_types_components!();
        OnMove::run($callback)
    }};
}

/// Macro used to create a system event for when a pointer
/// crosses into the bounds of the target entity.
#[macro_export]
macro_rules! on_contact {
    ($callback:expr) => {{
        define_ui_types_components!();
        OnContact::run($callback)
    }};
}

/// Macro used to create a system event for when a pointer
/// crosses out of the bounds of the target entity.
#[macro_export]
macro_rules! on_leave {
    ($callback:expr) => {{
        define_ui_types_components!();
        OnLeave::run($callback)
    }};
}