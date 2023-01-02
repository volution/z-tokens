

use crate::prelude::*;

use crate::tools::*;
use crate::tools_flags::*;




pub fn main (_arguments : Vec<String>) -> MainResult {
	
	
	let mut _randomizer_flags = RandomizerFlags::new () .else_wrap (0x839efea4) ?;
	
	{
		let mut _parser = ArgParser::new ();
		_randomizer_flags.parser (&mut _parser) .else_wrap (0x7a560f7c) ?;
		_parser.parse (_arguments, &mut stdout_locked (), &mut stderr_locked ()) .else_replace (0x06626616) ?;
	}
	
	let mut _randomizer = _randomizer_flags.build () .else_wrap (0xa43471c4) ?;
	let _randomizer = _randomizer.deref_mut ();
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
	for _pattern in patterns::all_token_patterns () .into_iter () {
		let &(ref _identifier, ref _pattern) = _pattern.as_ref ();
		
		if _identifier.contains ("-upper-") || _identifier.contains ("-upper:") {
			continue;
		}
		
		let _entropy = entropy_token (&_pattern) .else_wrap (0x6374858a) ?;
		let (_bits, _bits_exact) = _entropy.bits_exact ();
		
		if _bits > 256.0 {
			continue;
		}
		
		let _token = generate_token (&_pattern, _randomizer) .else_wrap (0xef0a3430) ?;
		let _string = output_token_to_string (&_token) .else_wrap (0x36471fa6) ?;
		
		if _string.len () > 128 {
			continue;
		}
		
		if _bits_exact {
			writeln! (&mut _stream, "|  {:22}  |  {:4.0}   bits  ||  {}", _identifier, _bits, _string) .else_wrap (0xd141c5ef) ?;
		} else {
			let _display_bits = (_bits * 10.0) .floor () / 10.0;
			writeln! (&mut _stream, "|  {:22}  |  {:6.1} bits  ||  {}", _identifier, _display_bits, _string) .else_wrap (0xd141c5ef) ?;
		}
	}
	
	Ok (())
}


