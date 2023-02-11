

use ::vrl_preludes::std_plus_extras::*;


use crate::tools::*;




pub fn bin_tools () -> Result<ExitCode, ()> {
	bin_wrapper (premain_tools)
}

#[ cfg (feature = "z-tokens-hashes-tool") ]
pub fn bin_hashes () -> Result<ExitCode, ()> {
	bin_wrapper (premain_hashes)
}

#[ cfg (feature = "z-tokens-encodings-tool") ]
pub fn bin_encodings () -> Result<ExitCode, ()> {
	bin_wrapper (premain_encodings)
}

#[ cfg (feature = "z-tokens-exchange-tool") ]
pub fn bin_exchange () -> Result<ExitCode, ()> {
	bin_wrapper (premain_exchange)
}




pub fn bin_wrapper <Main> (_main : Main) -> Result<ExitCode, ()> where Main : FnOnce () -> MainResult<ExitCode> {
	
	match _main () {
		
		Ok (_code) =>
			Ok (_code),
		
		Err (_error) => {
			::std::eprintln! ("[ee] [347cb3ad]  unexpected error encountered;  aborting!");
			::std::eprintln! ("[ee] [{:08x}]  ||  {}", _error.error_code () .code (), _error.message_string () .as_deref () .unwrap_or ("[no message]"));
			::std::eprintln! ("[ee] [{:08x}]  ||  {}", _error.error_code () .code (), _error.to_string ());
			Ok (ExitCode::FAILURE)
		}
	}
}


