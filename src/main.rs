#[macro_use]
extern crate throw;

// Optimal seating computation
//
// When organizing a tournament, it's important to devise a fair seating
// which also ensures a maximum of diversity over rounds and tables.
// Original criteria:
// https://groups.google.com/g/rec.games.trading-cards.jyhad/c/4YivYLDVYQc/m/CCH-ZBU5UiUJ
mod round;

use crate::round::Round;
use phf::{phf_map, phf_set, Map, Set};
use std::collections::{HashMap, LinkedList};

// The seating rules: code, label, weight
// weights are devised so that major rules always prevail over minor rules.
// Stddev rules (R3, R8) need a factor of 100 over the next one to prevail.
static RULES: [(&str, &str, i64); 9] = [
    ("R1", "predator-prey", 10_i64.pow(9)),
    ("R2", "opponent thrice", 10_i64.pow(8)),
    ("R3", "available vps", 10_i64.pow(7)),
    ("R4", "opponent twice", 10_i64.pow(6)),
    ("R5", "fifth seat", 10_i64.pow(5)),
    ("R6", "position", 10_i64.pow(4)),
    ("R7", "same seat", 10_i64.pow(3)),
    ("R8", "starting transfers", 10_i64.pow(2)),
    ("R9", "position group", 10_i64.pow(1)),
];

static OPPONENTS: Map<i8, Set<[i8; 8]>> = phf_map! {
    4_i8 => phf_set! {
        [1, 4, 1, 1, 0, 0, 0, 0],
        [1, 4, 2, 0, 1, 0, 0, 0],
        [1, 4, 3, 0, 0, 1, 0, 0],
        [1, 4, 4, 0, 0, 0, 1, 0],
    },
    5_i8 => phf_set! {
        [1, 5, 1, 1, 0, 0, 0, 0],
        [1, 5, 2, 0, 1, 0, 0, 0],
        [1, 5, 3, 0, 0, 1, 0, 0],
        [1, 5, 4, 0, 0, 0, 1, 0],
        [1, 5, 4, 0, 0, 0, 0, 1],
    },
};

fn main() {
    println!("Hello, world!");
}

fn player_mapping(rounds: LinkedList<Round>) {
    let number = 0;
    let mut mapping = HashMap::new();
    for round in rounds.into_iter() {
        for player in round.into_iter() {
            if !mapping.contains_key(&player) {
                mapping.insert(player, number);
            }
        }
    }
}

///Measure a round (list of tables), returns two matrices:
///    position (players_count x 8):
///        for each player:
///            played, vps, transfers (integer),
///            seat1, seat2, seat3, seat4, seat5 (1 for the seat they occupy)
///    opponents (players_count x players_count x 8):
///        for each pair of players, booleans indicating if they were:
///            opponent, prey, grand-prey, grand-predator, predator,
///            cross-table, neighbour, non-neighbour
///
/// Simply adding each round measure gives the total measure.
/// This allows to re-compute a single round measure when a single round is changed.
/// Pm must be map the (Hashable) players to consecutive integers 0..players_count
///
/// Previous and hint (index of changed tables) are used to speed up measure computation
/// when searching for an optimum (only recomputes the two tables impacted by a switch)
fn measure(pm: PlayerMapping, round: Round, previous: Measure, hints: Vec<String>) -> Measure {
    Measure()
}

struct PlayerMapping(HashMap<i32, i32>);

struct Measure();

struct Score();
