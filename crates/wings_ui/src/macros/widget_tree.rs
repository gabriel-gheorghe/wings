#[allow(unused_macros)]

#[macro_export]
macro_rules! define_ui_types {
    () => {
        type Scaffold = UiScaffoldBundle;
        type Container = UiContainerBundle;
        type SizedBox = UiSizedBoxBundle;
        type Padding = UiPaddingBundle;
        type Align = UiAlignBundle;
        type Center = UiCenterBundle;
        type Column = UiColumnBundle;
        type Row = UiRowBundle;

        type EdgeInsets = UiEdgeInsets;
        type Alignment = UiAlignment;
        type MainAxisSize = UiMainAxisSize;
        type MainAxisAlignment = UiMainAxisAlignment;
        type CrossAxisAlignment = UiCrossAxisAlignment;
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
             $({widget_tree!(parent, $child_t $body);})*;
        });
    };
}