static booleanAttributes: [&'static str; 30] = [
            "allowfullscreen", "async", "autofocus", "checked", "compact", "declare", "default", "defer", "disabled",
            "formnovalidate", "hidden", "inert", "ismap", "itemscope", "multiple", "muted", "nohref", "noresize",
            "noshade", "novalidate", "nowrap", "open", "readonly", "required", "reversed", "seamless", "selected",
            "sortable", "truespeed", "typemustmatch"
    ];

pub struct Attribute {
    key: String,
    value: String,
}

impl Attribute {
	pub fn new(key: &str, value: &str) -> Attribute {
		Attribute { key: key.to_string(), value: value.to_string() }
	}

	pub fn get_value(&self) -> String { self.value.clone() }

	// pub fn html() -> String {}

	// fn html(accum: StringBuilder, Document.OutputSettings out) {}

	// pub fn to_string() -> String { html() }
}

impl Clone for Attribute {
	fn clone(&self) -> Attribute {
		Attribute { key: self.key.clone(), value: self.value.clone() }
	}
}