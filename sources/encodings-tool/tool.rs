

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;
use ::z_tokens_runtime_flags::*;




define_error! (pub MainError, result : MainResult);




pub fn main_encode <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	{
		let mut _flags = create_flags () .else_wrap (0x2e40868a) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0x75b0a124) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	fail! (0xc8f6808d);
}




pub fn main_decode <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	{
		let mut _flags = create_flags () .else_wrap (0xe6cc7904) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0x53f611ac) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	fail! (0xf492145b);
}


