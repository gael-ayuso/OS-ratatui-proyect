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
    PreviousStep,
    Play,
    Pause,
    Stop,
    ScrollUp,
    ScrollDown,
}
