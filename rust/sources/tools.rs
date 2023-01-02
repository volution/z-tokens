

use crate::prelude::*;


use crate::{
		tools_generate::main as main_generate,
		tools_patterns::main as main_patterns,
	};




define_error! (pub MainError, result : MainResult);




pub fn main () -> MainResult {
	
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
	
	match _commands.iter () .map (String::as_str) .collect::<Vec<_>> () .as_slice () {
		&[] =>
			fail! (0xdcd5c356),
		&["patterns"] => {
			_arguments.insert (0, String::from ("patterns"));
			main_patterns (_arguments)
		}
		&["generate"] => {
			_arguments.insert (0, String::from ("generate"));
			main_generate (_arguments)
		}
		_ =>
			fail! (0x8e2b991f),
	}
}


