

use crate::prelude::*;


use crate::{
		tools_generate::main as main_generate,
		tools_patterns::main as main_patterns,
	};




define_error! (pub MainError, result : MainResult);




pub fn main () -> MainResult<ExitCode> {
	
	let mut _arguments_os : Vec<_> = env::args_os () .collect ();
	if _arguments_os.is_empty () {
		fail! (0xf28dc498);
	} else {
		_arguments_os.remove (0);
	};
	
	let mut _commands = Vec::with_capacity (1);
	let mut _arguments = Vec::with_capacity (_arguments_os.len ());
	for _argument_os in _arguments_os.into_iter () {
		let _argument = _argument_os.into_string () .else_replace (0xa03c1f9c) ?;
		if ! _arguments.is_empty () {
			_arguments.push (_argument);
		} else {
			match _argument.chars () .next () {
				None =>
					fail! (0xc83bdfd9),
				Some ('-') =>
					_arguments.push (_argument),
				Some (_char) if (_char >= 'a') && (_char <= 'z') =>
					_commands.push (_argument),
				_ =>
					fail! (0xa9f7ef96),
			}
		}
	}
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		(&["patterns"], _) | (&["p"], _) => {
			_arguments.insert (0, String::from ("z-tokens patterns"));
			main_patterns (_arguments)
		}
		
		(&["generate"], _) | (&["g"], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			main_generate (_arguments)
		}
		
		(&["help"], _) | (&["h"], _) |
		(&[], &["--help"]) | (&[], &["-h"]) |
		(&[], &["--help-text"]) =>
			print_and_exit (&[
						HELP_HEADER_TEXT,
						HELP_MAIN_TEXT.trim_matches ('\n'),
						"\n",
						HELP_FOOTER_TEXT,
					], true),
		
		(&["version"], _) | (&["v"], _) |
		(&[], &["--version"]) | (&[], &["-v"]) =>
			print_and_exit (&[BUILD_VERSION], true),
		
		(&[], &["--readme"]) |
		(&[], &["--readme-text"]) =>
			print_and_exit (&[README_TEXT], true),
		(&[], &["--readme-html"]) =>
			print_and_exit (&[README_HTML], true),
		
		(&[], &["--sbom"]) |
		(&[], &["--sbom-text"]) =>
			print_and_exit (&[SBOM_TEXT], true),
		(&[], &["--sbom-html"]) =>
			print_and_exit (&[SBOM_HTML], true),
		(&[], &["--sbom-json"]) =>
			print_and_exit (&[SBOM_JSON], true),
		
		(&[], _) =>
			print_and_exit (&["[ee] [427cd93b]  expected command and arguments;  see `z-tokens help`;  aborting!", "\n"], false),
		
		_ =>
			print_and_exit (&["[ee] [37d61e27]  invalid command;  see `z-tokens help`;  aborting!", "\n"], false),
	}
}




fn print_and_exit (_chunks : &[&str], _success : bool) -> MainResult<ExitCode> {
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
	for _chunk in _chunks {
		_stream.write (_chunk.as_bytes ()) .else_wrap (0x4c5c446d) ?;
	}
	
	drop (_stream.into_inner () .else_replace (0xbbc5724c) ?);
	
	if _success {
		Ok (ExitCode::SUCCESS)
	} else {
		Ok (ExitCode::FAILURE)
	}
}




static HELP_MAIN_TEXT : &'static str = include_str! ("./_embedded/manuals/main--help.txt");

static HELP_HEADER_TEXT : &'static str = include_str! ("./_embedded/manuals/help--header.txt");
static HELP_FOOTER_TEXT : &'static str = include_str! ("./_embedded/manuals/help--footer.txt");


static README_TEXT : &'static str = include_str! ("./_embedded/readme/readme.txt");
static README_HTML : &'static str = include_str! ("./_embedded/readme/readme.html");


static SBOM_TEXT : &'static str = include_str! ("./_embedded/sbom/sbom.txt");
static SBOM_HTML : &'static str = include_str! ("./_embedded/sbom/sbom.html");
static SBOM_JSON : &'static str = include_str! ("./_embedded/sbom/sbom.json");


static BUILD_VERSION : &'static str = include_str! ("./_embedded/build/version.txt");

