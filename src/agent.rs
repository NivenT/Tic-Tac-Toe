extern crate rand;

use self::rand::Rng;
use std::collections::HashMap;
use std::io;
use board::*;

pub struct Agent {
	quality:		HashMap<(Board, usize), f64>,
	gamma:			f64,
	learning_rate:	f64
}

impl Agent {
	pub fn new(gamma: f64, lr: f64) -> Agent {
		Agent{quality: HashMap::new(), gamma: gamma, learning_rate: lr}
	}
	fn reward(b: Board, m: usize) -> f64 {
		let b = b.make_move(m).unwrap();
		if b.is_over() {
			match b.get_winner() {
				Ok(Cell::X)	=> -100.,
				Ok(Cell::O) => 100.,
				_			=> 50.
			}
		} else {
			0.
		}
	}
	pub fn get_value(&mut self, b: Board, m: usize) -> f64 {
		if !self.quality.contains_key(&(b, m)) {
			self.quality.insert((b, m), 0.);
			self.update_quality(b, m);
		}
		self.quality[&(b, m)]
	}
	pub fn update_quality(&mut self, b: Board, m: usize) {
		let next = b.make_move(m).unwrap();
		let valid_next_moves = next.get_valid_moves();

		let mut next_quality = if !valid_next_moves.is_empty() {self.get_value(next, valid_next_moves[0])} else {0.};
		if b.get_turn() == Cell::O {
			for m in valid_next_moves {
				next_quality = next_quality.min(self.get_value(next, m));
			}
		} else {
			for m in valid_next_moves {
				next_quality = next_quality.max(self.get_value(next, m));
			}
		}

		let old_val = self.get_value(b,m);
		self.quality.insert((b,m), old_val + self.learning_rate*(Agent::reward(b,m) + self.gamma*next_quality - old_val));
	}
	pub fn get_move(&mut self, b: Board) -> usize {
		let valid_moves = b.get_valid_moves();
		let mut best_moves = vec![valid_moves[0]];
		for m in valid_moves {
			if self.get_value(b, m) > self.get_value(b, best_moves[0]) {
				best_moves = vec![m];
			} else if self.get_value(b, m) == self.get_value(b, best_moves[0]) && m != best_moves[0] {
				best_moves.push(m);
			}
		}
		*rand::thread_rng().choose(&best_moves).unwrap()
	}
	pub fn _print_quality(&self) {
		for (k, q) in &self.quality {
			println!("{}({}, {}) -> {}", k.0, k.0.get_turn(), k.1, q);
			io::stdin().read_line(&mut "".to_string()).ok().expect("Failed to read line");
		}
	}
}