

use ::vrl_preludes::std_plus_extras::*;

pub(crate) use ::vrl_errors::*;


#[ cfg (feature = "z-tokens-patterns-tool") ]
use ::z_tokens_patterns_tool::{
		
		generate::main as main_generate,
		patterns::main as main_patterns,
	};

#[ cfg (feature = "z-tokens-hashes-tool") ]
use ::z_tokens_hashes_tool::{
		tool::main as main_hash,
	};

#[ cfg (feature = "z-tokens-exchange-tool") ]
use ::z_tokens_exchange_tool::{
		tool::main_keys as main_exchange_keys,
		tool::main_encrypt as main_exchange_encrypt,
		tool::main_decrypt as main_exchange_decrypt,
		tool::main_password as main_exchange_password,
		tool::main_armor as main_exchange_armor,
		tool::main_dearmor as main_exchange_dearmor,
		tool::main_encode as main_exchange_encode,
		tool::main_decode as main_exchange_decode,
		tool::main_ssh_keys as main_exchange_ssh_keys,
		tool::main_ssh_wrap as main_exchange_ssh_wrap,
	};


#[ cfg (feature = "zt-runtime-allocator") ]
use ::z_tokens_runtime::allocator;


pub(crate) use ::std::process::ExitCode;


pub(crate) const IO_BUFFER_SIZE : usize = 4 * 1024;




define_error! (pub MainError, result : MainResult);




pub fn premain_tools () -> MainResult<ExitCode> {
	premain_wrapper (main_tools)
}

#[ cfg (feature = "z-tokens-hashes-tool") ]
pub fn premain_hashes () -> MainResult<ExitCode> {
	premain_wrapper (main_hashes)
}

#[ cfg (feature = "z-tokens-exchange-tool") ]
pub fn premain_exchange () -> MainResult<ExitCode> {
	premain_wrapper (main_exchange)
}




pub fn premain_wrapper <Main> (_main : Main) -> MainResult<ExitCode> where Main : FnOnce () -> MainResult<ExitCode> {
	
	#[ cfg (feature = "zt-runtime-allocator") ]
	if allocator::DEBUG_REPORT {
		allocator::report ();
	}
	
	let _outcome = _main ();
	
	#[ cfg (feature = "zt-runtime-allocator") ]
	if allocator::DEBUG_REPORT {
		allocator::report ();
	}
	
	_outcome
}








pub fn main_tools () -> MainResult<ExitCode> {
	
	let (mut _commands, mut _arguments) = main_arguments () ?;
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		
		
		
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		(&["patterns"], _) | (&["p"], _) => {
			_arguments.insert (0, String::from ("z-tokens patterns"));
			main_patterns (_arguments) .else_wrap (0x9093f429)
		}
		
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		(&["generate"], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			main_generate (_arguments) .else_wrap (0x7565abe0)
		}
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		(&["g"], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			_arguments.insert (1, String::from ("--compact"));
			_arguments.insert (2, String::from ("true"));
			_arguments.insert (3, String::from ("--token-count"));
			_arguments.insert (4, String::from ("1"));
			main_generate (_arguments) .else_wrap (0x6a8d26ca)
		}
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		(&["g", _pattern], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			_arguments.insert (1, String::from ("--compact"));
			_arguments.insert (2, String::from ("true"));
			_arguments.insert (3, String::from ("--token-count"));
			_arguments.insert (4, String::from ("1"));
			_arguments.insert (5, String::from ("--token-pattern"));
			_arguments.insert (6, String::from (_pattern));
			main_generate (_arguments) .else_wrap (0x284c1286)
		}
		
		
		
		
		#[ cfg (feature = "z-tokens-hashes-tool") ]
		(&["hash"], _) => {
			_arguments.insert (0, String::from ("z-tokens hash"));
			main_hash (_arguments) .else_wrap (0xff8dcc61)
		}
		
		
		
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "keys"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange keys"));
			main_exchange_keys (_arguments) .else_wrap (0x0df94b2b)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "encrypt"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange encrypt"));
			main_exchange_encrypt (_arguments) .else_wrap (0xef766e05)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "decrypt"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange decrypt"));
			main_exchange_decrypt (_arguments) .else_wrap (0xa73d3123)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "password"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange password"));
			main_exchange_password (_arguments) .else_wrap (0x07f0d87b)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "armor"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange armor"));
			main_exchange_armor (_arguments) .else_wrap (0xcc846bd9)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "dearmor"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange dearmor"));
			main_exchange_dearmor (_arguments) .else_wrap (0x605c4c42)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "raw", "encode"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange raw encode"));
			main_exchange_encode (_arguments) .else_wrap (0x0f6f25f9)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "raw", "decode"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange raw decode"));
			main_exchange_decode (_arguments) .else_wrap (0x4ea46e82)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "ssh", "keys"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange ssh keys"));
			main_exchange_ssh_keys (_arguments) .else_wrap (0xfe84133d)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["exchange", "ssh", "wrap"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange ssh wrap"));
			main_exchange_ssh_wrap (_arguments) .else_wrap (0x3108dc57)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}








#[ cfg (feature = "z-tokens-hashes-tool") ]
pub fn main_hashes () -> MainResult<ExitCode> {
	
	let (mut _commands, mut _arguments) = main_arguments () ?;
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		
		
		
		(&[], _) => {
			_arguments.insert (0, String::from ("z-hashes"));
			main_hash (_arguments) .else_wrap (0xf90b7753)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}








#[ cfg (feature = "z-tokens-exchange-tool") ]
pub fn main_exchange () -> MainResult<ExitCode> {
	
	let (mut _commands, mut _arguments) = main_arguments () ?;
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		
		
		
		(&["keys"], _) => {
			_arguments.insert (0, String::from ("z-exchange keys"));
			main_exchange_keys (_arguments) .else_wrap (0x7685fa9c)
		}
		(&["encrypt"], _) => {
			_arguments.insert (0, String::from ("z-exchange encrypt"));
			main_exchange_encrypt (_arguments) .else_wrap (0xadd1e78c)
		}
		(&["decrypt"], _) => {
			_arguments.insert (0, String::from ("z-exchange decrypt"));
			main_exchange_decrypt (_arguments) .else_wrap (0x46af8dea)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["password"], _) => {
			_arguments.insert (0, String::from ("z-tokens exchange password"));
			main_exchange_password (_arguments) .else_wrap (0x7dd79a95)
		}
		(&["armor"], _) => {
			_arguments.insert (0, String::from ("z-exchange armor"));
			main_exchange_armor (_arguments) .else_wrap (0x82a1222e)
		}
		(&["dearmor"], _) => {
			_arguments.insert (0, String::from ("z-exchange dearmor"));
			main_exchange_dearmor (_arguments) .else_wrap (0x1008ba10)
		}
		(&["raw", "encode"], _) => {
			_arguments.insert (0, String::from ("z-exchange raw encode"));
			main_exchange_encode (_arguments) .else_wrap (0x71c2c1b5)
		}
		(&["raw", "decode"], _) => {
			_arguments.insert (0, String::from ("z-exchange raw decode"));
			main_exchange_decode (_arguments) .else_wrap (0xecdd6ca7)
		}
		(&["ssh", "keys"], _) => {
			_arguments.insert (0, String::from ("z-exchange ssh keys"));
			main_exchange_ssh_keys (_arguments) .else_wrap (0x7fff2cbd)
		}
		(&["ssh", "wrap"], _) => {
			_arguments.insert (0, String::from ("z-exchange ssh wrap"));
			main_exchange_ssh_wrap (_arguments) .else_wrap (0xcb42bef7)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}








pub(crate) fn main_unknown (_commands : Vec<String>, _arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		#[ cfg (feature = "zt-embedded-help") ]
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
		
		#[ cfg (feature = "zt-embedded-readme") ]
		(&[], &["--readme"]) |
		(&[], &["--readme-text"]) =>
			print_and_exit (&[README_TEXT], true),
		#[ cfg (feature = "zt-embedded-readme") ]
		(&[], &["--readme-html"]) =>
			print_and_exit (&[README_HTML], true),
		
		#[ cfg (feature = "zt-embedded-sbom") ]
		(&[], &["--sbom"]) |
		(&[], &["--sbom-text"]) =>
			print_and_exit (&[SBOM_TEXT], true),
		#[ cfg (feature = "zt-embedded-sbom") ]
		(&[], &["--sbom-html"]) =>
			print_and_exit (&[SBOM_HTML], true),
		#[ cfg (feature = "zt-embedded-sbom") ]
		(&[], &["--sbom-json"]) =>
			print_and_exit (&[SBOM_JSON], true),
		
		#[ cfg (feature = "zt-embedded-sources") ]
		(&[], &["--sources-md5"]) =>
			dump_and_exit (BUILD_SOURCES_MD5.as_bytes (), true),
		#[ cfg (feature = "zt-embedded-sources") ]
		(&[], &["--sources-cpio"]) =>
			dump_and_exit (BUILD_SOURCES_CPIO_GZ, true),
		
		(&[], _) =>
			print_and_exit (&["[ee] [427cd93b]  expected command and arguments;  see `z-tokens help`;  aborting!", "\n"], false),
		
		_ =>
			print_and_exit (&["[ee] [37d61e27]  invalid command;  see `z-tokens help`;  aborting!", "\n"], false),
	}
}








pub fn main_arguments () -> MainResult<(Vec<String>, Vec<String>)> {
	
	let mut _arguments_os : Vec<_> = env::args_os () .collect ();
	if _arguments_os.is_empty () {
		fail! (0xf28dc498);
	} else {
		_arguments_os.remove (0);
	};
	
	let mut _commands = Vec::with_capacity (4);
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
	
	Ok ((_commands, _arguments))
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
fn print_and_exit (_chunks : &[impl AsRef<str>], _success : bool) -> MainResult<ExitCode> {
	
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


#[ cfg (feature = "zt-embedded-readme") ]
static README_TEXT : &'static str = include_str! ("../embedded/readme/readme.txt");
#[ cfg (feature = "zt-embedded-readme") ]
static README_HTML : &'static str = include_str! ("../embedded/readme/readme.html");


#[ cfg (feature = "zt-embedded-help") ]
static HELP_MAIN_TEXT : &'static str = include_str! ("../embedded/help/main.txt");

#[ cfg (feature = "zt-embedded-help") ]
static HELP_HEADER_TEXT : &'static str = include_str! ("../embedded/help/_header.txt");
#[ cfg (feature = "zt-embedded-help") ]
static HELP_FOOTER_TEXT : &'static str = include_str! ("../embedded/help/_footer.txt");


#[ cfg (feature = "zt-embedded-sbom") ]
static SBOM_TEXT : &'static str = include_str! ("../embedded/sbom/sbom.txt");
#[ cfg (feature = "zt-embedded-sbom") ]
static SBOM_HTML : &'static str = include_str! ("../embedded/sbom/sbom.html");
#[ cfg (feature = "zt-embedded-sbom") ]
static SBOM_JSON : &'static str = include_str! ("../embedded/sbom/sbom.json");


#[ cfg (feature = "zt-embedded-build-meta") ]
static BUILD_VERSION : &'static str = include_str! ("../embedded/build/version.txt");
#[ cfg (feature = "zt-embedded-build-meta") ]
static BUILD_NUMBER : &'static str = include_str! ("../embedded/build/number.txt");
#[ cfg (feature = "zt-embedded-build-meta") ]
static BUILD_TIMESTAMP : &'static str = include_str! ("../embedded/build/timestamp.txt");

#[ cfg (feature = "zt-embedded-sources") ]
static BUILD_SOURCES_HASH : &'static str = include_str! ("../embedded/build/sources.hash");
#[ cfg (feature = "zt-embedded-sources") ]
static BUILD_SOURCES_MD5 : &'static str = include_str! ("../embedded/build/sources.md5");

#[ cfg (feature = "zt-embedded-sources") ]
static BUILD_SOURCES_CPIO_GZ : &'static [u8] = include_bytes! ("../embedded/build/sources.cpio.gz");

#[ cfg (feature = "zt-embedded-build-meta") ]
static BUILD_GIT_HASH : &'static str = if let Some (_value) = ::std::option_env! ("__META__BUILD_GIT_HASH") { _value } else { "{unknown-bgh}" };
#[ cfg (feature = "zt-embedded-build-meta") ]
static BUILD_TARGET_TYPE : &'static str = if let Some (_value) = ::std::option_env! ("__META__BUILD_TARGET_TYPE") { _value } else { "{unknown-btt}" };

