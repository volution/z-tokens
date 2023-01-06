

use crate::prelude::*;


use crate::{
		tools_generate::main as main_generate,
		tools_patterns::main as main_patterns,
	};




define_error! (pub MainError, result : MainResult);




pub fn main () -> MainResult<ExitCode> {
	
	let mut _arguments_os : Vec<_> = env::args_os () .collect ();
	if _arguments_os.is_empty () {
		fail! (0xf28dc498);
	} else {
		_arguments_os.remove (0);
	};
	
	let mut _commands = Vec::with_capacity (1);
	let mut _arguments = Vec::with_capacity (_arguments_os.len ());
	for _argument_os in _arguments_os.into_iter () {
		let _argument = _argument_os.into_string () .else_replace (0xa03c1f9c) ?;
		if ! _arguments.is_empty () {
			_arguments.push (_argument);
		} else {
			match _argument.chars () .next () {
				None =>
					fail! (0xc83bdfd9),
				Some ('-') =>
					_arguments.push (_argument),
				Some (_char) if (_char >= 'a') && (_char <= 'z') =>
					_commands.push (_argument),
				_ =>
					fail! (0xa9f7ef96),
			}
		}
	}
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		(&["patterns"], _) | (&["p"], _) => {
			_arguments.insert (0, String::from ("z-tokens patterns"));
			main_patterns (_arguments)
		}
		
		(&["generate"], _) | (&["g"], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			main_generate (_arguments)
		}
		(&["generate", _pattern], _) | (&["g", _pattern], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			_arguments.insert (1, String::from ("--token-pattern"));
			_arguments.insert (2, String::from (_pattern));
			main_generate (_arguments)
		}
		
		(&["help"], _) | (&["h"], _) |
		(&[], &["--help"]) | (&[], &["-h"]) |
		(&[], &["--help-text"]) =>
			print_and_exit (&[
						HELP_HEADER_TEXT,
						HELP_MAIN_TEXT.trim_matches ('\n'),
						"\n",
						HELP_FOOTER_TEXT,
					], true),
		
		(&["version"], _) | (&["v"], _) |
		(&[], &["--version"]) | (&[], &["-v"]) =>
			print_version_and_exit (true),
		
		(&[], &["--readme"]) |
		(&[], &["--readme-text"]) =>
			print_and_exit (&[README_TEXT], true),
		(&[], &["--readme-html"]) =>
			print_and_exit (&[README_HTML], true),
		
		(&[], &["--sbom"]) |
		(&[], &["--sbom-text"]) =>
			print_and_exit (&[SBOM_TEXT], true),
		(&[], &["--sbom-html"]) =>
			print_and_exit (&[SBOM_HTML], true),
		(&[], &["--sbom-json"]) =>
			print_and_exit (&[SBOM_JSON], true),
		
		(&[], &["--sources-md5"]) =>
			dump_and_exit (BUILD_SOURCES_MD5.as_bytes (), true),
		(&[], &["--sources-cpio"]) =>
			dump_and_exit (BUILD_SOURCES_CPIO_GZ, true),
		
		(&[], _) =>
			print_and_exit (&["[ee] [427cd93b]  expected command and arguments;  see `z-tokens help`;  aborting!", "\n"], false),
		
		_ =>
			print_and_exit (&["[ee] [37d61e27]  invalid command;  see `z-tokens help`;  aborting!", "\n"], false),
	}
}




fn print_version_and_exit (_succeed : bool) -> MainResult<ExitCode> {
	
	let _executable_name = "z-tokens";
	
	let _executable = match env::current_exe () {
			Ok (_executable) =>
				_executable,
			Err (_error) => {
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
	
	let _build_version = BUILD_VERSION.trim_matches ('\n');
	let _build_number = BUILD_NUMBER.trim_matches ('\n');
	let _build_timestamp = BUILD_TIMESTAMP.trim_matches ('\n');
	let _build_git_hash = BUILD_GIT_HASH.trim_matches ('\n');
	let _build_sources_hash = BUILD_SOURCES_HASH.trim_matches ('\n');
	
	let _uname = match ::platform_info::PlatformInfo::new () {
			Ok (_uname) =>
				Some (_uname),
			Err (_error) => {
				::std::eprintln! ("[ee] [0fcf6fae]  unexpected error encountered;  ignoring!  //  {}", _error);
				None
			}
		};
	let (_uname_node, _uname_system, _uname_release, _uname_machine) = if let Some (_uname) = &_uname {
			use ::platform_info::Uname as _;
			(_uname.nodename (), _uname.sysname (), _uname.release (), _uname.machine ())
		} else {
			("{unknown}".into (), "{unknown}".into (), "{unknown}".into (), "{unknown}".into ())
		};
	
	print_and_exit (&[
			
			& format! ("* tool          : {}\n", _executable_name),
			& format! ("* version       : {}\n", _build_version),
			& if _executable0 == _executable {
				format! ("* executable    : {}\n", _executable.display ())
			} else {
				format! ("* executable    : {}\n", _executable.display ()) + &
				format! ("* executable-0  : {}\n", _executable0.display ())
			},
			& format! ("* build target  : {}\n", BUILD_TARGET_TYPE),
			& format! ("* build number  : {}, {}\n", _build_number, _build_timestamp),
			& format! ("* code & issues : {}\n", PROJECT_URL),
			& format! ("* sources git   : {}\n", _build_git_hash),
			& format! ("* sources hash  : {}\n", _build_sources_hash),
			& format! ("* uname node    : {}\n", _uname_node),
			& format! ("* uname system  : {}, {}, {}\n", _uname_system, _uname_release, _uname_machine),
		//	& format! ("* uname hash    : {}\n", _uname_fingerprint),
			
		], _succeed)
}




fn print_and_exit (_chunks : &[impl AsRef<str>], _success : bool) -> MainResult<ExitCode> {
	
	let mut _stream = BufWriter::with_capacity (16 * 1024, stdout_locked ());
	
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


fn dump_and_exit (_data : &[u8], _success : bool) -> MainResult<ExitCode> {
	
	let mut _stream = stdout_locked ();
	
	_stream.write (_data) .else_wrap (0xfbf75f69) ?;
	
	drop (_stream);
	
	if _success {
		Ok (ExitCode::SUCCESS)
	} else {
		Ok (ExitCode::FAILURE)
	}
}




static PROJECT_URL : &'static str = "https://github.com/volution/z-tokens";


static README_TEXT : &'static str = include_str! ("./_embedded/readme/readme.txt");
static README_HTML : &'static str = include_str! ("./_embedded/readme/readme.html");


static HELP_MAIN_TEXT : &'static str = include_str! ("./_embedded/help/main.txt");

static HELP_HEADER_TEXT : &'static str = include_str! ("./_embedded/help/_header.txt");
static HELP_FOOTER_TEXT : &'static str = include_str! ("./_embedded/help/_footer.txt");


static SBOM_TEXT : &'static str = include_str! ("./_embedded/sbom/sbom.txt");
static SBOM_HTML : &'static str = include_str! ("./_embedded/sbom/sbom.html");
static SBOM_JSON : &'static str = include_str! ("./_embedded/sbom/sbom.json");


static BUILD_VERSION : &'static str = include_str! ("./_embedded/build/version.txt");
static BUILD_NUMBER : &'static str = include_str! ("./_embedded/build/number.txt");
static BUILD_TIMESTAMP : &'static str = include_str! ("./_embedded/build/timestamp.txt");

static BUILD_SOURCES_HASH : &'static str = include_str! ("./_embedded/build/sources.hash");
static BUILD_SOURCES_MD5 : &'static str = include_str! ("./_embedded/build/sources.md5");
static BUILD_SOURCES_CPIO_GZ : &'static [u8] = include_bytes! ("./_embedded/build/sources.cpio.gz");

static BUILD_GIT_HASH : &'static str = if let Some (_value) = ::std::option_env! ("__META__BUILD_GIT_HASH") { _value } else { "{unknown-bgh}" };
static BUILD_TARGET_TYPE : &'static str = if let Some (_value) = ::std::option_env! ("__META__BUILD_TARGET_TYPE") { _value } else { "{unknown-btt}" };

