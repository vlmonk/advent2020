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

fn score(input: &Deck) -> usize {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(idx, value)| (idx + 1) * value)
        .sum()
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

impl Game {
    pub fn play(&mut self) -> usize {
        loop {
            if self.player_a.is_empty() {
                return score(&self.player_b);
            }

            if self.player_b.is_empty() {
                return score(&self.player_a);
            }

            self.turn();
        }
    }

    fn turn(&mut self) {
        let a = self.player_a.pop_front().unwrap();
        let b = self.player_b.pop_front().unwrap();

        if a > b {
            self.player_a.push_back(a);
            self.player_a.push_back(b);
        } else if a < b {
            self.player_b.push_back(b);
            self.player_b.push_back(a);
        } else {
            panic!("Equal cards");
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day22.txt")?;
    let mut game = Game::try_from(input.as_ref())?;

    let task_a = game.play();
    println!("Task A: {}", task_a);
    Ok(())
}
