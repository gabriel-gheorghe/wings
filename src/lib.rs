pub mod plugin;

pub mod prelude {
    pub use crate::plugin::WingsPlugin;

    pub use wings_ui::prelude::*;
    pub use wings_utils::prelude::*;
}
