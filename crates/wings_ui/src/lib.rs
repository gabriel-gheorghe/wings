pub mod classes;
pub mod common_tags;
pub mod enums;
pub mod events;
pub mod macros;
pub mod plugin;
pub mod queries;
pub(crate) mod systems;
pub mod widgets;

pub mod platform {
    pub use crate::classes::platform_classes::UiViewPadding;
}

pub mod prelude {
    pub use crate::{edge_insets_only, edge_insets_symmetric, define_ui_types, val, str};

    pub use crate::classes::alignment::UiAlignment;
    pub use crate::classes::edge_insets::UiEdgeInsets;

    pub use crate::common_tags::Collapsible;

    pub use crate::enums::{UiCrossAxisAlignment, UiMainAxisAlignment, UiMainAxisSize};

    pub use crate::events::{
        UiPointerClick, UiPointerDoubleClick, UiPointerEnter, UiPointerExit,
        UiPointerMove, UiPointerOver, UiPointerPress, UiPointerRelease,
    };

    pub use crate::plugin::WingsUiPlugin;

    pub use crate::queries::color::UiColorQuery;
    pub use crate::queries::size::UiSizeQuery;
    pub use crate::queries::visibility::{UiLayoutVisibilityQuery, UiVisibilityQuery};

    pub use crate::widgets::UiWidgetBundle;

    pub use crate::widgets::alignment::{UiAlign, UiAlignBundle, UiAlignProps, UiCenterBundle};
    pub use crate::widgets::button::{UiButton, UiButtonBundle, UiButtonProps};
    pub use crate::widgets::constrained::{
        UiConstrainedHeight, UiConstrainedHeightBundle,
        UiConstrainedWidth, UiConstrainedWidthBundle,
    };
    pub use crate::widgets::container::{UiContainer, UiContainerBundle, UiContainerProps};
    pub use crate::widgets::direction::{
        UiColumn, UiColumnBundle, UiColumnProps, UiRow, UiRowBundle, UiRowProps,
    };
    pub use crate::widgets::divider::{
        UiHorizontalDivider, UiHorizontalDividerBundle, UiHorizontalDividerProps,
        UiVerticalDivider, UiVerticalDividerBundle, UiVerticalDividerProps,
    };
    pub use crate::widgets::padding::{UiPadding, UiPaddingBundle, UiPaddingProps};
    pub use crate::widgets::scaffold::{UiScaffold, UiScaffoldBundle, UiScaffoldProps};
    pub use crate::widgets::sized_box::{UiSizedBoxBundle, UiSizedBoxProps};
    pub use crate::widgets::text::{UiText, UiTextBundle, UiTextProps};
    pub use crate::widgets::visibility::{
        UiLayoutVisibility, UiLayoutVisibilityBundle, UiVisibility, UiVisibilityBundle,
    };
}
