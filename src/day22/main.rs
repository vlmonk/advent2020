use std::collections::VecDeque;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

type Deck = VecDeque<usize>;

fn parse_deck(input: &str) -> Result<Deck, Box<dyn Error>> {
    input
        .lines()
        .skip(1)
        .map(|line| {
            line.parse::<usize>()
                .map_err(|_| format!("invalid input: {}", line).into())
        })
        .collect()
}

#[derive(Debug)]
struct Game {
    player_a: Deck,
    player_b: Deck,
}

impl TryFrom<&str> for Game {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut sections = input.split("\n\n");
        let player_a = parse_deck(sections.next().ok_or("player A input not found")?)?;
        let player_b = parse_deck(sections.next().ok_or("player B input not found")?)?;

        Ok(Self { player_a, player_b })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day22.txt")?;
    let game = Game::try_from(input.as_ref())?;

    dbg!(game);

    Ok(())
}
