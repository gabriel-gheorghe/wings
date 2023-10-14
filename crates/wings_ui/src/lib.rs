pub mod classes;
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
    pub use crate::{
        color,
        define_common_types, define_ui_types_bundles, define_ui_types_components,
        edge_insets_only, edge_insets_symmetric,
        on_contact, on_leave, on_move, on_tap, on_tap_down, on_tap_up,
        str, val,
    };

    pub use crate::classes::alignment::Alignment;
    pub use crate::classes::decoration::{Border, BorderSide, BorderStyle, BoxDecoration};
    pub use crate::classes::edge_insets::EdgeInsets;

    pub use crate::enums::{CrossAxisAlignment, LayoutVisibility, MainAxisAlignment, MainAxisSize};

    pub use crate::events::gesture::{
        OnContact, Contact, ContactListener,
        OnLeave, Leave, LeaveListener,
        OnMove, Move, MoveListener,
        OnTap, Tap, TapListener,
        OnTapDown, TapDown, TapDownListener,
        OnTapUp, TapUp, TapUpListener,
    };

    pub use crate::plugin::WingsUiPlugin;

    pub use crate::queries::color::ColorQuery;
    pub use crate::queries::size::SizeQuery;
    pub use crate::queries::text::TextQuery;
    pub use crate::queries::visibility::{LayoutVisibilityQuery, VisibleQuery};

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
    pub use crate::widgets::gesture_detector::{
        GestureDetectorBundle, GestureDetectorProps, GestureDetectorWidget,
    };
    pub use crate::widgets::padding::{PaddingBundle, PaddingProps, PaddingWidget};
    pub use crate::widgets::paragraph::{ParagraphBundle, ParagraphProps, ParagraphWidget};
    pub use crate::widgets::scaffold::{ScaffoldBundle, ScaffoldProps, ScaffoldWidget};
    pub use crate::widgets::sized_box::{SizedBoxBundle, SizedBoxProps, SizedBoxWidget};
    pub use crate::widgets::visibility::{
        LayoutVisibilityBundle, LayoutVisibilityProps, LayoutVisibilityWidget,
        VisibleBundle, VisibleProps, VisibleWidget,
    };
}
