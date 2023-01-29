

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;


use crate::keys::*;




define_error! (pub MainError, result : MainResult);




const STDOUT_BUFFER_SIZE : usize = 8 * 1024;




pub fn main_create_keys (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _sender_generate : Option<bool> = None;
	let mut _receiver_generate : Option<bool> = None;
	let mut _write_comments : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0x0102258d) ?;
		
		_parser.refer (&mut _sender_generate)
				.metavar ("{enabled}")
				.add_option (&["-s"], ArgStoreConst (Some (true)), "(generate sender key pair)")
				.add_option (&["--sender"], ArgStoreOption, "");
		
		_parser.refer (&mut _receiver_generate)
				.metavar ("{enabled}")
				.add_option (&["-r"], ArgStoreConst (Some (true)), "(generate receiver key pair)")
				.add_option (&["--receiver"], ArgStoreOption, "");
		
		_parser.refer (&mut _write_comments)
				.metavar ("{enabled}")
				.add_option (&["-c"], ArgStoreConst (Some (true)), "(output comments)")
				.add_option (&["--comments"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x082760e4) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _any_generate_explicit = _sender_generate.is_some () || _receiver_generate.is_some ();
	let _sender_generate = _sender_generate.unwrap_or (! _any_generate_explicit);
	let _receiver_generate = _receiver_generate.unwrap_or (! _any_generate_explicit);
	let _write_comments = _write_comments.unwrap_or (true);
	
	let mut _output = BufWriter::with_capacity (STDOUT_BUFFER_SIZE, stdout_locked ());
	
	if _sender_generate {
		
		let (_sender_private, _sender_public) = create_sender_pair () .else_wrap (0xd13990c4) ?;
		
		let _sender_private = _sender_private.encode () .else_wrap (0xa52ca3bc) ?;
		let _sender_public = _sender_public.encode () .else_wrap (0x92094072) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## sender private key") .else_wrap (0x6cfa2380) ?;
		}
		writeln! (&mut _output, "{}", _sender_private.deref ()) .else_wrap (0x91a2fad1) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## sender public key") .else_wrap (0x6cfa2380) ?;
		}
		writeln! (&mut _output, "{}", _sender_public.deref ()) .else_wrap (0xd2699fde) ?;
	}
	
	if _receiver_generate {
		
		let (_receiver_private, _receiver_public) = create_receiver_pair () .else_wrap (0x32a9769f) ?;
		
		let _receiver_private = _receiver_private.encode () .else_wrap (0x9845b620) ?;
		let _receiver_public = _receiver_public.encode () .else_wrap (0x7262954a) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## receiver private key") .else_wrap (0xad864cff) ?;
		}
		writeln! (&mut _output, "{}", _receiver_private.deref ()) .else_wrap (0x8f499bee) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## receiver public key") .else_wrap (0xc7fa9e1b) ?;
		}
		writeln! (&mut _output, "{}", _receiver_public.deref ()) .else_wrap (0x71da88be) ?;
	}
	
	drop (_output.into_inner () .else_replace (0x8ab3f5e2) ?);
	
	Ok (ExitCode::SUCCESS)
}


