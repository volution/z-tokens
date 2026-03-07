

use crate::prelude::*;


define_error! (pub ShortcutError, result : ShortcutResult);


pub fn generate_token_as_string (_pattern : &TokenPattern) -> ShortcutResult<String> {
	
	let mut _randomizer = OsRandomizer::from_os () .else_wrap (0x2814505f) ?;
	
	let _token = generate_token (_pattern, &mut _randomizer) .else_wrap (0x804566ab) ?;
	
	let _output_options = OutputOptions::default ();
	let _string = output_token_to_string (&_token, &_output_options) .else_wrap (0xddc4b0c4) ?;
	
	Ok (_string)
}


