

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;


use crate::main_specials::*;
use crate::main_helpers::*;
use crate::runtime::*;




#[ cfg (feature = "z-tokens-patterns-tool") ]
use ::z_tokens_patterns_tool::{
		main_list as main_patterns_list,
		main_generate as main_patterns_generate,
	};


#[ cfg (feature = "z-tokens-hashes-tool") ]
use ::z_tokens_hashes_tool::{
		main_hash as main_hashes_hash,
	};


#[ cfg (feature = "z-tokens-encodings-tool") ]
use ::z_tokens_encodings_tool::{
		main_encode as main_encodings_encode,
		main_decode as main_encodings_decode,
	};


#[ cfg (feature = "z-tokens-exchange-tool") ]
use ::z_tokens_exchange_tool::{
		main_keys as main_exchange_keys,
		main_encrypt as main_exchange_encrypt,
		main_decrypt as main_exchange_decrypt,
		main_password as main_exchange_password,
		main_armor as main_exchange_armor,
		main_dearmor as main_exchange_dearmor,
		main_encode as main_exchange_encode,
		main_decode as main_exchange_decode,
		main_ssh_keys as main_exchange_ssh_keys,
		main_ssh_wrap as main_exchange_ssh_wrap,
	};


#[ cfg (feature = "z-tokens-oracles-tool") ]
use ::z_tokens_oracles_tool::{
		main_experiments as main_oracles_tool,
	};


#[ cfg (feature = "z-tokens-secrets-tool") ]
use ::z_tokens_secrets_tool::{
		main_experiments as main_secrets_tool,
	};








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
			main_patterns_list (_arguments) .else_wrap (0x9093f429)
		}
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		(&["generate"], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			main_patterns_generate (_arguments) .else_wrap (0x7565abe0)
		}
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		(&["g"], _) => {
			_arguments.insert (0, String::from ("z-tokens generate"));
			_arguments.insert (1, String::from ("--compact"));
			_arguments.insert (2, String::from ("true"));
			_arguments.insert (3, String::from ("--token-count"));
			_arguments.insert (4, String::from ("1"));
			main_patterns_generate (_arguments) .else_wrap (0x6a8d26ca)
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
			main_patterns_generate (_arguments) .else_wrap (0x284c1286)
		}
		
		
		
		
		#[ cfg (feature = "z-tokens-hashes-tool") ]
		(&["hash"], _) |
		(&["hashes", "hash"], _) => {
			_arguments.insert (0, String::from ("z-tokens hashes hash"));
			main_hashes_hash (_arguments) .else_wrap (0xff8dcc61)
		}
		
		
		
		
		#[ cfg (feature = "z-tokens-encodings-tool") ]
		(&["encodings", "encode"], _) => {
			_arguments.insert (0, String::from ("z-tokens encodings encode"));
			main_encodings_encode (_arguments) .else_wrap (0xdb709271)
		}
		#[ cfg (feature = "z-tokens-encodings-tool") ]
		(&["encodings", "decode"], _) => {
			_arguments.insert (0, String::from ("z-tokens encodings decode"));
			main_encodings_decode (_arguments) .else_wrap (0x19ef259a)
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
		
		
		
		
		#[ cfg (feature = "z-tokens-oracles-tool") ]
		(&["oracles"], _) => {
			_arguments.insert (0, String::from ("z-tokens oracles"));
			main_oracles_tool (_arguments) .else_wrap (0xf41acddd)
		}
		
		
		
		
		#[ cfg (feature = "z-tokens-secrets-tool") ]
		(&["secrets"], _) => {
			_arguments.insert (0, String::from ("z-tokens secrets"));
			main_secrets_tool (_arguments) .else_wrap (0x7f006d5d)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}








#[ cfg (feature = "z-tokens-patterns-tool") ]
pub fn main_patterns () -> MainResult<ExitCode> {
	
	let (mut _commands, mut _arguments) = main_arguments () ?;
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		
		
		
		(&["list"], _) | (&["l"], _) => {
			_arguments.insert (0, String::from ("zt-patterns list"));
			main_patterns_list (_arguments) .else_wrap (0xd1a6fe40)
		}
		
		(&["generate"], _) => {
			_arguments.insert (0, String::from ("zt-patterns generate"));
			main_patterns_generate (_arguments) .else_wrap (0x87484b00)
		}
		
		(&["g"], _) => {
			_arguments.insert (0, String::from ("zt-patterns generate"));
			_arguments.insert (1, String::from ("--compact"));
			_arguments.insert (2, String::from ("true"));
			_arguments.insert (3, String::from ("--token-count"));
			_arguments.insert (4, String::from ("1"));
			main_patterns_generate (_arguments) .else_wrap (0xba4528d0)
		}
		
		(&["g", _pattern], _) => {
			_arguments.insert (0, String::from ("zt-patterns generate"));
			_arguments.insert (1, String::from ("--compact"));
			_arguments.insert (2, String::from ("true"));
			_arguments.insert (3, String::from ("--token-count"));
			_arguments.insert (4, String::from ("1"));
			_arguments.insert (5, String::from ("--token-pattern"));
			_arguments.insert (6, String::from (_pattern));
			main_patterns_generate (_arguments) .else_wrap (0x7cdbc803)
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
		
		
		
		
		(&["hash"], _) => {
			_arguments.insert (0, String::from ("zt-hashes hash"));
			main_hashes_hash (_arguments) .else_wrap (0xf90b7753)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}








#[ cfg (feature = "z-tokens-encodings-tool") ]
pub fn main_encodings () -> MainResult<ExitCode> {
	
	let (mut _commands, mut _arguments) = main_arguments () ?;
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		
		
		
		(&["encode"], _) => {
			_arguments.insert (0, String::from ("zt-encodings encode"));
			main_encodings_encode (_arguments) .else_wrap (0x75298a87)
		}
		(&["decode"], _) => {
			_arguments.insert (0, String::from ("zt-encodings decode"));
			main_encodings_decode (_arguments) .else_wrap (0x8f9ff25b)
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
			_arguments.insert (0, String::from ("zt-exchange keys"));
			main_exchange_keys (_arguments) .else_wrap (0x7685fa9c)
		}
		(&["encrypt"], _) => {
			_arguments.insert (0, String::from ("zt-exchange encrypt"));
			main_exchange_encrypt (_arguments) .else_wrap (0xadd1e78c)
		}
		(&["decrypt"], _) => {
			_arguments.insert (0, String::from ("zt-exchange decrypt"));
			main_exchange_decrypt (_arguments) .else_wrap (0x46af8dea)
		}
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		(&["password"], _) => {
			_arguments.insert (0, String::from ("zt-tokens exchange password"));
			main_exchange_password (_arguments) .else_wrap (0x7dd79a95)
		}
		(&["armor"], _) => {
			_arguments.insert (0, String::from ("zt-exchange armor"));
			main_exchange_armor (_arguments) .else_wrap (0x82a1222e)
		}
		(&["dearmor"], _) => {
			_arguments.insert (0, String::from ("zt-exchange dearmor"));
			main_exchange_dearmor (_arguments) .else_wrap (0x1008ba10)
		}
		(&["raw", "encode"], _) => {
			_arguments.insert (0, String::from ("zt-exchange raw encode"));
			main_exchange_encode (_arguments) .else_wrap (0x71c2c1b5)
		}
		(&["raw", "decode"], _) => {
			_arguments.insert (0, String::from ("zt-exchange raw decode"));
			main_exchange_decode (_arguments) .else_wrap (0xecdd6ca7)
		}
		(&["ssh", "keys"], _) => {
			_arguments.insert (0, String::from ("zt-exchange ssh keys"));
			main_exchange_ssh_keys (_arguments) .else_wrap (0x7fff2cbd)
		}
		(&["ssh", "wrap"], _) => {
			_arguments.insert (0, String::from ("zt-exchange ssh wrap"));
			main_exchange_ssh_wrap (_arguments) .else_wrap (0xcb42bef7)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}








#[ cfg (feature = "z-tokens-oracles-tool") ]
pub fn main_oracles () -> MainResult<ExitCode> {
	
	let (mut _commands, mut _arguments) = main_arguments () ?;
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		
		
		
		(&[], _) => {
			_arguments.insert (0, String::from ("zt-oracles"));
			main_oracles_tool (_arguments) .else_wrap (0x65ec678f)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}








#[ cfg (feature = "z-tokens-secrets-tool") ]
pub fn main_secrets () -> MainResult<ExitCode> {
	
	let (mut _commands, mut _arguments) = main_arguments () ?;
	
	let _commands_refs = _commands.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _commands_refs = _commands_refs.as_slice ();
	let _arguments_refs = _arguments.iter () .map (String::as_str) .collect::<Vec<_>> ();
	let _arguments_refs = _arguments_refs.as_slice ();
	
	match (_commands_refs, _arguments_refs) {
		
		
		
		
		(&[], _) => {
			_arguments.insert (0, String::from ("zt-secrets"));
			main_secrets_tool (_arguments) .else_wrap (0x0d14e404)
		}
		
		
		
		
		_ =>
			main_unknown (_commands, _arguments),
	}
}


