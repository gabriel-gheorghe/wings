#[allow(unused_macros)]

#[macro_export]
macro_rules! define_ui_types {
    () => {
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

        // TAGS
        type Collapsible = UiCollapsible;
    }
}

#[macro_export]
macro_rules! widget_tree {
    // CMDS, IDENT,?
    (
        $commands:expr,
        $t:ident $(,)?
    ) => {
        define_ui_types!();

        $commands.spawn((
            $t::default(),
        ));
    };
    // CMDS, IDENT { |IDENT: EXPR,|* } ,?
    (
        $commands:expr,
        $t:ident {
            $($field_name:ident: $field_expr:expr,)*
        } $(,)?
    ) => {
        define_ui_types!();

        use paste::paste;
        type Props = paste!([<Ui $t Props>]);

        $commands.spawn((
            $t::from(Props {
                $($field_name: $field_expr,)*
                ..default()
            }),
        ));
    };
    // CMDS, IDENT { :IDENT } ,?
    (
        $commands:expr,
        $t:ident {
            :$child_t:ident
        } $(,)?
    ) => {
        define_ui_types!();

        $commands.spawn((
            $t::default(),
        )).with_children(|parent| {
             widget_tree!(parent, $child_t);
        });
    };
    // CMDS, IDENT { |IDENT: EXPR,|* :IDENT } ,?
    (
        $commands:expr,
        $t:ident {
            $($field_name:ident: $field_expr:expr,)*
            :$child_t:ident
        } $(,)?
    ) => {
        define_ui_types!();

        use paste::paste;
        type Props = paste!([<Ui $t Props>]);

        $commands.spawn((
            $t::from(Props {
                $($field_name: $field_expr,)*
                ..default()
            }),
        )).with_children(|parent| {
             widget_tree!(parent, $child_t);
        });
    };
    // CMDS, IDENT { :IDENT BODY } ,?
    (
        $commands:expr,
        $t:ident {
            :$child_t:ident $body:tt
        } $(,)?
    ) => {
        define_ui_types!();

        $commands.spawn((
            $t::default(),
        )).with_children(|parent| {
             widget_tree!(parent, $child_t $body);
        });
    };
    // CMDS, IDENT { |IDENT: EXPR,|* :IDENT BODY } ,?
    (
        $commands:expr,
        $t:ident {
            $($field_name:ident: $field_expr:expr,)*
            :$child_t:ident $body:tt
        } $(,)?
    ) => {
        define_ui_types!();

        use paste::paste;
        type Props = paste!([<Ui $t Props>]);

        $commands.spawn((
            $t::from(Props {
                $($field_name: $field_expr,)*
                ..default()
            }),
        )).with_children(|parent| {
             widget_tree!(parent, $child_t $body);
        });
    };
    // CMDS, IDENT { |IDENT: EXPR,|* :[ |IDENT BODY,|* ] } ,?
    (
        $commands:expr,
        $t:ident {
            $($field_name:ident: $field_expr:expr,)*
            :[ $($child_t:ident $body:tt,)* ]
        } $(,)?
    ) => {
        define_ui_types!();

        use paste::paste;
        type Props = paste!([<Ui $t Props>]);

        $commands.spawn((
            $t::from(Props {
                $($field_name: $field_expr,)*
                ..default()
            }),
        )).with_children(|parent| {
             $({
                 widget_tree!(parent, $child_t $body);
             })*;
        });
    };
    // CMDS, IDENT { |IDENT: EXPR,|* :[ for _ in 0..n { |IDENT BODY,|* } ] } ,?
    (
        $commands:expr,
        $t:ident {
            $($field_name:ident: $field_expr:expr,)*
            :[ for _ in $start:literal..$n:literal { $($child_t:ident $body:tt,)* }, ]
        } $(,)?
    ) => {
        define_ui_types!();

        use paste::paste;
        type Props = paste!([<Ui $t Props>]);

        $commands.spawn((
            $t::from(Props {
                $($field_name: $field_expr,)*
                ..default()
            }),
        )).with_children(|parent| {
            for i in $start..$n {
                $({
                    widget_tree!(parent, $child_t $body);
                })*;
            }
        });
    };
    // CMDS, IDENT { |IDENT: EXPR,|* :[ for _ in 0..=n { |IDENT BODY,|* } ] } ,?
    (
        $commands:expr,
        $t:ident {
            $($field_name:ident: $field_expr:expr,)*
            :[ for _ in $start:literal..=$n:literal { $($child_t:ident $body:tt,)* }, ]
        } $(,)?
    ) => {
        define_ui_types!();

        use paste::paste;
        type Props = paste!([<Ui $t Props>]);

        $commands.spawn((
            $t::from(Props {
                $($field_name: $field_expr,)*
                ..default()
            }),
        )).with_children(|parent| {
            for i in $start..=$n {
                $({
                    widget_tree!(parent, $child_t $body);
                })*;
            }
        });
    };
}