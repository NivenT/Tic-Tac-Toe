extern crate rand;

use self::rand::Rng;
use std::io;
use std::thread::sleep_ms;

mod agent;
mod board;

fn train_agent(a: &mut agent::Agent, num_episodes: usize) {
	for _i in (0..num_episodes) {
		let mut b = board::Board::new();
		while !b.is_over() {
			let mut m: usize;
			if b.get_turn() == board::Cell::O {
				m = *rand::thread_rng().choose(&b.get_valid_moves()).unwrap();
			} else {
				m = a.get_move(b.flip());
			}
			a.update_quality(b, m);
			b = b.make_move(m).unwrap();
		}
	}
}

fn computer_turn(a: &mut agent::Agent, b: board::Board) -> usize {
	sleep_ms(1000);
	a.get_move(b)
}

fn player_turn() -> usize {
	println!("Where would you like to move?");
	loop {
		let mut pos_string = String::new();
		io::stdin().read_line(&mut pos_string).ok().expect("Failed to read line");
		match pos_string.trim().parse() {
			Ok(num) 	=> return num,
			Err(why)	=> println!("Could not parse usize from string because of error: {}", why)
		}
		println!("Please reenter number:");
	}
}

fn main() {
    let mut ttt_agent = agent::Agent::new(0.8, 0.6);
    train_agent(&mut ttt_agent, 1000);

    loop {
		let mut ttt_board = board::Board::new();

		let mut player_2: usize;
		println!("Who would you like to play against?\n1. 2nd Player\n2. AI");
		loop {
			let mut player_2_string = String::new();
			io::stdin().read_line(&mut player_2_string).ok().expect("Failed to read line");
			match player_2_string.trim().parse() {
				Ok(num)		=> if num == 1 || num == 2 {player_2 = num; break} else {println!("Please enter 1 or 2:")},
				Err(why)	=> println!("Could not parse usize from string because of error: {}\nPlease reenter number:", why)
			}
		}

	    while !ttt_board.is_over() {
	    	println!("{}", ttt_board);
	    	
	    	let m = if ttt_board.get_turn() == board::Cell::X || player_2 == 1 {player_turn()} else {computer_turn(&mut ttt_agent, ttt_board)};
	    	ttt_board = match ttt_board.make_move(m) {
	    		Ok(b)	 => b,
	    		Err(why) => {println!("Could not update board because of error: {}", why); ttt_board}
	    	}
	    }

	    println!("{}", ttt_board);
	    match ttt_board.get_winner() {
	    	Ok(win) => println!("The winner is {}", win),
	    	Err(_)  => println!("It is a tie")
	    }

	    println!("Would you like to play again? (y/n)");
	    let mut response = String::new();
	    io::stdin().read_line(&mut response).ok().expect("Failed to read line");
	    if !response.starts_with("y") {
	    	break
	    }
	}
	println!("Thanks for playing");
	io::stdin().read_line(&mut "".to_string()).ok().expect("Failed to read line");
	//ttt_agent._print_quality();
}
