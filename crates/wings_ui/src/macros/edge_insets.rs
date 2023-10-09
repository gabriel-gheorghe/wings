#[allow(unused_macros)]

// TODO. ADD 3 and 4 combinations
// TODO. ADD unittest for each case
#[macro_export]
macro_rules! edge_insets_only {
    (left: $l:expr) => { EdgeInsets::only_left($l) };
    (right: $r:expr) => { EdgeInsets::only_right($r) };
    (top: $t:expr) => { EdgeInsets::only_top($t) };
    (bottom: $b:expr) => { EdgeInsets::only_bottom($b) };

    // LEFT - RIGHT
    (left: $l:expr, right: $r:expr $(,)?) => {
        EdgeInsets::from_ltrb($l, Val::Px(0.), $r, Val::Px(0.))
    };
    // LEFT - TOP
    (left: $l:expr, top: $t:expr $(,)?) => {
        EdgeInsets::from_ltrb($l, $t, Val::Px(0.), Val::Px(0.))
    };
    // LEFT - BOTTOM
    (left: $l:expr, bottom: $b:expr $(,)?) => {
        EdgeInsets::from_ltrb($l, Val::Px(0.), Val::Px(0.), $b)
    };

    // RIGHT - LEFT
    (right: $r:expr, left: $l:expr $(,)?) => {
        EdgeInsets::from_ltrb($l, Val::Px(0.), $r, Val::Px(0.))
    };
    // RIGHT - TOP
    (right: $r:expr, top: $t:expr $(,)?) => {
        EdgeInsets::from_ltrb(Val::Px(0.), $t, $r, Val::Px(0.))
    };
    // RIGHT - BOTTOM
    (right: $r:expr, bottom: $b:expr $(,)?) => {
        EdgeInsets::from_ltrb(Val::Px(0.), Val::Px(0.), $r, $b)
    };

    // TOP - LEFT
    (top: $l:expr, left: $r:expr $(,)?) => {
        EdgeInsets::from_ltrb($left, $top, Val::Px(0.), Val::Px(0.))
    };
    // TOP - RIGHT
    (top: $l:expr, right: $r:expr $(,)?) => {
        EdgeInsets::from_ltrb(Val::Px(0.), $top, $right, Val::Px(0.))
    };
    // TOP - BOTTOM
    (top: $l:expr, bottom: $r:expr $(,)?) => {
        EdgeInsets::from_ltrb(Val::Px(0.), $top, Val::Px(0.), $bottom)
    };

    // BOTTOM - LEFT
    (bottom: $l:expr, left: $r:expr $(,)?) => {
        EdgeInsets::from_ltrb($left, Val::Px(0.), Val::Px(0.), $bottom)
    };
    // BOTTOM - RIGHT
    (bottom: $l:expr, right: $r:expr $(,)?) => {
        EdgeInsets::from_ltrb(Val::Px(0.), Val::Px(0.), $right, $bottom)
    };
    // BOTTOM - TOP
    (bottom: $l:expr, top: $r:expr $(,)?) => {
        EdgeInsets::from_ltrb(Val::Px(0.), $top, Val::Px(0.), $bottom)
    };
}

#[macro_export]
macro_rules! edge_insets_symmetric {
    (vertical: $v:expr) => { EdgeInsets::symmetric_vertical($v) };
    (horizontal: $h:expr) => { EdgeInsets::symmetric_horizontal($h) };
    (vertical: $v:expr, horizontal: $h:expr $(,)?) => {
        EdgeInsets::symmetric_vh($v, $h)
    };
    (horizontal: $h:expr, vertical: $v:expr $(,)?) => {
        EdgeInsets::symmetric_vh($v, $h)
    };
}
