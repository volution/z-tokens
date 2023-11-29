

use ::vrl_preludes::std_plus_extras::*;


use crate::mains::*;
use crate::runtime::*;




pub fn premain_tools () -> MainResult<ExitCode> {
	premain_wrapper (main_tools)
}


#[ cfg (feature = "z-tokens-patterns-tool") ]
pub fn premain_patterns () -> MainResult<ExitCode> {
	premain_wrapper (main_patterns)
}


#[ cfg (feature = "z-tokens-hashes-tool") ]
pub fn premain_hashes () -> MainResult<ExitCode> {
	premain_wrapper (main_hashes)
}


#[ cfg (feature = "z-tokens-encodings-tool") ]
pub fn premain_encodings () -> MainResult<ExitCode> {
	premain_wrapper (main_encodings)
}


#[ cfg (feature = "z-tokens-exchange-tool") ]
pub fn premain_exchange () -> MainResult<ExitCode> {
	premain_wrapper (main_exchange)
}


#[ cfg (feature = "z-tokens-oracles-tool") ]
pub fn premain_oracles () -> MainResult<ExitCode> {
	premain_wrapper (main_oracles)
}


#[ cfg (feature = "z-tokens-secrets-tool") ]
pub fn premain_secrets () -> MainResult<ExitCode> {
	premain_wrapper (main_secrets)
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


