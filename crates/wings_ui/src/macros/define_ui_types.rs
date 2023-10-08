#[allow(unused_macros)]

#[macro_export]
macro_rules! define_ui_types {
    () => {
        // COMMON
        use bevy::ui::Val::Auto;
        use bevy::ui::Val::Percent;
        use bevy::ui::Val::Px;

        // WIDGETS
        type Scaffold = UiScaffoldBundle;
        type Container = UiContainerBundle;
        type SizedBox = UiSizedBoxBundle;
        type Align = UiAlignBundle;
        type Center = UiCenterBundle;
        type Button = UiButtonBundle;
        type Column = UiColumnBundle;
        type Row = UiRowBundle;
        type ConstrainedWidth = UiConstrainedWidthBundle;
        type ConstrainedHeight = UiConstrainedHeightBundle;
        type HorizontalDivider = UiHorizontalDividerBundle;
        type VerticalDivider = UiVerticalDividerBundle;
        type Visibility = UiVisibilityBundle;
        type LayoutVisibility = UiLayoutVisibilityBundle;
        type Padding = UiPaddingBundle;

        // CLASSES
        type ViewPadding = wings::platform::UiViewPadding;
        type EdgeInsets = UiEdgeInsets;
        type Alignment = UiAlignment;

        // ENUMS
        type MainAxisSize = UiMainAxisSize;
        type MainAxisAlignment = UiMainAxisAlignment;
        type CrossAxisAlignment = UiCrossAxisAlignment;

        // EVENTS
        type PointerClick = UiPointerClick;
    }
}