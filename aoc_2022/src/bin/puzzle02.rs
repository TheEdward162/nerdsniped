use std::io::Read;

use anyhow::Context;

use aoc_commons as aoc;
use aoc::anyhow;

#[derive(Debug)]
enum GamePlay {
	Rock,
	Paper,
	Scissors
}
impl GamePlay {
	pub fn score(&self) -> u32 {
		match self {
			Self::Rock => 1,
			Self::Paper => 2,
			Self::Scissors => 3
		}
	}

	pub fn select_by_outcome(&self, desired_outcome: &GameOutcome) -> Self {
		match (self, desired_outcome) {
			(Self::Rock, GameOutcome::Lose) => Self::Scissors,
			(Self::Rock, GameOutcome::Tie) => Self::Rock,
			(Self::Rock, GameOutcome::Win) => Self::Paper,
			(Self::Paper, GameOutcome::Lose) => Self::Rock,
			(Self::Paper, GameOutcome::Tie) => Self::Paper,
			(Self::Paper, GameOutcome::Win) => Self::Scissors,
			(Self::Scissors, GameOutcome::Lose) => Self::Paper,
			(Self::Scissors, GameOutcome::Tie) => Self::Scissors,
			(Self::Scissors, GameOutcome::Win) => Self::Rock
		}
	}
}
impl<'a> From<&'a str> for GamePlay {
	fn from(val: &'a str) -> Self {
		match val {
			"A" | "X" => Self::Rock,
			"B" | "Y" => Self::Paper,
			"C" | "Z" => Self::Scissors,
			_ => unreachable!()
		}
	}
}

#[derive(Debug)]
enum GameOutcome {
	Lose,
	Tie,
	Win
}
impl GameOutcome {
	pub fn score(&self) -> u32 {
		match self {
			Self::Lose => 0,
			Self::Tie => 3,
			Self::Win => 6
		}
	}
}
impl<'a> From<&'a str> for GameOutcome {
	fn from(val: &'a str) -> Self {
		match val {
			"X" => Self::Lose,
			"Y" => Self::Tie,
			"Z" => Self::Win,
			_ => unreachable!()
		}
	}
}

fn play(them: &GamePlay, us: &GamePlay) -> GameOutcome {
	match (them, us) {
		(GamePlay::Rock, GamePlay::Rock)
		| (GamePlay::Paper, GamePlay::Paper)
		| (GamePlay::Scissors, GamePlay::Scissors)
			=> GameOutcome::Tie,
		(GamePlay::Rock, GamePlay::Paper)
		| (GamePlay::Paper, GamePlay::Scissors)
		| (GamePlay::Scissors, GamePlay::Rock)
			=> GameOutcome::Win,
		(GamePlay::Rock, GamePlay::Scissors)
		| (GamePlay::Paper, GamePlay::Rock)
		| (GamePlay::Scissors, GamePlay::Paper)
			=> GameOutcome::Lose
	}
}

fn main() -> anyhow::Result<()> {
	let mut file = aoc::initialize()?;

	let mut input = String::new();
	file.read_to_string(&mut input).context("Failed to read input file")?;

	let mut total_score = 0;
	let mut total_score2 = 0;
	for round_str in input.split('\n').filter(|s| !s.is_empty()) {
		let (them_str, us_str) = round_str.split_once(" ").context("Failed to split round row by space")?;
		let them = GamePlay::from(them_str);
		let us = GamePlay::from(us_str);
		let outcome = play(&them, &us);
		total_score += us.score() + outcome.score();

		let outcome2 = GameOutcome::from(us_str);
		let us2 = them.select_by_outcome(&outcome2);
		total_score2 += us2.score() + outcome2.score();

		// log::debug!("round: {} - them = {:?}, us = {:?}, outcome = {:?}, score = {}", round_str, them, us2, outcome2, us2.score() + outcome2.score());
	}

	println!("Total score: {}", total_score);
	println!("Total score 2: {}", total_score2);

	Ok(())
}
