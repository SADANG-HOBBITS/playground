use std::io;

extern crate rand;

const INPUT_NUMBER_COUNT: usize = 3;

/// [플레이어가 맞춰야 할 숫자 생성]
/// 3개의 1 ~ 9 사이의 서로 다른 무작위 숫자 생성 후 Vector로 반환
/// return: 8bit unsigned Vector
fn generate_answer() -> Vec<u8> {
	let mut answer: Vec<u8> = Vec::new();

	// Vector 크기는 항상 INPUT_NUMBER_COUNT로 고정
	while answer.len() < INPUT_NUMBER_COUNT {
		let rand_number = rand::random::<u8>() % 9 + 1;

		// 생성한 랜덤 숫자와 같은 숫자가 이미 포함되어 있는지 answer를 순회하면서 검사 
		// reference: https://doc.rust-lang.org/stable/book/closures.html
		match answer.iter().find(|x| **x == rand_number) {
			Some(_) => {},	
			None => answer.push(rand_number),
		}
	}

	answer
}

/// [플레이어 입력 처리]
/// 키보드 입력을 받고 비교 가능한 숫자로 변환 후 Vector를 포함한 Result type으로 반환
/// return: Result type < Vector, 문자열 >
/// Result 추가 정보: https://doc.rust-lang.org/std/result/
fn get_user_input() -> Result<Vec<u8>, &'static str> {
	// 플레이어 입력을 받아 str Vector에 입력
	let mut input: String = String::new();
	io::stdin().read_line(&mut input);
	let input_numbers: Vec<&str> = input.trim().split(' ').collect();

	// 입력받은 숫자 개수를 확인
	if input_numbers.len() == INPUT_NUMBER_COUNT {	
		let mut converted_number_list:Vec<u8> = Vec::new();

		for each_number_str in input_numbers {
			// 입력 Vector를 순회하며 10진수로 변환
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

/// [게임 로직 함수]
/// 무작위로 생성되는 숫자 그룹과 플레이어가 입력한 숫자 그룹 간 비교 후 튜플로 반환
/// parameter1 answer: 무작위 생성 숫자 Vector reference
/// parameter2 input_numbers: 사용자 입력 숫자 Vector reference
/// return: 8bit unsigned int tuple
fn compare_user_input_with_answer(answer: &Vec<u8>, input_numbers: &Vec<u8>) -> (u8, u8) {
	let mut strike = 0;
	let mut ball = 0;

	// enumerate()로 생성한 index와 Vector 요소를 순회하면서 검사
	for (idx, each_number) in input_numbers.iter().enumerate() {
		// answer vector 내부를 순회하며 find closure에서 비교
		// reference: https://doc.rust-lang.org/stable/book/closures.html
		match answer.iter().find(|&x| *x == *each_number) {
			Some(_) => {
				// 숫자도 같고 index 위치도 같으면 strike
				if answer[idx] == *each_number { strike += 1; }
				else { ball += 1; }
			},
			None => {},
		}
	}

	(strike, ball)
}

/// [입력 오류 안내 출력]
/// parameter1 message: 오류 원인
fn print_input_error_message(message: &str) {
	println!("{}", message);

	println!("check your input format");
	println!("input format : number number number");
	println!("example : 1 2 3");
}

/// [main loop]
fn main() {
	// 정답 생성
	let answer = generate_answer();
	
	// 현재 시도 횟수로 최종 점수로 표시
	let mut turn = 0;
	println!(">>> Game started...");

	loop {
		println!("[turn : {}] Input your answer", turn);

		// 유저입력값 3개를 담은 Vector를 가져온다
		let user_input = match get_user_input() {
			Ok(input) => input,
			Err(message) => {
				print_input_error_message(message);
				continue;
			}
		};

		// 유효한 입력이 있으면 시도횟수 1회 증가
		turn += 1;

		// 무작위 숫자와 플레이어 입력을 비교해 결과 값 확인
		let (strike, ball) = compare_user_input_with_answer(&answer, &user_input);

		println!("[turn : {}] strike : {}, ball : {}\n", turn, strike, ball);

		// 게임 종료
		if strike == INPUT_NUMBER_COUNT as u8 { 
			println!("clear!\nyour score : {} turn", turn);
			return; 
		}
	}
}
