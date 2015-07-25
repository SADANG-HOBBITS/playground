use std::collections::HashMap;
use std::collections::hash_map::Iter;

use super::attribute::Attribute;

static dataPrefix: &'static str = "data-";

pub struct Attributes {
	attributes: HashMap<String, Attribute>,
}

impl Attributes {
	pub fn new() -> Attributes {
		Attributes { attributes: HashMap::<String, Attribute>::new() }
	}

	pub fn get(&self, key: &str) -> String {
		// TODO: validate key
		// validate.not_empty(key);

		self.attributes.get(key).unwrap().get_value()
	}

	pub fn put(&mut self, key: &str, value: &str) {
		self.attributes.insert(key.to_string(), Attribute::new(key, value));
	}

	pub fn put_attribute(&mut self, attribute: Attribute) {
		self.attributes.insert(attribute.get_value(), attribute.clone());
	}

	pub fn remove(&mut self, key: &str) {
		self.attributes.remove(key);
	}

	pub fn has_key(&self, key: &str) -> bool {
		self.attributes.contains_key(key)
	}

	pub fn size(&self) -> usize {
		self.attributes.len()
	}

	pub fn iter(&self) -> Iter<String, Attribute> {
		self.attributes.iter()
	}

	pub fn add_all(&mut self, attributes: &Attributes) {
		if attributes.size() > 0 {
			for (key, value) in attributes.iter() {
				self.attributes.insert(key.to_string(), value.clone());
			}
		}
	}
}

impl Clone for Attributes {
	fn clone(&self) -> Attributes {
		Attributes { attributes: self.attributes.clone() }
	}
}