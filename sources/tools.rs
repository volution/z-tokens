

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
		
		(&["help"], _) | (&["h"], _) |
		(&[], &["--help"]) | (&[], &["-h"]) => {
			let mut _stdout = stdout_locked ();
			_stdout.write (MAIN_DOC.as_bytes ()) .else_wrap (0x1741e3c0) ?;
			Ok (ExitCode::SUCCESS)
		}
		
		(&["version"], _) | (&["v"], _) |
		(&[], &["--version"]) | (&[], &["-v"]) => {
			let mut _stdout = stdout_locked ();
			_stdout.write (MAIN_VERSION.as_bytes ()) .else_wrap (0x51f29851) ?;
			Ok (ExitCode::SUCCESS)
		}
		
		(&[], _) => {
			let mut _stderr = stderr_locked ();
			writeln! (_stderr, "[ee] [427cd93b]  expected command and arguments;  see `z-tokens help`;  aborting!") .else_wrap (0xf3c17b50) ?;
			Ok (ExitCode::SUCCESS)
		}
		
		(&["patterns"], _) | (&["p"], _) => {
			_arguments.insert (0, String::from ("z-tokens patterns"));
			main_patterns (_arguments)
		}
		
		(&["generate"], _) | (&["g"], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			main_generate (_arguments)
		}
		
		_ => {
			let mut _stderr = stderr_locked ();
			writeln! (_stderr, "[ee] [37d61e27]  invalid command;  see `z-tokens help`;  aborting!") .else_wrap (0x03f719cd) ?;
			Ok (ExitCode::FAILURE)
		}
	}
}




static MAIN_DOC : &'static str = include_str! ("./tools_manuals/main--help.txt");

static MAIN_VERSION : &'static str = include_str! ("./tools_build/version.txt");

