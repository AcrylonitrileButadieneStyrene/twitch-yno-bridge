#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Command {
    Keys(Vec<Key>),
    Bang(Bang),
}

impl Command {
    pub const fn is_trusted(&self) -> bool {
        match self {
            Self::Keys(_) => false,
            Self::Bang(command) => command.is_trusted(),
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "t", content = "c")]
#[non_exhaustive]
pub enum Bang {
    SwitchGame(String),
    Loop(Vec<Key>),
}

impl Bang {
    pub const fn is_trusted(&self) -> bool {
        match self {
            Self::Loop(_) => false,
            _ => true,
        }
    }
}

#[derive(Clone, Debug, serde_repr::Serialize_repr, PartialEq, Eq)]
#[repr(u8)]
pub enum Key {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Up = 10,
    Right = 11,
    Down = 12,
    Left = 13,
    Confirm = 14,
    Cancel = 15,
    Shift = 16,
}
