use std::str;
use Action::*;
use Outcome::*;

const _EXAMPLE: &str = "\
A Y\n\
B X\n\
C Z\n\
";

const USE_EXAMPLE: bool = false;

fn contents() -> String {
    if USE_EXAMPLE {
        _EXAMPLE.to_owned()
    } else {
        std::fs::read_to_string("inputs/2").expect("Input not found")
    }
}

type Score = i32;

enum Action {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

fn play(own: &Action, opponent: &Action) -> Outcome {
    match (own, opponent) {
        (Rock, Paper) => Lose,
        (Paper, Rock) => Win,
        (Rock, Scissors) => Lose,
        (Scissors, Rock) => Win,
        (Scissors, Paper) => Lose,
        (Paper, Scissors) => Win,
        _ => Draw,
    }
}

fn base_score(action: &Action) -> Score {
    match action {
        Rock => 1,
        Paper => 2,
        Scissors => 3,
    }
}

fn outcome_score(outcome: &Outcome) -> Score {
    match outcome {
        Win => 6,
        Draw => 3,
        Lose => 0,
    }
}

fn parse_action<'a>(action: &str, encoding: fn(&Action) -> &'a str) -> Result<Action, String> {
    if action == encoding(&Rock) {Ok(Rock)}
    else if action == encoding(&Paper) {Ok(Paper)}
    else if action == encoding(&Scissors) {Ok(Scissors)}
    else {Err(format!("Invalid action {}", action))}
}

fn parse_opponent_action(action: &str) -> Result<Action, String> {
    parse_action(action, |a| match a {
        Rock => "A",
        Paper => "B",
        Scissors => "C",
    })
}

fn parse_own_action(action: &str) -> Result<Action, String> {
    parse_action(action, |a| match a {
        Rock => "X",
        Paper => "Y",
        Scissors => "Z",
    })
}

fn parse_outcome(outcome: &str) -> Result<Outcome, String> {
    match outcome {
        "X" => Ok(Lose),
        "Y" => Ok(Draw),
        "Z" => Ok(Win),
        _ => Err(format!("Invalid outcome {}", outcome))
    }
}

fn line_score1(opponent_str: &str, own_str: &str) -> Result<Score, String> {
    let opponent = parse_opponent_action(opponent_str)?;
    let own = parse_own_action(own_str)?;
    Ok(base_score(&own) + outcome_score(&play(&own, &opponent)))
}

fn line_score2(opponent_str: &str, own_str: &str) -> Result<Score, String> {
    let opponent = parse_opponent_action(opponent_str)?;
    let outcome = parse_outcome(own_str)?;
    let own = match (opponent, &outcome) {
        (action, Draw) => action,
        (Rock, Win) => Paper,
        (Rock, Lose) => Scissors,
        (Paper, Win) => Scissors,
        (Paper, Lose) => Rock,
        (Scissors, Win) => Rock,
        (Scissors, Lose) => Paper,
    };
    Ok(base_score(&own) + outcome_score(&outcome))
}

pub fn run() {
    fn get_score(line_actions: fn(&str, &str) -> Result<Score, String>) -> Result<Score, String> {
        contents().split('\n').collect::<Vec<&str>>()
            .iter()
            .filter(|line| line != &&"")
            .map(|line| {
                if let &[a, b] = &(line.split(" ").collect::<Vec<&str>>())[..] {
                    line_actions(a, b)
                } else {
                    Err(format!("Bad input {}", line))
                }
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|scores| scores.iter().sum::<Score>())
    }

    println!("{:?}", get_score(line_score1).unwrap());

    println!("{:?}", get_score(line_score2).unwrap());
}
