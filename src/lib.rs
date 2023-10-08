pub mod plugin;

pub mod platform {
    pub use wings_ui::platform::*;
}

pub mod prelude {
    pub use crate::plugin::WingsPlugin;

    pub use wings_ui::prelude::*;
    pub use wings_ui_derive::ui_builder;
    pub use wings_ui::{widget_tree, define_ui_types, edge_insets};
    pub use wings_utils::prelude::*;
}