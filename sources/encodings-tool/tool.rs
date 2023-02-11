

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;




define_error! (pub MainError, result : MainResult);




pub fn main (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	{
		let mut _parser = create_parser () .else_wrap (0x2e40868a) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0x75b0a124) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	fail! (0xc8f6808d);
}


