use serde::{Deserialize, Serialize};
use std::collections::BTreeMap as Map;
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Struct {
	a: u32,
	b: String,
	#[serde(flatten)]
	other: Map<String, Value>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn it_works() {
		let raw_json = r#"{
			"a": 3,
			"b": "some string",
			"foo": "bar",
			"bar": "foo",
			"foobar": {
				"foo": "bar",
				"bar": "foo"
			}
		}"#;

		match serde_json::from_str::<Struct>(raw_json) {
			Ok(s) => println!("It just works: {:?}", s),
			Err(e) => println!("Error!")
		}
	}
}
