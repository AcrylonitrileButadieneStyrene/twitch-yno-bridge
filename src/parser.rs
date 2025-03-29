use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, one_of, space0},
    combinator::map,
    multi::many1,
    sequence::preceded,
};

use crate::command::{Bang, Command, Key};

pub fn parse(input: &str) -> IResult<&str, Command> {
    alt((parse_bang, parse_keys.map(Command::Keys))).parse(input)
}

fn parse_bang(input: &str) -> IResult<&str, Command> {
    preceded(char('!'), alt((parse_loop, parse_bang_switch)))
        .map(Command::Bang)
        .parse(input)
}

fn parse_loop(input: &str) -> IResult<&str, Bang> {
    preceded(tag("loop "), parse_keys)
        .map(Bang::Loop)
        .parse(input)
}

fn parse_bang_switch(input: &str) -> IResult<&str, Bang> {
    preceded(
        tag("switch "),
        map(take_while(nom::AsChar::is_alphanum), |game: &str| {
            Bang::SwitchGame(game.to_owned())
        }),
    )
    .parse(input)
}

fn parse_keys(input: &str) -> IResult<&str, Vec<Key>> {
    many1(parse_key).parse(input)
}

fn parse_key(input: &str) -> IResult<&str, Key> {
    preceded(
        space0,
        alt((
            map(tag("shift"), |_| Key::Shift),
            map(one_of("wasdzx0123456789"), |x| match x {
                'w' => Key::Up,
                'a' => Key::Left,
                's' => Key::Down,
                'd' => Key::Right,
                'z' => Key::Confirm,
                'x' => Key::Cancel,
                '0' => Key::Zero,
                '1' => Key::One,
                '2' => Key::Two,
                '3' => Key::Three,
                '4' => Key::Four,
                '5' => Key::Five,
                '6' => Key::Six,
                '7' => Key::Seven,
                '8' => Key::Eight,
                '9' => Key::Nine,
                _ => unreachable!(),
            }),
        )),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_parse_bang_switch() {
        let (input, command) = super::parse("!switch 2kki").unwrap();
        assert_eq!(input, "");

        let super::Command::Bang(super::Bang::SwitchGame(game)) = command else {
            panic!();
        };
        assert_eq!(game, "2kki");
    }

    #[test]
    fn test_parse_keys() {
        let super::Command::Keys(keys) = super::parse("shift zwasdshiftx").unwrap().1 else {
            panic!();
        };

        use super::Key;
        assert_eq!(keys, vec![
            Key::Shift,
            Key::Confirm,
            Key::Up,
            Key::Left,
            Key::Down,
            Key::Right,
            Key::Shift,
            Key::Cancel,
        ]);
    }

    #[test]
    fn test_parse_loop() {
        use super::Key;

        let super::Command::Bang(super::Bang::Loop(keys)) = super::parse("!loop wasd").unwrap().1
        else {
            panic!();
        };

        assert_eq!(keys, vec![Key::Up, Key::Left, Key::Down, Key::Right,]);
    }
}
