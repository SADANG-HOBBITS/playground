use std::io;

extern crate rand;

const INPUT_NUMBER_COUNT: usize = 3;

/// [generates the random numbers for quiz answer]
/// random intigers generated in range 1 ~ 9.
/// return: 8bit unsigned Vector
fn generate_answer() -> Vec<u8> {
	let mut answer: Vec<u8> = Vec::new();

	// Vector size must be fixed to 3(INPUT_NUMBER_COUNT)
	while answer.len() < INPUT_NUMBER_COUNT {
		let rand_number = rand::random::<u8>() % 9 + 1;

		// Only sole number push in Vector
		// reference: https://doc.rust-lang.org/stable/book/closures.html
		match answer.iter().find(|x| **x == rand_number) {
			Some(_) => {},	
			None => answer.push(rand_number),
		}
	}

	answer
}

/// [handling player input]
/// takes keyboard input from player for making comparable number Vector
/// return: Result type < Vector, str >
/// reference: https://doc.rust-lang.org/std/result/
fn get_user_input() -> Result<Vec<u8>, &'static str> {
	// takes player's input and converts to str Vector
	let mut input: String = String::new();
	io::stdin().read_line(&mut input);
	let input_numbers: Vec<&str> = input.trim().split(' ').collect();

	// checks the count of input number
	if input_numbers.len() == INPUT_NUMBER_COUNT {	
		let mut converted_number_list:Vec<u8> = Vec::new();

		for each_number_str in input_numbers {
			// converts values of input Vector(str) to decimal number
			match u8::from_str_radix(each_number_str, 10) {
				Ok(num) => { converted_number_list.push(num); },
				Err(_) => { return Err("invalid number format"); },
			};
		}

		Ok(converted_number_list)
	} else {
		Err("invalid input number")
	}
}

/// [game logic]
/// returns the result of comparing answer and player's input number
/// parameter1 answer: randomly generated numbers Vector reference
/// parameter2 input_numbers: player's input numbers Vector reference
/// return: 8bit unsigned int tuple
fn compare_user_input_with_answer(answer: &Vec<u8>, input_numbers: &Vec<u8>) -> (u8, u8) {
	let mut strike = 0;
	let mut ball = 0;

	// validates each number with its value and index
	for (idx, each_number) in input_numbers.iter().enumerate() {
		
		// checks the number is in answer or not using closure
		// reference: https://doc.rust-lang.org/stable/book/closures.html
		match answer.iter().find(|&x| *x == *each_number) {
			Some(_) => {
				// Strike is not only same value but also same index.
				if answer[idx] == *each_number { strike += 1; }
				else { ball += 1; }
			},
			None => {},
		}
	}

	(strike, ball)
}

/// [prints messages about input errors]
/// parameter1 message: causes of errors
fn print_input_error_message(message: &str) {
	println!("{}", message);

	println!("check your input format");
	println!("input format : number number number");
	println!("example : 1 2 3");
}

/// [main loop]
fn main() {
	// generates answer
	let answer = generate_answer();
	
	// keeps the number of attempt to figure out final score
	let mut turn = 0;
	println!(">>> Game started...");

	loop {
		println!("[turn : {}] Input your answer", turn);

		// validates and convert player's input
		let user_input = match get_user_input() {
			Ok(input) => input,
			Err(message) => {
				print_input_error_message(message);
				continue;
			}
		};

		// increases the number of attempt, only if user's input is valid
		turn += 1;

		// figures the each turn result out
		let (strike, ball) = compare_user_input_with_answer(&answer, &user_input);

		println!("[turn : {}] strike : {}, ball : {}\n", turn, strike, ball);

		// finish the game
		if strike == INPUT_NUMBER_COUNT as u8 { 
			println!("clear!\nyour score : {} turn", turn);
			return; 
		}
	}
}
