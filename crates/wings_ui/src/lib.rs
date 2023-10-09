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
    pub use crate::classes::platform_classes::ViewPadding;
}

pub mod prelude {
    pub use crate::{edge_insets_only, edge_insets_symmetric, define_ui_types, val, str};

    pub use crate::classes::alignment::Alignment;
    pub use crate::classes::decoration::{BoxDecoration, Border, BorderSide, BorderStyle};
    pub use crate::classes::edge_insets::EdgeInsets;

    pub use crate::common_tags::Collapsible;

    pub use crate::enums::{CrossAxisAlignment, MainAxisAlignment, MainAxisSize};

    pub use crate::events::{
        PointerClick, PointerDoubleClick, PointerEnter, PointerExit,
        PointerMove, PointerOver, PointerPress, PointerRelease,
    };

    pub use crate::plugin::WingsUiPlugin;

    pub use crate::queries::color::ColorQuery;
    pub use crate::queries::size::SizeQuery;
    pub use crate::queries::text::TextQuery;
    pub use crate::queries::visibility::{LayoutVisibilityQuery, VisibilityQuery};

    pub use crate::widgets::WidgetBundle;

    pub use crate::widgets::alignment::{
        AlignBundle, AlignProps, AlignWidget, CenterBundle, CenterWidget,
    };
    pub use crate::widgets::button::{FlatButtonBundle, FlatButtonProps, FlatButtonWidget};
    pub use crate::widgets::constrained::{
        ConstrainedHeightBundle, ConstrainedHeightWidget,
        ConstrainedWidthBundle, ConstrainedWidthWidget,
    };
    pub use crate::widgets::container::{ContainerBundle, ContainerProps, ContainerWidget};
    pub use crate::widgets::direction::{
        ColumnBundle, ColumnProps, ColumnWidget, RowBundle, RowProps, RowWidget,
    };
    pub use crate::widgets::divider::{
        HorizontalDividerBundle, HorizontalDividerProps, HorizontalDividerWidget,
        VerticalDividerBundle, VerticalDividerProps, VerticalDividerWidget,
    };
    pub use crate::widgets::padding::{PaddingBundle, PaddingProps, PaddingWidget};
    pub use crate::widgets::paragraph::{ParagraphBundle, ParagraphProps, ParagraphWidget};
    pub use crate::widgets::scaffold::{ScaffoldBundle, ScaffoldProps, ScaffoldWidget};
    pub use crate::widgets::sized_box::{SizedBoxBundle, SizedBoxProps, SizedBoxWidget};
    pub use crate::widgets::visibility::{
        LayoutVisibilityBundle, LayoutVisibilityWidget, VisibleBundle, VisibleWidget,
    };
}
