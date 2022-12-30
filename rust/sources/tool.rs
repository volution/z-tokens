

use crate::prelude::*;




define_error! (pub MainError, result : MainResult);




pub fn main () -> MainResult {
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
	for _pattern in patterns::all_token_patterns () .into_iter () {
		let &(ref _identifier, ref _pattern) = _pattern.as_ref ();
		
		let _token = generate_token (&_pattern) .else_wrap (0xef0a3430) ?;
		let _entropy = entropy_token (&_pattern) .else_wrap (0x6374858a) ?;
		
		write! (&mut _stream, "|  {:25}  |  {:6.1} bits  ||  ", _identifier, _entropy.bits ()) .else_wrap (0xd141c5ef) ?;
		output_token (&_token, &mut _stream) .else_wrap (0xdef2b059) ?;
		writeln! (&mut _stream) .else_wrap (0x339d5a87) ?;
	}
	
	Ok (())
}


