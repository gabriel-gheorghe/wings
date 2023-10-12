/// The type of the layout visibility,
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum LayoutVisibility {
    /// The widget inherits the parent visibility.
    #[default]
    Inherited,
    /// The widget is visible in the layout.
    Visible,
    /// The widget is hidden but it still occupies space in the layout.
    Hidden,
    /// The widget is hidden but it doesn't occupy space in the layout.
    /// This simply removes the Flex layout.
    Collapsed,
}

/// How much space should be occupied in the main axis.
///
/// During a flex layout, available space along the main axis is allocated to
/// children. After allocating space, there might be some remaining free space.
/// This value controls whether to maximize or minimize the amount of free
/// space, subject to the incoming layout constraints.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MainAxisSize {
    /// Minimize the amount of free space along the main axis, subject to the
    /// incoming layout constraints.
    #[default]
    Min,
    /// Maximize the amount of free space along the main axis, subject to the
    /// incoming layout constraints.
    Max,
}

/// How the children should be placed along the main axis in a flex layout.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MainAxisAlignment {
    /// Place the children as close to the start of the main axis as possible.
    ///
    /// This is the default main-axis alignment.
    #[default]
    Start,
    /// Place the children as close to the end of the main axis as possible.
    End,
    /// Place the children as close to the middle of the main axis as possible.
    Center,
    /// Place the free space evenly between the children.
    SpaceBetween,
    /// Place the free space evenly between the children as well as half of that
    /// space before and after the first and last child.
    SpaceAround,
    /// Place the free space evenly between the children as well as before and
    /// after the first and last child.
    SpaceEvenly,
}

/// How the children should be placed along the cross axis in a flex layout.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum CrossAxisAlignment {
    /// Place the children with their start edge aligned with the start side of
    /// the cross axis.
    Start,
    /// Place the children as close to the end of the cross axis as possible.
    End,
    /// Place the children so that their centers align with the middle of the
    /// cross axis.
    ///
    /// This is the default cross-axis alignment.
    #[default]
    Center,
    /// Require the children to fill the cross axis.
    ///
    /// This causes the constraints passed to the children to be tight in the
    /// cross axis.
    Stretch,
    /// Place the children along the cross axis such that their baselines match.
    ///
    /// Because baselines are always horizontal, this alignment is intended for
    /// horizontal main axes. If the main axis is vertical, then this value is
    /// treated like [`Start`](CrossAxisAlignment::Start).
    ///
    /// For horizontal main axes, if the minimum height constraint passed to the
    /// flex layout exceeds the intrinsic height of the cross axis, children will
    /// be aligned as close to the top as they can be while honoring the baseline
    /// alignment. In other words, the extra space will be below all the children.
    ///
    /// Children who report no baseline will be top-aligned.
    Baseline,
}