

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::embeddings::*;
use crate::runtime::*;


use ::z_tokens_runtime::crates::platform_info::{
		PlatformInfo,
		PlatformInfoAPI as _,
		UNameAPI as _,
	};








pub(crate) fn print_version_and_exit (_succeed : bool) -> MainResult<ExitCode> {
	
	let _executable_name = "z-tokens";
	
	let _executable = match env::current_exe () {
			Ok (_executable) =>
				_executable,
			Err (_error) => {
				#[ cfg (not (target_os = "wasi")) ]
				::std::eprintln! ("[ee] [51d238d5]  unexpected error encountered;  ignoring!  //  {}", _error);
				PathBuf::from ("<unknown>")
			}
		};
	
	let _executable0 = match env::args_os () .into_iter () .next () {
			Some (_argument) =>
				PathBuf::from (_argument),
			None => {
				::std::eprintln! ("[ee] [8102483f]  unexpected error encountered;  ignoring!");
				PathBuf::from ("<unknown>")
			}
		};
	
	#[ cfg (feature = "zt-embedded-build-meta") ]
	let _build_version = BUILD_VERSION.trim_matches ('\n');
	#[ cfg (feature = "zt-embedded-build-meta") ]
	let _build_number = BUILD_NUMBER.trim_matches ('\n');
	#[ cfg (feature = "zt-embedded-build-meta") ]
	let _build_timestamp = BUILD_TIMESTAMP.trim_matches ('\n');
	#[ cfg (feature = "zt-embedded-build-meta") ]
	let _build_git_hash = BUILD_GIT_HASH.trim_matches ('\n');
	#[ cfg (feature = "zt-embedded-sources") ]
	let _build_sources_hash = BUILD_SOURCES_HASH.trim_matches ('\n');
	
	let _uname = match PlatformInfo::new () {
			Ok (_uname) =>
				Some (_uname),
			Err (_error) => {
				::std::eprintln! ("[ee] [0fcf6fae]  unexpected error encountered;  ignoring!  //  {}", _error);
				None
			}
		};
	let (_uname_node, _uname_system, _uname_release, _uname_machine) = if let Some (_uname) = &_uname {
			(
				_uname.nodename () .to_string_lossy (),
				_uname.sysname () .to_string_lossy (),
				_uname.release () .to_string_lossy (),
				_uname.machine () .to_string_lossy (),
			)
		} else {
			("{unknown}".into (), "{unknown}".into (), "{unknown}".into (), "{unknown}".into ())
		};
	
	print_and_exit (&[
			
			& format! ("* tool          : {}\n", _executable_name),
			
			#[ cfg (feature = "zt-embedded-build-meta") ]
			& format! ("* version       : {}\n", _build_version),
			
			& if _executable0 == _executable {
				format! ("* executable    : {}\n", _executable.display ())
			} else {
				format! ("* executable    : {}\n", _executable.display ()) + &
				format! ("* executable-0  : {}\n", _executable0.display ())
			},
			
			#[ cfg (feature = "zt-embedded-build-meta") ]
			& format! ("* build target  : {}\n", BUILD_TARGET_TYPE),
			#[ cfg (feature = "zt-embedded-build-meta") ]
			& format! ("* build number  : {}, {}\n", _build_number, _build_timestamp),
			
			& format! ("* code & issues : {}\n", PROJECT_URL),
			
			#[ cfg (feature = "zt-embedded-build-meta") ]
			& format! ("* sources git   : {}\n", _build_git_hash),
			#[ cfg (feature = "zt-embedded-sources") ]
			& format! ("* sources hash  : {}\n", _build_sources_hash),
			
			& format! ("* uname node    : {}\n", _uname_node),
			& format! ("* uname system  : {}, {}, {}\n", _uname_system, _uname_release, _uname_machine),
		//	& format! ("* uname hash    : {}\n", _uname_fingerprint),
			
		], _succeed)
}








#[ allow (dead_code) ]
pub(crate) fn print_and_exit (_chunks : &[impl AsRef<str>], _success : bool) -> MainResult<ExitCode> {
	
	let mut _stream = BufWriter::with_capacity (IO_BUFFER_SIZE, stdout_locked ());
	
	for _chunk in _chunks {
		_stream.write (_chunk.as_ref () .as_bytes ()) .else_wrap (0x4c5c446d) ?;
	}
	
	drop (_stream.into_inner () .else_replace (0xbbc5724c) ?);
	
	if _success {
		Ok (ExitCode::SUCCESS)
	} else {
		Ok (ExitCode::FAILURE)
	}
}




#[ allow (dead_code) ]
pub(crate) fn dump_and_exit (_data : &[u8], _success : bool) -> MainResult<ExitCode> {
	
	let mut _stream = stdout_locked ();
	
	_stream.write (_data) .else_wrap (0xfbf75f69) ?;
	
	drop (_stream);
	
	if _success {
		Ok (ExitCode::SUCCESS)
	} else {
		Ok (ExitCode::FAILURE)
	}
}


