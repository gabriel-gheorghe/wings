pub mod plugin;
pub mod queries;
pub mod tags;
pub mod visibility;
pub mod widgets;

pub mod prelude {
    pub use crate::queries::color::UiColorQuery;
    pub use crate::queries::visibility::UiVisibilityQuery;

    pub use crate::widgets::alignment::UiCenter;
    pub use crate::widgets::button::{UiButton, UiButtonProps};
    pub use crate::widgets::container::{UiContainer, UiContainerProps};
    pub use crate::widgets::direction::{UiColumn, UiRow};
    pub use crate::widgets::divider::{
        UiHorizontalDivider, UiHorizontalDividerProps,
        UiVerticalDivider, UiVerticalDividerProps,
    };
    pub use crate::widgets::screen::{UiScreen, UiScreenProps};

    pub use crate::plugin::WingsUiPlugin;

    pub use crate::tags::UiTagCollapsible;

    pub use crate::visibility::UiVisibility;
}
