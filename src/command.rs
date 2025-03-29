#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum Command {
    KeyPresses(Vec<Key>),
    Bang(BangCommand),
}

#[derive(Clone, Debug, serde::Serialize)]
#[serde(tag = "t", content = "c")]
pub enum BangCommand {
    SwitchGame(String),
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
