#[allow(unused_macros)]

#[macro_export]
macro_rules! edge_insets {
    (only(left: $l:expr)) => { UiEdgeInsets::only_left($l) };
    (only(right: $r:expr)) => { UiEdgeInsets::only_right($r) };
    (only(top: $t:expr)) => { UiEdgeInsets::only_top($t) };
    (only(bottom: $b:expr)) => { UiEdgeInsets::only_bottom($b) };

    (only(left: $l:expr, right: $r:expr)) => {
        UiEdgeInsets::from_ltrb($l, Val::Px(0.), $r, Val::Px(0.))
    };
    (only(left: $l:expr, top: $t:expr)) => {
        UiEdgeInsets::from_ltrb($l, $t, Val::Px(0.), Val::Px(0.))
    };
    (only(left: $l:expr, bottom: $b:expr)) => {
        UiEdgeInsets::from_ltrb($l, Val::Px(0.), Val::Px(0.), $b)
    };

    (only(right: $r:expr, left: $l:expr)) => {
        UiEdgeInsets::from_ltrb($l, Val::Px(0.), $r, Val::Px(0.))
    };
    (only(right: $r:expr, top: $t:expr)) => {
        UiEdgeInsets::from_ltrb(Val::Px(0.), $t, $r, Val::Px(0.))
    };
    (only(right: $r:expr, bottom: $b:expr)) => {
        UiEdgeInsets::from_ltrb(Val::Px(0.), Val::Px(0.), $r, $b)
    };

    (symmetric(vertical: $v:expr)) => { UiEdgeInsets::symmetric_vertical($v) };
    (symmetric(horizontal: $h:expr)) => { UiEdgeInsets::symmetric_horizontal($h) };
    (symmetric(vertical: $v:expr, horizontal: $h:expr $(,)?)) => {
        UiEdgeInsets::symmetric_vh($v, $h)
    };
    (symmetric(horizontal: $h:expr, vertical: $v:expr $(,)?)) => {
        UiEdgeInsets::symmetric_vh($v, $h)
    };
}
