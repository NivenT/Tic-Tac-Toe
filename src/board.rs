extern crate rand;

use self::rand::Rng;
use std::fmt;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub enum Cell{X, O, EMPTY}

impl fmt::Display for Cell {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Cell::X			=> write!(f, "X"),
			Cell::O			=> write!(f, "O"),
			Cell::EMPTY		=> write!(f, " ")
		}
	}
}

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Board {
	cells: 	[Cell; 9],
	turn:	Cell,
	winner: Cell
}

impl fmt::Display for Board {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "Turn: {}\n {} | {} | {} \n---+---+---\n {} | {} | {} \n---+---+---\n {} | {} | {} \n",
				  self.turn, self.cells[0], self.cells[1], self.cells[2], self.cells[3], self.cells[4], self.cells[5], self.cells[6], self.cells[7], self.cells[8])
	}
}

impl Board {
	pub fn new() -> Board {
		Board{cells: [Cell::EMPTY; 9], turn: if rand::thread_rng().gen() {Cell::X} else {Cell::O}, winner: Cell::EMPTY}
	}
	pub fn is_over(&self) -> bool {
		self.winner != Cell::EMPTY || self.cells.into_iter().all(|c| *c != Cell::EMPTY)
	}
	pub fn get_turn(&self) -> Cell {
		self.turn
	}
	pub fn get_winner(&self) -> Result<Cell, Cell> {
		if self.winner == Cell::EMPTY {
			Err(Cell::EMPTY)
		} else {
			Ok(self.winner)
		}
	}
	pub fn get_valid_moves(&self) -> Vec<usize> {
		(0..9).filter(|i| self.cells[*i] == Cell::EMPTY).collect()
	}
	pub fn switch_turn(&mut self) {
		self.turn = if self.turn == Cell::X {Cell::O} else {Cell::X}
	}
	pub fn make_move(&self, pos: usize) -> Result<Board, &str> {
		let mut ret: Board = *self;
		if  pos < 9 && ret.cells[pos] == Cell::EMPTY {
			ret.cells[pos] = ret.turn;
			ret.switch_turn();
			ret.check_winner();
			return Ok(ret)
		}
		return Err("invalid index")
	}
	fn check_winner(&mut self) {
		for i in 0..3 {
			if self.cells[3*i] == self.cells[3*i+1] && self.cells[3*i+1] == self.cells[3*i+2] && self.cells[3*i+2] != Cell::EMPTY {
				self.winner = self.cells[3*i]; return;
			}
			if self.cells[i] == self.cells[i+3] && self.cells[i+3] == self.cells[i+6] && self.cells[i+6] != Cell::EMPTY {
				self.winner = self.cells[i]; return;
			}
		}
		if self.cells[0] == self.cells[4] && self.cells[4] == self.cells[8] && self.cells[8] != Cell::EMPTY {
			self.winner = self.cells[4]; return;
		} else if self.cells[2] == self.cells[4] && self.cells[4] == self.cells[6] && self.cells[6] != Cell::EMPTY {
			self.winner = self.cells[4]; return;
		}
	}
	pub fn flip(&self) -> Board {
		let mut cells = [Cell::EMPTY; 9];
		for i in (0..9) {
			cells[i] = match self.cells[i] {
				Cell::X => Cell::O,
				Cell::O => Cell::X,
				_		=> Cell::EMPTY
			}
		}
		let winner = match self.winner {
			Cell::X => Cell::O,
			Cell::O => Cell::X,
			_		=> Cell::EMPTY
		};
		Board{cells: cells, turn: if self.turn == Cell::X {Cell::O} else {Cell::X}, winner: winner}
	}
}