use day_02::{Error, Game, Instruction, Move, Tournament};

fn main() {
    let input = include_str!("../../../input/day_02.txt");

    let games = input
        .lines()
        .map(parse)
        .collect::<Result<Vec<_>, Error>>()
        .unwrap();

    let tournament = Tournament::new(&games);

    println!("my projected score: {}", tournament.my_score());
}

fn parse(line: &str) -> Result<Game, Error> {
    let game = line
        .split_once(' ')
        .ok_or_else(|| Error::ParseGame {
            line: line.to_owned(),
        })
        .and_then(|(lhs, rhs)| {
            let opponent: Move = lhs.parse()?;
            let instruction: Instruction = rhs.parse()?;

            let you = opponent.oppose_with(instruction);

            Ok(Game::new(you, opponent))
        })?;

    Ok(game)
}
