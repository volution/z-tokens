

use ::z_tokens_runtime::preludes::std_plus_extras::*;
use ::z_tokens_runtime::preludes::errors::*;
use ::z_tokens_runtime_flags::*;


use crate::main_specials::*;
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
		main_secrets as main_secrets_tool,
	};








pub fn main_tools () -> MainResult<ExitCode> {
	
	let mut _arguments = main_arguments () .else_wrap (0x0da1ecf8) ?;
	
	
	
	
	if let Some (_command_0) = _arguments.command_0_deref () {
		match _command_0 {
			
			#[ cfg (feature = "z-tokens-patterns-tool") ]
			"zt-patterns" | "z-tokens-patterns" | "z-patterns" =>
				return main_patterns (),
			
			#[ cfg (feature = "z-tokens-hashes-tool") ]
			"zt-hashes" | "z-tokens-hashes" | "z-hashes" =>
				return main_hashes (),
			
			#[ cfg (feature = "z-tokens-encodings-tool") ]
			"zt-encodings" | "z-tokens-encodings" | "z-encodings" =>
				return main_encodings (),
			
			#[ cfg (feature = "z-tokens-exchange-tool") ]
			"zt-exchange" | "z-tokens-exchange" | "z-exchange" =>
				return main_exchange (),
			
			#[ cfg (feature = "z-tokens-oracles-tool") ]
			"zt-oracles" | "z-tokens-oracles" | "z-oracles" =>
				return main_oracles (),
			
			#[ cfg (feature = "z-tokens-secrets-tool") ]
			"zt-secrets" | "z-tokens-secrets" | "z-secrets" =>
				return main_secrets (),
			
			_ =>
				(),
		}
	}
	
	
	
	
	match _arguments.commands_deref_vec () .as_slice () {
		
		
		
		
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		&["patterns"] | &["p"] =>
			return main_patterns_list (_arguments) .else_wrap (0x9093f429),
		
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		&["generate"] =>
			return main_patterns_generate (_arguments) .else_wrap (0x7565abe0),
		
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		&["g"] => {
			_arguments.arguments_prepend_all (["--compact", "true", "--token-count", "1"] .iter () .map (OsStr::new));
			return main_patterns_generate (_arguments) .else_wrap (0x6a8d26ca);
		}
		
		#[ cfg (feature = "z-tokens-patterns-tool") ]
		&["g", _pattern] => {
			_arguments.arguments_prepend (OsString::from (_pattern));
			_arguments.arguments_prepend_all (["--compact", "true", "--token-count", "1", "--token-pattern"] .iter () .map (OsStr::new));
			return main_patterns_generate (_arguments) .else_wrap (0x284c1286);
		}
		
		
		
		
		#[ cfg (feature = "z-tokens-hashes-tool") ]
		&["hash"] |
		&["hashes", "hash"] =>
			return main_hashes_hash (_arguments) .else_wrap (0xff8dcc61),
		
		
		
		
		#[ cfg (feature = "z-tokens-encodings-tool") ]
		&["encodings", "encode"] =>
			return main_encodings_encode (_arguments) .else_wrap (0xdb709271),
		
		#[ cfg (feature = "z-tokens-encodings-tool") ]
		&["encodings", "decode"] =>
			return main_encodings_decode (_arguments) .else_wrap (0x19ef259a),
		
		
		
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "keys"] =>
			return main_exchange_keys (_arguments) .else_wrap (0x0df94b2b),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "encrypt"] =>
			return main_exchange_encrypt (_arguments) .else_wrap (0xef766e05),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "decrypt"] =>
			return main_exchange_decrypt (_arguments) .else_wrap (0xa73d3123),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "password"] =>
			return main_exchange_password (_arguments) .else_wrap (0x07f0d87b),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "armor"] =>
			return main_exchange_armor (_arguments) .else_wrap (0xcc846bd9),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "dearmor"] =>
			return main_exchange_dearmor (_arguments) .else_wrap (0x605c4c42),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "raw", "encode"] =>
			return main_exchange_encode (_arguments) .else_wrap (0x0f6f25f9),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "raw", "decode"] =>
			return main_exchange_decode (_arguments) .else_wrap (0x4ea46e82),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "ssh", "keys"] =>
			return main_exchange_ssh_keys (_arguments) .else_wrap (0xfe84133d),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["exchange", "ssh", "wrap"] =>
			return main_exchange_ssh_wrap (_arguments) .else_wrap (0x3108dc57),
		
		
		
		
		#[ cfg (feature = "z-tokens-oracles-tool") ]
		&["oracles"] =>
			return main_oracles_tool (_arguments) .else_wrap (0xf41acddd),
		
		
		
		
		#[ cfg (feature = "z-tokens-secrets-tool") ]
		&["secrets"] =>
			return main_secrets_tool (_arguments) .else_wrap (0x7f006d5d),
		
		
		
		
		_ =>
			return main_unknown (_arguments),
	}
	
	; // NOP
}








#[ cfg (feature = "z-tokens-patterns-tool") ]
pub fn main_patterns () -> MainResult<ExitCode> {
	
	let mut _arguments = main_arguments () .else_wrap (0xd932a3b8) ?;
	
	match _arguments.commands_deref_vec () .as_slice () {
		
		
		
		
		&["list"] | &["l"] =>
			return main_patterns_list (_arguments) .else_wrap (0xd1a6fe40),
		
		&["generate"] =>
			return main_patterns_generate (_arguments) .else_wrap (0x87484b00),
		
		&["g"] => {
			_arguments.arguments_prepend_all (["--compact", "true", "--token-count", "1"] .iter () .map (OsStr::new));
			return main_patterns_generate (_arguments) .else_wrap (0xba4528d0);
		}
		
		&["g", _pattern] => {
			_arguments.arguments_prepend (OsString::from (_pattern));
			_arguments.arguments_prepend_all (["--compact", "true", "--token-count", "1", "--token-pattern"] .iter () .map (OsStr::new));
			return main_patterns_generate (_arguments) .else_wrap (0x7cdbc803);
		}
		
		
		
		
		_ =>
			return main_unknown (_arguments),
	}
	
	; // NOP
}








#[ cfg (feature = "z-tokens-hashes-tool") ]
pub fn main_hashes () -> MainResult<ExitCode> {
	
	let _arguments = main_arguments () .else_wrap (0x384b7517) ?;
	
	match _arguments.commands_deref_vec () .as_slice () {
		
		
		
		
		&["hash"] =>
			return main_hashes_hash (_arguments) .else_wrap (0xf90b7753),
		
		
		
		
		_ =>
			return main_unknown (_arguments),
	}
	
	; // NOP
}








#[ cfg (feature = "z-tokens-encodings-tool") ]
pub fn main_encodings () -> MainResult<ExitCode> {
	
	let _arguments = main_arguments () .else_wrap (0xa79b72a0) ?;
	
	match _arguments.commands_deref_vec () .as_slice () {
		
		
		
		
		&["encode"] =>
			return main_encodings_encode (_arguments) .else_wrap (0x75298a87),
		
		&["decode"] =>
			return main_encodings_decode (_arguments) .else_wrap (0x8f9ff25b),
		
		
		
		
		_ =>
			return main_unknown (_arguments),
	}
	
	; // NOP
}








#[ cfg (feature = "z-tokens-exchange-tool") ]
pub fn main_exchange () -> MainResult<ExitCode> {
	
	let _arguments = main_arguments () .else_wrap (0x00fc937a) ?;
	
	match _arguments.commands_deref_vec () .as_slice () {
		
		
		
		
		&["keys"] =>
			return main_exchange_keys (_arguments) .else_wrap (0x7685fa9c),
		
		&["encrypt"] =>
			return main_exchange_encrypt (_arguments) .else_wrap (0xadd1e78c),
		
		&["decrypt"] =>
			return main_exchange_decrypt (_arguments) .else_wrap (0x46af8dea),
		
		#[ cfg (feature = "z-tokens-exchange-tool") ]
		&["password"] =>
			return main_exchange_password (_arguments) .else_wrap (0x7dd79a95),
		
		&["armor"] =>
			return main_exchange_armor (_arguments) .else_wrap (0x82a1222e),
		
		&["dearmor"] =>
			return main_exchange_dearmor (_arguments) .else_wrap (0x1008ba10),
		
		&["raw", "encode"] =>
			return main_exchange_encode (_arguments) .else_wrap (0x71c2c1b5),
		
		&["raw", "decode"] =>
			return main_exchange_decode (_arguments) .else_wrap (0xecdd6ca7),
		
		&["ssh", "keys"] =>
			return main_exchange_ssh_keys (_arguments) .else_wrap (0x7fff2cbd),
		
		&["ssh", "wrap"] =>
			return main_exchange_ssh_wrap (_arguments) .else_wrap (0xcb42bef7),
		
		
		
		
		_ =>
			return main_unknown (_arguments),
	}
	
	; // NOP
}








#[ cfg (feature = "z-tokens-oracles-tool") ]
pub fn main_oracles () -> MainResult<ExitCode> {
	
	let _arguments = main_arguments () .else_wrap (0xc9159f26) ?;
	
	match _arguments.commands_deref_vec () .as_slice () {
		
		
		
		
		&[] =>
			return main_oracles_tool (_arguments) .else_wrap (0x65ec678f),
		
		
		
		
		_ =>
			return main_unknown (_arguments),
	}
	
	; // NOP
}








#[ cfg (feature = "z-tokens-secrets-tool") ]
pub fn main_secrets () -> MainResult<ExitCode> {
	
	let _arguments = main_arguments () .else_wrap (0xb8fd5238) ?;
	
	match _arguments.commands_deref_vec () .as_slice () {
		
		
		
		
		&[] =>
			return main_secrets_tool (_arguments) .else_wrap (0x0d14e404),
		
		
		
		
		_ =>
			return main_unknown (_arguments),
	}
	
	; // NOP
}


