pub struct Scanner {
  path: &str
  file_type: &str
  recursive: bool
}

pub struct ScanResult {
  files_scanned: uint16
  directories_scanned: uint16
}

impl Scanner {
  pub fn scan(&self) ->  ScanResult {
    
  }
}
