pub fn join(strings : &Vec<&str>, sep : &str) -> String {
	let mut joined_string = String::new();
	
	if strings.len() > 0 {
		joined_string.push_str( strings[0]);
	}

	for i in 1..strings.len() {
		joined_string.push_str(sep);
		joined_string.push_str(strings[i]);
	}

	/*
	let mut it = strings.iter().next();
	loop {
		match it.peek() {
			Some(e) => { 
				joined_string.push_str(e);
				joined_string.push_str(sep);
			},
			None => {
				break;
			},
		}
		it.next();
	}
	*/
// 	need to remove last sep
//	joined_string.push_str(	 );

	joined_string
}


pub fn padding ( width : u32 ) -> String {
	let mut pads = String::new();

	for _ in 0..width {
		pads.push_str(" ");
	}

	pads
}

pub fn is_numeric(string : &str ) -> bool {
	if string.is_empty() {
		return false;
	}

	for each_char in string.chars() {
		if !each_char.is_numeric() {
			return false;
		}
	}

	true
}

pub fn is_blank(string : &str ) -> bool {
	if string.len() == 0 {
		return true;
	}

	// let v: Vec<char> = string.chars().collect();

	// for i in v {
	// 	println!("{:?}", i);
	// 	if !i.is_whitespace() {
	// 		return false;
	// 	}
	// }

	for each_char in string.chars() {
		if !each_char.is_whitespace() {
			return false;
		}
	}

	true
}

/*
// there is 'is_whitespace' function in char method.
// without /f 
pub fn is_whitespace( c : char ) -> bool {
	(c == ' ' || c == '\t' || c == '\n' || c == '\r')
}
*/

// 
pub fn nomalize_whitespace(string : &str) -> String {
	let mut nomalized_string = String::new();
	append_normalized_whitespace(&mut nomalized_string, string, false);
	nomalized_string
}

// strip_leading : set to true if you wish to remove any leading whitespace
pub fn append_normalized_whitespace(accum : &mut String , string : &str , strip_leading : bool) {
	let mut last_was_whitespace : bool = false;
	let mut reached_non_whitespace : bool = false;

	for each_char in string.chars() {
		if each_char.is_whitespace() {
			// check leading white spaces
			if strip_leading && !reached_non_whitespace {
				continue;
			}

			// check whitespace continuity
			if last_was_whitespace {
				continue;
			}

			// push one space		
			accum.push( ' ' );
			last_was_whitespace = true;
		} else {
			// push character
			accum.push( each_char );
			last_was_whitespace = false;

			// turn off leading whitespace checker
			reached_non_whitespace = true;
		}

	}

}

// find 
pub fn in_haystack(needle : &str, haystack : &Vec<&str> ) -> bool {
	for &each_string in haystack {
		if needle == each_string {
			return true;
		}
	}
	false
}

fn main() {
	let vec : Vec<&str> = vec!["abcd", "efg", "hi", "jk"];
	let vec1 : Vec<&str> = vec!["abcd"];
	let vec2 : Vec<&str> = vec!();

	// test haystack
	println!("\ntest haystack()");
	println!("test haystack {:?}", in_haystack("abcd", &vec));
	println!("test haystack {:?}", in_haystack("abcde", &vec));

	// test join
	println!("\ntest join()");
	println!("{:?}", vec);
	println!("{:?}", join(&vec, "\t") );
	println!("{:?}", join(&vec1, "\t") );
	println!("{:?}", join(&vec2, "\t") );
	
	// test padding
	println!("\ntest padding()");
	println!("{:?}", padding(4));

	// test whitespace and blank
	println!("\ntest is_whitespace()");
	println!("{:?} {:?}", (' ').is_whitespace(),('t').is_whitespace() );

	println!("\ntest is_blank()");
	println!("{:?} {:?}", is_blank("\t\t\t"), is_blank("tptptp"));

	// test normalize_whitespace
	println!("\ntest normalize_whitespace()");
	println!("{:?}", nomalize_whitespace("\t\t\tkakaka\t\tkoko\t\t")); 

}