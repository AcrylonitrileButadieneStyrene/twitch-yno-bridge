use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while},
    character::complete::{char, one_of, space0},
    combinator::map,
    multi::many1,
    sequence::preceded,
};

use crate::command::{BangCommand, Command, Key};

pub fn parse(input: &str) -> IResult<&str, Command> {
    alt((parse_bang, parse_keys)).parse(input)
}

fn parse_bang(input: &str) -> IResult<&str, Command> {
    map(preceded(char('!'), alt((parse_bang_switch,))), |x| {
        Command::Bang(x)
    })
    .parse(input)
}

fn parse_bang_switch(input: &str) -> IResult<&str, BangCommand> {
    preceded(
        tag("switch "),
        map(take_while(nom::AsChar::is_alpha), |game: &str| {
            BangCommand::SwitchGame(game.to_owned())
        }),
    )
    .parse(input)
}

fn parse_keys(input: &str) -> IResult<&str, Command> {
    map(many1(parse_key), Command::KeyPresses).parse(input)
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
        use nom::Parser;

        let (input, command) = super::parse.parse("!switch test").unwrap();
        assert_eq!(input, "");

        let super::Command::Bang(super::BangCommand::SwitchGame(game)) = command else {
            panic!();
        };
        assert_eq!(game, "test");
    }

    #[test]
    fn parse_keys() {
        use super::Key;
        use nom::Parser;

        let super::Command::KeyPresses(keys) = super::parse.parse("shift zwasdshiftx").unwrap().1
        else {
            panic!();
        };

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
}
