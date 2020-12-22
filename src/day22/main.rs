use std::collections::VecDeque;
use std::convert::TryFrom;
use std::error::Error;
use std::fs;

#[derive(Debug)]
struct Deck(VecDeque<usize>);

fn parse_deck(input: &str) -> Result<Deck, Box<dyn Error>> {
    let cards = input
        .lines()
        .skip(1)
        .map(|line| {
            line.parse::<usize>()
                .map_err(|_| format!("invalid input: {}", line))
        })
        .collect::<Result<VecDeque<_>, _>>()?;

    Ok(Deck(cards))
}

impl Deck {
    pub fn score(&self) -> usize {
        self.0
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, value)| (idx + 1) * value)
            .sum()
    }

    pub fn top(&mut self) -> Option<usize> {
        self.0.get(0).copied()
    }

    pub fn change(&mut self, first: usize, second: usize) {
        let _ = self.0.pop_front();
        self.0.push_back(first);
        self.0.push_back(second);
    }

    pub fn remove_top(&mut self) {
        let _ = self.0.pop_front();
    }
}

enum TurnResult {
    WinA,
    WinB,
    Continue,
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
            match self.turn() {
                TurnResult::WinA => return self.player_a.score(),
                TurnResult::WinB => return self.player_b.score(),
                _ => {}
            }
        }
    }

    fn turn(&mut self) -> TurnResult {
        let a = match self.player_a.top() {
            Some(a) => a,
            _ => return TurnResult::WinB,
        };

        let b = match self.player_b.top() {
            Some(b) => b,
            _ => return TurnResult::WinA,
        };

        if a > b {
            self.player_a.change(a, b);
            self.player_b.remove_top();
        } else if a < b {
            self.player_b.change(b, a);
            self.player_a.remove_top();
        } else {
            panic!("Equal cards")
        }

        TurnResult::Continue
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day22.txt")?;
    let mut game = Game::try_from(input.as_ref())?;

    let task_a = game.play();
    println!("Task A: {}", task_a);
    Ok(())
}
