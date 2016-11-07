use glob::glob;

pub struct Scanner {
	pub path: String,
	pub file_type: String,
	pub recursive: bool,
}

pub struct ScanResult {
 	pub files_found: Vec<String>,
}

impl Scanner {
  pub fn scan(&self) -> ScanResult {
  	let path =
  	if self.recursive { 
  		format!("{}{}{}", self.path, "/**/*", self.file_type)
  	} else {
  		format!("{}{}{}", self.path, "/*", self.file_type)
  	};

  	let scan_for: &str = &path[..];

    for entry in glob(scan_for).expect("Failed to read glob pattern") {
	    match entry {
	        Ok(path) => println!("{:?}", path.display()),
	        Err(e) => println!("{:?}", e),
	    }
	};

	ScanResult { files_found: vec!("string".to_string()) }
  }
}
