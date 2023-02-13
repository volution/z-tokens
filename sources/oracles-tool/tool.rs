

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;








define_error! (pub MainError, result : MainResult);








pub fn main (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	
	if _arguments.len () != 1 {
		fail! (0xb5340b89);
	}
	
	
	fail! (0x0886b17c);
}


