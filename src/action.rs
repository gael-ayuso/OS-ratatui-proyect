#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Quit,
    Tick,
    SelectNext,
    SelectPrevious,
    SelectNextColumn,
    SelectPreviousColumn,
    Noop,
    NextStep,
    ScrollUp,
    ScrollDown,
    Reset,
}
