

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;




pub use ::argparse::{
		ArgumentParser as ArgParser,
		Store as ArgStore,
		StoreTrue as ArgStoreTrue,
		StoreFalse as ArgStoreFalse,
		StoreConst as ArgStoreConst,
		StoreOption as ArgStoreOption,
	};


pub use ::std::process::ExitCode;




define_error! (pub FlagsError, result : FlagsResult);




pub fn create_parser <'a> () -> FlagsResult<ArgParser<'a>> {
	Ok (ArgParser::new ())
}




pub fn execute_parser (_parser : ArgParser, _arguments : Vec<String>) -> FlagsResult<bool> {
	
	match _parser.parse (_arguments, &mut stdout_locked (), &mut stderr_locked ()) {
		Ok (()) =>
			Ok (false),
		Err (0) =>
			Ok (true),
		Err (_error) =>
			fail! (0x0f71ad86),
	}
}


