#[allow(unused_macros)]

#[macro_export]
macro_rules! define_ui_types {
    () => {
        // COMMON
        use bevy::ui::Val::Auto;
        use bevy::ui::Val::Percent;
        use bevy::ui::Val::Px;

        // WIDGETS
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
    }
}