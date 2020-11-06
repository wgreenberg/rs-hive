use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;
use crate::game_state::{Turn, GameState};
use crate::hex::Hex;
use crate::piece::Piece;
use crate::parser::parse_piece_string;

fn read_sgf_file<P: AsRef<Path>>(path: P) -> Option<GameState> {
    let file = File::open(path).unwrap();
    let mut game = GameState::new();
    let mut origin: Option<Hex> = None;
    let mut last_turn: Option<Turn> = None;
    for maybe_line in BufReader::new(file).lines() {
        let line = maybe_line.unwrap();
        if line.starts_with("SU[") {
            if line != "SU[Hive]" {
                return None
            }
        }
        if line.starts_with("; ") {
            if line.contains("move") || line.contains("dropb") || line.contains("pass") {
                last_turn = parse_turn(&line, &game.unplayed_pieces, &mut origin);
            } else if line.contains("resign") {
                return Some(game);
            } else if line.contains("done]") {
                assert_eq!(game.submit_turn(last_turn.unwrap()), Ok(()));
            }
        }
    }
    Some(game)
}

fn parse_turn(input: &str, unplayed_pieces: &Vec<Piece>, origin: &mut Option<Hex>) -> Option<Turn> {
    if input.contains("move") || input.contains("dropb") {
        let mut tokens = input.split_whitespace();
        let _semicolon = tokens.next();
        let _turn_no = tokens.next();
        let move_type = tokens.next();
        if move_type == Some("move") {
            let _color = tokens.next();
        }
        let piece = parse_piece_string(tokens.next().unwrap()).unwrap();
        let axial_col = tokens.next().unwrap();
        let axial_row = tokens.next().unwrap().parse::<i64>().unwrap();
        let dest = axial_to_hex(axial_col, axial_row);
        // wherever the first hex is in absolute space, normalize it so everything's centered
        // around (0, 0, 0)
        if origin.is_none() {
            *origin = Some(dest);
        }
        if unplayed_pieces.contains(&piece) {
            Some(Turn::Place(piece, dest.sub(origin.unwrap())))
        } else {
            Some(Turn::Move(piece, dest.sub(origin.unwrap())))
        }
    } else if input.contains("pass") {
        Some(Turn::Pass)
    } else {
        None
    }
}

fn axial_to_hex(col: &str, row: i64) -> Hex {
    let x: i64 = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".find(col).unwrap() as i64;
    let z: i64 = -row;
    let y: i64 = -x-z;
    Hex::new(x, y, z)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sgf_games() {
        std::fs::read_dir("./test_data")
            .expect("failed to open dir")
            .flat_map(|entry| entry)
            .for_each(|entry| { read_sgf_file(entry.path()); });
    }
}