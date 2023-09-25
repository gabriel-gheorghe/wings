pub mod plugin;
pub mod queries;
pub mod tags;
pub mod visibility;
pub mod widgets;

pub mod prelude {
    pub use crate::queries::color::UiColorQuery;
    pub use crate::queries::visibility::UiVisibilityQuery;

    pub use crate::widgets::alignment::UiCenter;
    pub use crate::widgets::container::{UiContainerProps, UiContainer};
    pub use crate::widgets::direction::{UiRow, UiColumn};
    pub use crate::widgets::divider::{
        UiHorizontalDividerProps, UiHorizontalDivider,
        UiVerticalDividerProps, UiVerticalDivider,
    };
    pub use crate::widgets::screen::{UiScreenProps, UiScreen};

    pub use crate::plugin::WingsUiPlugin;

    pub use crate::tags::UiTagCollapsible;

    pub use crate::visibility::UiVisibility;
}
