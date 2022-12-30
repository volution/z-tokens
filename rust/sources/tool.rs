

use crate::prelude::*;




define_error! (pub MainError, result : MainResult);




pub fn main () -> MainResult {
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
	for _pattern in patterns::all_token_patterns () .into_iter () {
		let &(ref _identifier, ref _pattern) = _pattern.as_ref ();
		
		let _entropy = entropy_token (&_pattern) .else_wrap (0x6374858a) ?;
		let _bits = _entropy.bits ();
		
		let _token = generate_token (&_pattern) .else_wrap (0xef0a3430) ?;
		let _string = output_token_to_string (&_token) .else_wrap (0x36471fa6) ?;
		
		if _bits > 256.0 {
			continue;
		}
		if _string.len () > 128 {
			continue;
		}
		
		writeln! (&mut _stream, "|  {:20}  |  {:6.1} bits  ||  {}", _identifier, _bits, _string) .else_wrap (0xd141c5ef) ?;
	}
	
	Ok (())
}


