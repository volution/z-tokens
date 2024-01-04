

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;
use ::z_tokens_runtime_flags::*;




define_error! (pub MainError, result : MainResult);




pub fn main (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	if ! _arguments.is_empty () {
		fail! (0xd4a80d9a);
	}
	
	fail! (0xfc7dd373);
}


