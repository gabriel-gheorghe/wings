#[macro_export]
macro_rules! on_tap {
    ($callback:expr) => {{
        define_ui_types_components!();
        OnTap::run($callback)
    }};
}