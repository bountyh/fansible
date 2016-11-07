pub struct Scanner {
  path: &str,
  file_type: &str,
  recursive: bool,
}

pub struct ScanResult {
  files_scanned: u16,
  directories_scanned: u16,
}

impl Scanner {
  pub fn scan(&self) -> ScanResult {
    
  }
}
