

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;

pub use ::vrl_cli_arguments::*;

pub use ::std::process::ExitCode;




define_error! (pub FlagsError, result : FlagsResult);




pub fn create_flags <'a> () -> FlagsResult<FlagsParserBuilder<'a>> {
	Ok (FlagsParserBuilder::new ())
}




pub fn execute_flags (mut _parser : FlagsParserBuilder, mut _arguments : Vec<String>) -> FlagsResult<bool> {
	
	// FIXME!
	_arguments.remove (0);
	
	_parser.define_version ('v', "version");
	_parser.define_help ('h', "help");
	
	let _parser = _parser.build () .else_wrap (0xf8191edb) ?;
	let _parsed = _parser.parse_vec_string (_arguments);
	
	if _parsed.is_version_requested () {
		fail! (0xee8fb3cb);
	}
	if _parsed.is_help_requested () {
		_parsed.help_print (stdout_locked ()) .else_wrap (0xeb9669f7) ?;
		return Ok (true);
	}
	_parsed.done () .else_wrap (0x2e356555) ?;
	
	Ok (false)
}


