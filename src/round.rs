use crate::round::RoundError::IndexError;
use rand::rngs::mock::StepRng;
use rand::seq::SliceRandom;
use std::collections::HashMap;
use std::mem::replace;
use std::ops::{Deref, DerefMut};
use std::vec::IntoIter;
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub(crate) struct Round(Vec<Vec<i32>>);

impl IntoIterator for Round {
    type Item = Vec<i32>;
    type IntoIter = IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl Deref for Round {
    type Target = [Vec<i32>];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Round {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Round {
    pub(crate) fn from_players(players: &[i32]) -> Result<Round, RoundError> {
        match players.len() {
            0..=5 => Err(RoundError::NotEnoughPlayers()),
            6 | 7 | 11 => Err(RoundError::NeedsStaggered(players.len())),
            _ => {
                let fours = match players.len() {
                    x if x % 5 == 0 => 0,
                    x => 5 - x % 5,
                };
                let fives = (players.len() - 4 * fours) / 5;
                let mut result_list = Vec::with_capacity(fours + fives);
                for round in players[..fives * 5]
                    .chunks(5)
                    .chain(players[fives * 5..].chunks(4))
                {
                    result_list.push(Vec::from(round));
                }
                Ok(Round(result_list))
            }
        }
    }

    pub(crate) fn copy(round: Round) -> Round {
        Round(round.into_iter().map(Vec::from).collect())
    }
    pub(crate) fn shuffle(self) {
        let mut players = Vec::from_iter(self.iter_players());
        let mut rng = StepRng::new(2, 13);

        players.shuffle(&mut rng);
    }
    pub(crate) fn iter_table_players(self) -> IntoIter<(usize, usize, usize, i32)> {
        let mut table_size;
        let mut table_players = Vec::new();
        for (table_number, players) in self.iter().cloned().enumerate() {
            table_size = players.len();
            for (position, player) in players.into_iter().enumerate() {
                table_players.push((table_number, position, table_size, player));
            }
        }
        table_players.into_iter()
    }
    pub(crate) fn iter_tables(self) -> IntoIter<Vec<i32>> {
        self.into_iter()
    }
    pub(crate) fn iter_players(&self) -> impl Iterator<Item=&i32> {
        self.iter().flatten()
    }
    pub(crate) fn tables_count(self) -> usize {
        self.0.len()
    }
    pub(crate) fn players_count(self) -> usize {
        self.iter_players().count()
    }
    pub(crate) fn _global_indexes(self) -> HashMap<usize, Vec<i32>> {
        let mut global_indexes: HashMap<_, Vec<_>> = HashMap::new();

        for (i, table) in self.iter().enumerate() {
            global_indexes.insert(i, table.clone());
        }

        global_indexes
    }
    pub(crate) fn __global_index_to_tuple(
        &self,
        mut index: usize,
    ) -> Result<(usize, usize), RoundError> {
        let mut table_index = 0;
        for table in self.iter() {
            if index >= table.len() {
                index -= table.len();
                table_index += 1;
                continue;
            }
            return Ok((table_index, index));
        }
        Err(IndexError())
    }
    pub(crate) fn get_table(&self, index: usize) -> Vec<i32> {
        self[index].clone()
    }
    pub(crate) fn set_table(mut self, index: usize, value: Vec<i32>) {
        let _ = replace(&mut self[index], value);
    }
    pub(crate) fn get_player(&self, index: usize) -> i32 {
        let (i, j) = self.__global_index_to_tuple(index).unwrap();

        self[i][j]
    }
    pub(crate) fn set_player(mut self, index: usize, value: i32) {
        let (i, j) = self.__global_index_to_tuple(index).unwrap();

        let _ = replace(&mut self[i][j], value);
    }
}

#[derive(Debug, Clone, Error, PartialEq)]
pub(crate) enum RoundError {
    #[error("{0} players require a staggered round")]
    NeedsStaggered(usize),
    #[error("Need at least 6 players")]
    NotEnoughPlayers(),
    #[error("Out of bounds")]
    IndexError(),
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round_from_19() {
        let manually_constructed_round = Ok(Round(vec![
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19],
        ]));
        let players: Vec<i32> = (1..=19).collect();
        assert_eq!(manually_constructed_round, Round::from_players(&players));
    }

    #[test]
    fn test_err_on_staggered() {
        let six_players: Vec<_> = (1..=6).collect();
        let seven_players: Vec<_> = (1..=7).collect();
        let eleven_players: Vec<_> = (1..=11).collect();
        assert_eq!(
            Err(RoundError::NeedsStaggered(6usize)),
            Round::from_players(&six_players)
        );
        assert_eq!(
            Err(RoundError::NeedsStaggered(7usize)),
            Round::from_players(&seven_players)
        );
        assert_eq!(
            Err(RoundError::NeedsStaggered(11usize)),
            Round::from_players(&eleven_players)
        );
    }

    #[test]
    fn test_iter_players() {
        let players: Vec<_> = (1..=9).collect();
        let expected = players.iter();
        let round = Round::from_players(&players).unwrap();
        for (i, j) in expected.zip(round.iter_players()) {
            assert_eq!(i, j);
        }
    }
}
