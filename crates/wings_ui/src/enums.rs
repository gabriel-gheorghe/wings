#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum UiMainAxisSize {
    #[default]
    Min,
    Max,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum UiMainAxisAlignment {
    #[default]
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default)]
pub enum UiCrossAxisAlignment {
    #[default]
    Start,
    End,
    Center,
    Stretch,
    Baseline,
}
