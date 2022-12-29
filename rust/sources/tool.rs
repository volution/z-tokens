

use crate::prelude::*;




define_error! (pub MainError, result : MainResult);




pub fn main () -> MainResult {
	
	let _pattern = TokenPattern::Empty;
	
	let _token = generate_token (&_pattern) .else_wrap (0xef0a3430) ?;
	
	output_token (&_token, io::stdout () .lock ()) .else_wrap (0xdef2b059) ?;
	
	Ok (())
}


