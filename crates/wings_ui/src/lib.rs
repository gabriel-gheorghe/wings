pub mod components;
pub mod enums;
pub mod plugin;
pub mod queries;
pub mod common_tags;
pub mod utils;
pub mod widgets;

pub mod prelude {
    pub use crate::components::{UiButton, UiColumn, UiContainer, UiRow, UiScreen, UiVisibility};

    pub use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};

    pub use crate::plugin::WingsUiPlugin;

    pub use crate::queries::color::UiColorQuery;
    pub use crate::queries::visibility::UiVisibilityQuery;

    pub use crate::common_tags::UiTagCollapsible;

    pub use crate::widgets::alignment::UiCenter;
    pub use crate::widgets::button::{UiButtonBundle, UiButtonProps};
    pub use crate::widgets::container::{UiContainerBundle, UiContainerProps};
    pub use crate::widgets::direction::{UiColumnBundle, UiColumnProps, UiRowBundle, UiRowProps};
    pub use crate::widgets::divider::{
        UiHorizontalDivider, UiHorizontalDividerProps,
        UiVerticalDivider, UiVerticalDividerProps,
    };
    pub use crate::widgets::screen::{UiScreenBundle, UiScreenProps};
}
