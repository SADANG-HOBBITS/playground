extern crate rsoup;

use rsoup::helper::string_util;

#[test]
fn node_test() {
	println!("{}", rsoup::nodes::node::test());
}

fn helper_test() {
	// println!("{:?}", rsoup::helper::validate::is_true(true));
	// println!("{:?}", rsoup::helper::validate::fail("fail!!"));
}

fn string_util_test() {
	let vec : Vec<&str> = vec!["abcd", "efg", "hi", "jk"];
	let vec1 : Vec<&str> = vec!["abcd"];
	let vec2 : Vec<&str> = vec!();

	// test haystack
	println!("\ntest haystack()");
	println!("test haystack {:?}", rsoup::helper::string_util::in_haystack("abcd", &vec));
	println!("test haystack {:?}", rsoup::helper::string_util::in_haystack("abcde", &vec));

	// test join
	println!("\ntest join()");
	println!("{:?}", vec);
	println!("{:?}", rsoup::helper::string_util::join(&vec, "\t") );
	println!("{:?}", rsoup::helper::string_util::join(&vec1, "\t") );
	println!("{:?}", rsoup::helper::string_util::join(&vec2, "\t") );
	
	// test padding
	println!("\ntest padding()");
	println!("{:?}", rsoup::helper::string_util::padding(4));

	// test whitespace and blank
	println!("\ntest is_whitespace()");
	println!("{:?} {:?}", (' ').is_whitespace(),('t').is_whitespace() );

	println!("\ntest is_blank()");
	println!("{:?} {:?}", rsoup::helper::string_util::is_blank("\t\t\t"), rsoup::helper::string_util::is_blank("tptptp"));

	// test normalize_whitespace
	println!("\ntest normalize_whitespace()");
	println!("{:?}", rsoup::helper::string_util::nomalize_whitespace("\t\t\tkakaka\t\tkoko\t\t")); 

}