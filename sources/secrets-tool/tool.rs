

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;
use ::z_tokens_runtime_flags::*;




define_error! (pub MainError, result : MainResult);




pub fn main_secrets <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	if _arguments.has_arguments () {
		fail! (0xd4a80d9a);
	}
	
	fail! (0xfc7dd373);
}


