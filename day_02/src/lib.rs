use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    /// Determines the move to play, against `self`, given the [`instruction`].
    ///
    /// [`instruction`]: Instruction
    pub fn oppose_with(&self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Win => match self {
                Self::Rock => Self::Paper,
                Self::Paper => Self::Scissors,
                Self::Scissors => Self::Rock,
            },
            Instruction::Draw => *self,
            Instruction::Lose => match self {
                Self::Rock => Self::Scissors,
                Self::Paper => Self::Rock,
                Self::Scissors => Self::Paper,
            },
        }
    }

    pub fn points(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "A" | "X" => Move::Rock,
            "B" | "Y" => Move::Paper,
            "C" | "Z" => Move::Scissors,
            unknown => {
                return Err(Error::ParseMove {
                    input: unknown.to_owned(),
                })
            }
        })
    }
}

#[derive(Debug)]
pub struct Game {
    you: Move,
    opponent: Move,
}

impl Game {
    pub fn new(you: Move, opponent: Move) -> Self {
        Self { you, opponent }
    }

    /// Determines the winner of the game.
    ///
    /// Rock beats scissors. Scissors beats paper. Paper beats rock. ALl other variations result in
    /// a draw.
    // For completeness sake, and since the compiler helps us enforce all cases this way, draws
    // have been included explicitly in the match.
    pub fn winner(&self) -> Winner {
        match (self.you, self.opponent) {
            (Move::Rock, Move::Rock) => Winner::Draw,
            (Move::Rock, Move::Scissors) => Winner::You,
            (Move::Rock, Move::Paper) => Winner::Opponent,
            (Move::Paper, Move::Rock) => Winner::You,
            (Move::Paper, Move::Paper) => Winner::Draw,
            (Move::Paper, Move::Scissors) => Winner::Opponent,
            (Move::Scissors, Move::Rock) => Winner::Opponent,
            (Move::Scissors, Move::Paper) => Winner::You,
            (Move::Scissors, Move::Scissors) => Winner::Draw,
        }
    }

    /// Compute my score of the round.
    /// 1 for Rock, 2 for Paper, and 3 for Scissors.
    /// 0 if you lost, 3 if the round was a draw, and 6 if you won
    pub fn my_score(&self) -> u32 {
        let shape = self.you.points();
        let winner = self.winner().points();

        shape + winner
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Winner {
    You,
    Opponent,
    Draw, // i.e. no winner, but you do get a score :)
}

impl Winner {
    pub fn points(&self) -> u32 {
        match self {
            Self::You => 6,
            Self::Draw => 3,
            Self::Opponent => 0,
        }
    }
}

pub struct Tournament<'games> {
    games: &'games [Game],
}

impl<'games> Tournament<'games> {
    pub fn new(games: &'games [Game]) -> Self {
        Self { games }
    }

    /// Returns scores for `(me, opponent)`.
    pub fn my_score(&self) -> u32 {
        self.games.iter().map(|g| g.my_score()).sum()
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Instruction {
    Lose, // X
    Draw, // Y
    Win,  // Z
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(match input {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            unknown => {
                return Err(Error::ParseInstruction {
                    input: unknown.to_owned(),
                })
            }
        })
    }
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum Error {
    #[error("Could not parse '{line}': invalid game specification")]
    ParseGame { line: String },

    #[error("Could not parse '{input}': move not recognized")]
    ParseMove { input: String },

    #[error("Could not parse '{input}': instruction not recognized")]
    ParseInstruction { input: String },
}
