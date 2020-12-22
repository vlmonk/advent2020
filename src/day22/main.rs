use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::fs;
use std::hash::Hash;
use std::hash::Hasher;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let cards = self
            .0
            .iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "[{}]", cards)
    }
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
        self.0.pop_front()
    }

    pub fn add(&mut self, first: usize, second: usize) {
        self.0.push_back(first);
        self.0.push_back(second);
    }

    pub fn add_front(&mut self, value: usize) {
        self.0.push_back(value)
    }

    pub fn has(&self, value: usize) -> bool {
        self.0.len() >= value
    }

    pub fn copy(&self, value: usize) -> Deck {
        assert!(self.0.len() >= value);
        let copy = self.0.iter().take(value).copied().collect::<VecDeque<_>>();

        Deck(copy)
    }

    pub fn hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.0.hash(&mut hasher);
        hasher.finish()
    }
}

enum GameResult {
    WinA,
    WinB,
}

enum TurnResult {
    WinA,
    WinB,
    Continue,
}

#[derive(Debug, Clone)]
struct Game {
    player_a: Deck,
    player_b: Deck,
    memory: HashSet<(u64, u64)>,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A: {}, B: {}", self.player_a, self.player_b)
    }
}

impl TryFrom<&str> for Game {
    type Error = Box<dyn Error>;

    fn try_from(input: &str) -> Result<Self, Self::Error> {
        let mut sections = input.split("\n\n");
        let player_a = parse_deck(sections.next().ok_or("player A input not found")?)?;
        let player_b = parse_deck(sections.next().ok_or("player B input not found")?)?;

        Ok(Self {
            player_a,
            player_b,
            memory: HashSet::new(),
        })
    }
}

impl Game {
    pub fn new(player_a: Deck, player_b: Deck) -> Self {
        Self {
            player_a,
            player_b,
            memory: HashSet::new(),
        }
    }

    pub fn play(&mut self) -> usize {
        loop {
            println!("Play: {}", self);
            match self.turn() {
                TurnResult::WinA => return self.player_a.score(),
                TurnResult::WinB => return self.player_b.score(),
                _ => {}
            }
        }
    }

    pub fn play_recursive(&mut self) -> usize {
        println!("Play recursive: {}", self);

        match self.game_recursive() {
            GameResult::WinA => self.player_a.score(),
            GameResult::WinB => self.player_b.score(),
        }
    }

    fn turn(&mut self) -> TurnResult {
        let a = match self.player_a.top() {
            Some(a) => a,
            _ => return TurnResult::WinB,
        };

        let b = match self.player_b.top() {
            Some(b) => b,
            _ => {
                self.player_a.add_front(a);
                return TurnResult::WinA;
            }
        };

        if a > b {
            self.player_a.add(a, b);
        } else if a < b {
            self.player_b.add(b, a);
        } else {
            panic!("Equal cards")
        }

        TurnResult::Continue
    }

    fn game_recursive(&mut self) -> GameResult {
        println!("Game recursive: {}", self);
        loop {
            match self.turn_recursive() {
                TurnResult::WinA => return GameResult::WinA,
                TurnResult::WinB => return GameResult::WinB,
                _ => {
                    let hash = (self.player_a.hash(), self.player_b.hash());
                    self.memory.insert(hash);
                }
            }
        }
    }

    fn turn_recursive(&mut self) -> TurnResult {
        println!("Turn: {}", self);

        let a = match self.player_a.top() {
            Some(a) => a,
            _ => return TurnResult::WinB,
        };

        let b = match self.player_b.top() {
            Some(b) => b,
            _ => {
                self.player_a.add_front(a);
                return TurnResult::WinA;
            }
        };

        let hash = (self.player_a.hash(), self.player_b.hash());
        if self.memory.contains(&hash) {
            return TurnResult::WinA;
        }

        if self.player_a.has(a) && self.player_b.has(b) {
            let next_a = self.player_a.copy(a);
            let next_b = self.player_b.copy(b);

            let mut game = Game::new(next_a, next_b);

            match game.game_recursive() {
                GameResult::WinA => self.player_a.add(a, b),
                GameResult::WinB => self.player_b.add(b, a),
            }
        } else if a > b {
            self.player_a.add(a, b);
        } else if a < b {
            self.player_b.add(b, a);
        } else {
            panic!("Equal cards")
        }

        TurnResult::Continue
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("data/day22.txt")?;
    let mut game_a = Game::try_from(input.as_ref())?;
    let mut game_b = game_a.clone();

    let task_a = game_a.play();
    println!("Task A: {}", task_a);

    let task_b = game_b.play_recursive();
    println!("Task B: {}", task_b);

    Ok(())
}
