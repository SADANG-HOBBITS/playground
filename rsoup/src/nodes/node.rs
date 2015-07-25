use super::attributes::Attributes;

pub fn test() -> String {
	"nodes/node.rs".to_string()
}

pub trait NodeTrait {
	fn get_node<'a>(&'a self) -> &'a Node;

	// fn get_attribute_value(&self, attribute_key: &str) -> String;

	// fn get_attributes(&self) -> Attributes;

	// // TODO: check the necessity for returning Node type
	// fn add_new_attribute(&self, attribute_key: &str, attribute_value: &str) -> Node;

	// fn has_attribute(&self, attribute_key: &str) -> bool;

	// // TODO: check the necessity for returning Node type
	// fn remove_attribute(&self, attribute_key: &str) -> Node;

	// fn get_base_url(&self) -> String;

	// fn set_base_url(&self, base_url: &str);

	// fn get_abs_url(&self, attribute_key: &str) -> String;

	// fn get_child_node(&self, index: usize) -> Node;

	fn get_child_nodes<'a>(&'a self) -> &Vec<Option<Box<Node>>>;

	// fn get_child_nodes_clone(&self) -> Vec<Option<Box<Node>>>;

	fn child_nodes_size(&self) -> usize;
}

pub struct Node {
	parent_node: Option<Box<Node>>,
	childNodes: Vec<Option<Box<Node>>>,
	attributes: Attributes,
	baseUrl: String,
	siblingIndex: usize,
}

impl NodeTrait for Node {
	fn get_node<'a>(&'a self) -> &'a Node { self }

	fn get_child_nodes<'a>(&'a self) -> &'a Vec<Option<Box<Node>>> { &self.childNodes }

	fn child_nodes_size(&self) -> usize { self.childNodes.len() }
}