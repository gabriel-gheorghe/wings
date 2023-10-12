pub mod plugin;

/// Some useful tags to build widgets and query them.
pub mod common_tags;

/// Use `wings::platform::*;` to import platform classes.
pub mod platform {
    pub use wings_ui::platform::*;
}

/// Use `wings::prelude::*;` to import common classes, widgets and queries.
pub mod prelude {
    pub use crate::common_tags::*;
    pub use crate::plugin::WingsPlugin;

    pub use wings_ui::prelude::*;
    pub use wings_ui_derive::widget_tree;
    pub use wings_utils::prelude::*;
}

/// Provides user interface classes, widgets and queries.
pub mod ui {
    pub use wings_ui::*;
}

/// Provides proc macros for building UIs easily.
pub mod derive {
    pub use wings_ui_derive::*;
}

/// Provides utilities and miscellaneous functionality.
pub mod utils {
    pub use wings_utils::*;
}
