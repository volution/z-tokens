

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::errors::*;
use ::z_tokens_runtime_flags::*;








define_error! (pub MainError, result : MainResult);








pub fn main_experiments (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	
	if _arguments.len () != 1 {
		fail! (0xb5340b89);
	}
	
	
	fail! (0x0886b17c);
}


