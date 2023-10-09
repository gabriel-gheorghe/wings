#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum LayoutVisibility {
    #[default]
    Inherited,
    Visible,
    Hidden,
    Collapsed,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MainAxisSize {
    #[default]
    Min,
    Max,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum MainAxisAlignment {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum CrossAxisAlignment {
    #[default]
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}