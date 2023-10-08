pub mod plugin;

pub mod platform {
    pub use wings_ui::platform::*;
}

pub mod prelude {
    pub use crate::plugin::WingsPlugin;

    pub use wings_ui::prelude::*;
    pub use wings_ui_derive::widget_tree;
    pub use wings_ui::{define_ui_types, edge_insets_only, edge_insets_symmetric, val};
    pub use wings_utils::prelude::*;
}