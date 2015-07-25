use std::string::String;
use std::boxed::Box;
use std::fmt;

#[derive(Debug)]
pub enum ExceptionType {
    IlligalArgumentException { information: &'static str},
    //IncorrectFuncElemet
}


pub fn not_null<T>(obj: T) -> Result<bool, ExceptionType> {
	let optional: Option<Box<T>> = Some(Box::new(obj));

	match optional {
		Some(_) => Ok(true),
		None => Err(ExceptionType::IlligalArgumentException { information: "Object must not be null"})
	}
}

pub fn is_true(val: bool) -> Result<bool, ExceptionType> {
	match val {
		true => Ok(true),
		false => Err(ExceptionType::IlligalArgumentException { information: "Must be true"})
	}
}

pub fn is_true_msg(val: bool, msg: &'static str) -> Result<bool, ExceptionType> {
	match  val {
		true => Ok(true),
		false => Err(ExceptionType::IlligalArgumentException { information: msg})
	}
}

pub fn is_false(val: bool) -> Result<bool, ExceptionType> {
	match val {
		false => Ok(true),
		true => Err(ExceptionType::IlligalArgumentException { information: "Must be true"})
	}	
}

// pub fn not_null_element<T>(objs: [T]) -> Result<bool, ExceptionType> {
// 	for obj in objs.iter() {
// 		let optional: Option<Box<T>> = Some(Box::new(*obj));

// 		match optional {
// 			Some(_) => continue,
// 			None => return Err(ExceptionType::IlligalArgumentException { information: "Array must not contain any null objects"})
// 		}
// 	}

// 	Ok(true)
// }

pub fn string_not_empty(input_string: &String) -> Result<bool, ExceptionType> {
	match input_string.is_empty() {
		true => Err(ExceptionType::IlligalArgumentException {information: "String must not be empty" } ),
		false => Ok(true)
	}
}

pub fn fail(input_string: &'static str) -> Result<bool, ExceptionType> {
	Err(ExceptionType::IlligalArgumentException {information: input_string})
}

pub trait Validator {

}

