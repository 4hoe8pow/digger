use std::fmt;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum AppMode {
    //通常
    Normal,
    //照会
    Inquery,
    //訂正
    Amend,
    //起票
    Raise,
}

impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mode_str = match self {
            AppMode::Normal => "Normal",
            AppMode::Inquery => "Inquery",
            AppMode::Amend => "Amend",
            AppMode::Raise => "Raise",
        };
        write!(f, "{}", mode_str)
    }
}
