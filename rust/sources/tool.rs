

use crate::prelude::*;




define_error! (pub MainError, result : MainResult);




pub fn main () -> MainResult {
	
	// let mut _randomizer = RngRandomizer::from_os () .else_wrap (0xa8d84147) ?;
	let mut _randomizer = RngRandomizer::for_testing () .else_wrap (0x341026da) ?;
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
	for _pattern in patterns::all_token_patterns () .into_iter () {
		let &(ref _identifier, ref _pattern) = _pattern.as_ref ();
		
		if _identifier.contains ("-upper-") {
			continue;
		}
		
		let _entropy = entropy_token (&_pattern) .else_wrap (0x6374858a) ?;
		let _bits = _entropy.bits ();
		
		if _bits > 256.0 {
			continue;
		}
		
		let _token = generate_token (&_pattern, &mut _randomizer) .else_wrap (0xef0a3430) ?;
		let _string = output_token_to_string (&_token) .else_wrap (0x36471fa6) ?;
		
		if _string.len () > 128 {
			continue;
		}
		
		writeln! (&mut _stream, "|  {:20}  |  {:6.1} bits  ||  {}", _identifier, _bits, _string) .else_wrap (0xd141c5ef) ?;
	}
	
	Ok (())
}


