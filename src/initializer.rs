use scanner::*;

pub struct Initializer {
	directory: String,
	playbooks: Vec<String>,
}

impl Initializer {
	pub fn scan_dir(path: String, file_type: String, recursive: bool) -> ScanResult {
		let scanner  = Scanner { path: path, file_type: file_type, recursive: recursive };
		scanner.scan()
	}
}
