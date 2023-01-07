

pub fn main () -> ::z_tokens::MainResult<::std::process::ExitCode> {
	match ::z_tokens::premain () {
		Ok (_code) =>
			Ok (_code),
		Err (_error) => {
			::std::eprintln! ("[ee] [347cb3ad]  unexpected error encountered;  aborting!");
			::std::eprintln! ("[ee] [{:08x}]  ||  {}", _error.error_code () .code (), _error.message_string () .as_deref () .unwrap_or ("[no message]"));
			Ok (::std::process::ExitCode::FAILURE)
		}
	}
}


