use std::io::{BufRead, BufReader};
use std::fs::File;

#[macro_use]
mod applog;
mod startup;

#[derive(Debug, Copy, Clone)]
enum RpsMove {
    Rock,
    Paper,
    Scissors
}

#[derive(Debug, Copy, Clone)]
struct RpsGame {
    opponent: RpsMove,
    me: RpsMove,
    score: i32
}
impl Default for RpsGame {
    fn default () -> RpsGame {
        RpsGame{opponent: RpsMove::Rock, me: RpsMove::Rock, score:0}
    }
}

/*------------------------------------------------------------------- main - */

fn main() {
    let reader = startup::get_reader().unwrap();

    if startup::is_part1() {  
        part1(reader);
    } else {
        part2(reader);
    }  

    applog::end_timestamp(startup::get_start_time());
}

/*------------------------------------------------------------------ part1 - */

fn part1(reader: BufReader<File>) {
    calculate_total_score(reader, true);
}

/*------------------------------------------------------------------ part2 - */

fn part2(reader: BufReader<File>) {
    calculate_total_score(reader, false);
}

/*------------------------------------------------- calculate_total_score - */

fn calculate_total_score(reader: BufReader<File>, part1: bool) {

    let mut games: Vec<RpsGame> = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let strategy: Vec<char> = line.chars().collect();
        let mut game: RpsGame = RpsGame::default();
        game.opponent = match strategy[0] {
            'A' => RpsMove::Rock,
            'B' => RpsMove::Paper,
            _ => RpsMove::Scissors,
        };
        game.me = get_my_move(strategy[2], game.opponent, part1);
        game.score = compute_rps_score(&game);
        games.push(game);

        //applog!("Line: {}, score: {}", line, game.score);
    }

    let mut score: i32 = 0;
    for game in games {
        score += game.score;
    }

    applog!("Total score: {}", score);
}

/*------------------------------------------------------ compute_rps_score - */

fn compute_rps_score(game: &RpsGame) -> i32 {
    let basic_score = match game.me {
        RpsMove::Rock => 1,
        RpsMove::Paper => 2,
        _ => 3,
    };

    let result_score = match game.me {
        RpsMove::Rock => match game.opponent {
            RpsMove::Rock => 3,
            RpsMove::Paper => 0,
            _ => 6,
        },
        RpsMove::Paper => match game.opponent {
            RpsMove::Rock => 6,
            RpsMove::Paper => 3,
            _ => 0,
        },
        _ => match game.opponent {
            RpsMove::Rock => 0,
            RpsMove::Paper => 6,
            _ => 3,
        },
    };

    return basic_score + result_score;
}

/*------------------------------------------------------------ get_my_move - */

fn get_my_move(strategy: char, opponent: RpsMove, part1: bool) -> RpsMove {
    let my_move: RpsMove = 
        if part1 {
            match strategy {
                'X' => RpsMove::Rock,
                'Y' => RpsMove::Paper,
                _ => RpsMove::Scissors,
            }
        } else {
            match strategy {
                'X' => match opponent { // Need to lose
                    RpsMove::Rock => RpsMove::Scissors,
                    RpsMove::Paper => RpsMove::Rock,
                    _ => RpsMove::Paper,
                },
                'Y' => opponent, // Need to draw
                _ => match opponent { // Need to win
                    RpsMove::Rock => RpsMove::Paper,
                    RpsMove::Paper => RpsMove::Scissors,
                    _ => RpsMove::Rock,
                },
            }
        };

    return my_move;
}

/*--------------------------------------------------------- End of main.rs - */