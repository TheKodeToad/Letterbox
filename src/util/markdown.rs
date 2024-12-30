// hacky but it works pretty well
// can't find any crates for this 🤷‍♀️
const FORMATTING_CHARS: &str = r"\/*_-`#@<>.~|:[]()";

pub fn escape(input: &str) -> String {
	let mut result = String::new();

	for char in input.chars() {
		if FORMATTING_CHARS.contains(char) {
			result.push('\\');
		}

		result.push(char);
	}

	result
}
