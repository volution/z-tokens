

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;




define_error! (pub MainError, result : MainResult);




pub fn main (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	if ! _arguments.is_empty () {
		fail! (0xb1bda4b1);
	}
	
	fail! (0x8829df41);
}


