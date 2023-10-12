#[allow(unused_macros)]

#[macro_export]
macro_rules! define_common_types {
    () => {
        use bevy::ui::Val::Auto;
        use bevy::ui::Val::Percent;
        use bevy::ui::Val::Px;
    };
}

#[macro_export]
macro_rules! define_ui_types_bundles {
    () => {
        define_common_types!();

        type Scaffold = ScaffoldBundle;
        type Container = ContainerBundle;
        type SizedBox = SizedBoxBundle;
        type Align = AlignBundle;
        type Center = CenterBundle;
        type FlatButton = FlatButtonBundle;
        type Column = ColumnBundle;
        type Row = RowBundle;
        type ConstrainedWidth = ConstrainedWidthBundle;
        type ConstrainedHeight = ConstrainedHeightBundle;
        type HorizontalDivider = HorizontalDividerBundle;
        type VerticalDivider = VerticalDividerBundle;
        type Visible = VisibleBundle;
        type LayoutVisibility = LayoutVisibilityBundle;
        type Padding = PaddingBundle;
        type Paragraph = ParagraphBundle;
        type GestureDetector = GestureDetectorBundle;
    }
}

#[macro_export]
macro_rules! define_ui_types_components {
    () => {
        define_common_types!();

        type Scaffold = ScaffoldWidget;
        type Container = ContainerWidget;
        type SizedBox = SizedBoxWidget;
        type Align = AlignWidget;
        type Center = CenterWidget;
        type FlatButton = FlatButtonWidget;
        type Column = ColumnWidget;
        type Row = RowWidget;
        type ConstrainedWidth = ConstrainedWidthWidget;
        type ConstrainedHeight = ConstrainedHeightWidget;
        type HorizontalDivider = HorizontalDividerWidget;
        type VerticalDivider = VerticalDividerWidget;
        type Visible = VisibleWidget;
        type LayoutVisibility = LayoutVisibilityWidget;
        type Padding = PaddingWidget;
        type Paragraph = ParagraphWidget;
        type GestureDetector = GestureDetectorWidget;
    }
}