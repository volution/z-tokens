

use ::vrl_preludes::std_plus_extras::*;


use crate::tools::*;




pub fn bin () -> Result<ExitCode, ()> {
	
	match premain () {
		
		Ok (_code) =>
			Ok (_code),
		
		Err (_error) => {
			::std::eprintln! ("[ee] [347cb3ad]  unexpected error encountered;  aborting!");
			::std::eprintln! ("[ee] [{:08x}]  ||  {}", _error.error_code () .code (), _error.message_string () .as_deref () .unwrap_or ("[no message]"));
			Ok (ExitCode::FAILURE)
		}
	}
}


