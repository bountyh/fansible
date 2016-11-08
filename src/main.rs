extern crate glob;
extern crate rustc_serialize;
extern crate docopt;

use docopt::Docopt;

pub mod scanner;
pub mod initializer;

const USAGE: &'static str = "
Usage:
	fansible init <directory> [ --recursive ]
	fansible <project> [ --list ]
	fansible ls
	fansible <project> <play>
Options:
  -h --help		Show this screen.
  -v 			Show version.
";

#[derive(RustcDecodable)]
struct Args {
	arg_directory: String,
	arg_project: String,
	arg_play: String,
	flag_recursive: bool,
}

fn main() {
	let args: Args = Docopt::new(USAGE)
                            .and_then(|d| d.decode())
                            .unwrap_or_else(|e| e.exit());
}