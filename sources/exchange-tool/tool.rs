

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;
use ::z_tokens_runtime::memory::Rb;


use crate::keys::*;
use crate::crypto::*;
use crate::armor::*;
use crate::ssh::*;
use crate::io::*;








define_error! (pub MainError, result : MainResult);








pub fn main_keys (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _sender_generate : Option<bool> = None;
	let mut _recipient_generate : Option<bool> = None;
	let mut _secret_generate : Option<bool> = None;
	let mut _pin_generate : Option<bool> = None;
	let mut _self_generate : Option<bool> = None;
	let mut _write_comments : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0xd885e228) ?;
		
		_parser.refer (&mut _sender_generate)
				.metavar ("{enabled}")
				.add_option (&["-s"], ArgStoreConst (Some (true)), "(generate sender key pair)")
				.add_option (&["--sender"], ArgStoreOption, "");
		
		_parser.refer (&mut _recipient_generate)
				.metavar ("{enabled}")
				.add_option (&["-r"], ArgStoreConst (Some (true)), "(generate recipient key pair)")
				.add_option (&["--recipient"], ArgStoreOption, "");
		
		_parser.refer (&mut _secret_generate)
				.metavar ("{enabled}")
				.add_option (&["-x"], ArgStoreConst (Some (true)), "(generate shared secret)")
				.add_option (&["--secret"], ArgStoreOption, "");
		
		_parser.refer (&mut _pin_generate)
				.metavar ("{enabled}")
				.add_option (&["-p"], ArgStoreConst (Some (true)), "(generate shared PIN)")
				.add_option (&["--pin"], ArgStoreOption, "");
		
		_parser.refer (&mut _self_generate)
				.metavar ("{enabled}")
				.add_option (&["-o"], ArgStoreConst (Some (true)), "(**CAUTION**, generate one key, and encode it both for sending and receiving)")
				.add_option (&["--self"], ArgStoreOption, "");
		
		_parser.refer (&mut _write_comments)
				.metavar ("{enabled}")
				.add_option (&["-c"], ArgStoreConst (Some (true)), "(output comments)")
				.add_option (&["--comments"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x082760e4) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _any_generate_explicit = _sender_generate.is_some () || _recipient_generate.is_some () || _self_generate.is_some () || _secret_generate.is_some () || _pin_generate.is_some ();
	let _self_generate = _self_generate.unwrap_or (false);
	let _sender_generate = _sender_generate.unwrap_or (! _any_generate_explicit || _self_generate);
	let _recipient_generate = _recipient_generate.unwrap_or (! _any_generate_explicit || _self_generate);
	let _secret_generate = _secret_generate.unwrap_or (! _any_generate_explicit);
	let _pin_generate = _pin_generate.unwrap_or (! _any_generate_explicit);
	let _write_comments = _write_comments.unwrap_or (true);
	
	let mut _output = BufWriter::with_capacity (STDOUT_BUFFER_SIZE, stdout_locked ());
	
	let (_sender_pair, _recipient_pair) = if _self_generate {
			
			let (_sender_private, _sender_public) = create_sender_pair () .else_wrap (0x82797f52) ?;
			
			let _recipient_private = _sender_private.to_recipient ();
			let _recipient_public = _sender_public.to_recipient ();
			
			(
				Some ((_sender_private, _sender_public)),
				Some ((_recipient_private, _recipient_public)),
			)
			
		} else {
			(
				if _sender_generate {
						
						let _sender_pair = create_sender_pair () .else_wrap (0xd13990c4) ?;
						
						Some (_sender_pair)
					} else { None },
				
				if _recipient_generate {
						
						let _recipient_pair = create_recipient_pair () .else_wrap (0x32a9769f) ?;
						
						Some (_recipient_pair)
					} else { None },
			)
		};
	
	if let Some ((_sender_private, _sender_public)) = _sender_pair {
		
		let _sender_private = _sender_private.encode () .else_wrap (0xa52ca3bc) ?;
		let _sender_public = _sender_public.encode () .else_wrap (0x92094072) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## sender private key") .else_wrap (0x75658618) ?;
		}
		writeln! (&mut _output, "{}", _sender_private.deref ()) .else_wrap (0x91a2fad1) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## sender public key") .else_wrap (0x6cfa2380) ?;
		}
		writeln! (&mut _output, "{}", _sender_public.deref ()) .else_wrap (0xd2699fde) ?;
		
		writeln! (&mut _output) .else_wrap (0xd2b185da) ?;
	}
	
	if let Some ((_recipient_private, _recipient_public)) = _recipient_pair {
		
		let _recipient_private = _recipient_private.encode () .else_wrap (0x9845b620) ?;
		let _recipient_public = _recipient_public.encode () .else_wrap (0x7262954a) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## recipient private key") .else_wrap (0xad864cff) ?;
		}
		writeln! (&mut _output, "{}", _recipient_private.deref ()) .else_wrap (0x8f499bee) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## recipient public key") .else_wrap (0xc7fa9e1b) ?;
		}
		writeln! (&mut _output, "{}", _recipient_public.deref ()) .else_wrap (0x71da88be) ?;
		
		writeln! (&mut _output) .else_wrap (0xf9be83e3) ?;
	}
	
	if _secret_generate {
		
		let _secret = create_shared_secret () .else_wrap (0xf61895cb) ?;
		
		let _secret = _secret.encode () .else_wrap (0x1a9d778c) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## shared secret (optional)") .else_wrap (0xa95dbf57) ?;
		}
		writeln! (&mut _output, "{}", _secret.deref ()) .else_wrap (0x6c8c9dd9) ?;
		
		writeln! (&mut _output) .else_wrap (0x5cd3e5be) ?;
	}
	
	if _pin_generate {
		
		let _pin = create_shared_pin () .else_wrap (0xcee02c7f) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## shared pin (optional)") .else_wrap (0x4ba07df1) ?;
		}
		writeln! (&mut _output, "{}", _pin.deref ()) .else_wrap (0x61fd4511) ?;
		
		writeln! (&mut _output) .else_wrap (0x10c04432) ?;
	}
	
	drop (_output.into_inner () .else_replace (0x6c15be3e) ?);
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_encrypt (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _sender_private : Option<String> = None;
	let mut _recipient_public : Option<String> = None;
	let mut _secret : Option<String> = None;
	let mut _pin : Option<String> = None;
	let mut _ssh_wrapper : Option<String> = None;
	let mut _empty_is_missing : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0x93d41b76) ?;
		
		_parser.refer (&mut _sender_private)
				.metavar ("{sender}")
				.add_option (&["-s", "--sender"], ArgStoreOption, "(sender private key)");
		
		_parser.refer (&mut _recipient_public)
				.metavar ("{recipient}")
				.add_option (&["-r", "--recipient"], ArgStoreOption, "(recipient public key)");
		
		_parser.refer (&mut _secret)
				.metavar ("{secret}")
				.add_option (&["-x", "--secret"], ArgStoreOption, "(shared secret, for additional security)");
		
		_parser.refer (&mut _pin)
				.metavar ("{pin}")
				.add_option (&["-p", "--pin"], ArgStoreOption, "(shared PIN, for **WEAK** additional security)");
		
		_parser.refer (&mut _ssh_wrapper)
				.metavar ("{key}")
				.add_option (&["--ssh-wrap"], ArgStoreOption, "(shared SSH agent key handle)");
		
		_parser.refer (&mut _empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x8a373e9a) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _empty_is_missing = _empty_is_missing.unwrap_or (false);
	
	let _sender_private = _sender_private.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _sender_private = _sender_private.map (SenderPrivateKey::decode_and_zeroize) .transpose () .else_wrap (0x750a42c0) ?;
	let _sender_private = _sender_private.as_ref ();
	
	let _recipient_public = _recipient_public.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _recipient_public = _recipient_public.map (RecipientPublicKey::decode_and_zeroize) .transpose () .else_wrap (0x233175e9) ?;
	let _recipient_public = _recipient_public.as_ref ();
	
	let _ssh_wrapper = _ssh_wrapper.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _ssh_wrapper = _ssh_wrapper.map (SshWrapperKey::decode_and_zeroize) .transpose () .else_wrap (0x6d68c3c2) ?;
	let mut _ssh_wrapper = _ssh_wrapper.map (Rb::new) .map (SshWrapper::connect) .transpose () .else_wrap (0xe1f9e4bf) ?;
	let _ssh_wrapper = _ssh_wrapper.as_mut ();
	let _ssh_wrapper : Vec<_> = _ssh_wrapper.into_iter () .collect ();
	
	let _secret = _secret.filter (|_secret| ! (_secret.is_empty () && _empty_is_missing));
	let _secret = _secret.map (SharedSecret::decode_and_zeroize) .transpose () .else_wrap (0xab68aede) ?;
	let _secret = _secret.as_ref () .map (SharedSecret::access_bytes) .map (|_bytes| _bytes.as_slice ());
	let _secret : Vec<_> = _secret.into_iter () .collect ();
	
	let _pin = _pin.filter (|_pin| ! (_pin.is_empty () && _empty_is_missing));
	let _pin = _pin.as_ref () .map (String::as_bytes);
	let _pin : Vec<_> = _pin.into_iter () .collect ();
	
	let _decrypted = read_at_most (stdin_locked (), CRYPTO_DECRYPTED_SIZE_MAX) .else_wrap (0xb0e8db93) ?;
	
	let mut _encrypted = Vec::new ();
	encrypt (_sender_private, _recipient_public, &_secret, &_pin, &_decrypted, &mut _encrypted, _ssh_wrapper) .else_wrap (0x38d2ce1e) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_encrypted) .else_wrap (0x815d15bc) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_decrypt (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _recipient_private : Option<String> = None;
	let mut _sender_public : Option<String> = None;
	let mut _secret : Option<String> = None;
	let mut _pin : Option<String> = None;
	let mut _ssh_wrapper : Option<String> = None;
	let mut _empty_is_missing : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0x608139b1) ?;
		
		_parser.refer (&mut _recipient_private)
				.metavar ("{sender}")
				.add_option (&["-r", "--recipient"], ArgStoreOption, "(recipient private key)");
		
		_parser.refer (&mut _sender_public)
				.metavar ("{recipient}")
				.add_option (&["-s", "--sender"], ArgStoreOption, "(sender public key)");
		
		_parser.refer (&mut _secret)
				.metavar ("{secret}")
				.add_option (&["-x", "--secret"], ArgStoreOption, "(shared secret, for additional security)");
		
		_parser.refer (&mut _pin)
				.metavar ("{pin}")
				.add_option (&["-p", "--pin"], ArgStoreOption, "(shared PIN, for **WEAK** additional security)");
		
		_parser.refer (&mut _ssh_wrapper)
				.metavar ("{key}")
				.add_option (&["--ssh-wrap"], ArgStoreOption, "(shared SSH agent key handle)");
		
		_parser.refer (&mut _empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0xe3a49130) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _empty_is_missing = _empty_is_missing.unwrap_or (false);
	
	let _recipient_private = _recipient_private.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _recipient_private = _recipient_private.map (RecipientPrivateKey::decode_and_zeroize) .transpose () .else_wrap (0xd58c9ad4) ?;
	let _recipient_private = _recipient_private.as_ref ();
	
	let _sender_public = _sender_public.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _sender_public = _sender_public.map (SenderPublicKey::decode_and_zeroize) .transpose () .else_wrap (0xbb6f004f) ?;
	let _sender_public = _sender_public.as_ref ();
	
	let _ssh_wrapper = _ssh_wrapper.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _ssh_wrapper = _ssh_wrapper.map (SshWrapperKey::decode_and_zeroize) .transpose () .else_wrap (0x1fea5617) ?;
	let mut _ssh_wrapper = _ssh_wrapper.map (Rb::new) .map (SshWrapper::connect) .transpose () .else_wrap (0xfeda4c77) ?;
	let _ssh_wrapper = _ssh_wrapper.as_mut ();
	let _ssh_wrapper : Vec<_> = _ssh_wrapper.into_iter () .collect ();
	
	let _secret = _secret.filter (|_secret| ! (_secret.is_empty () && _empty_is_missing));
	let _secret = _secret.map (SharedSecret::decode_and_zeroize) .transpose () .else_wrap (0x07d3b030) ?;
	let _secret = _secret.as_ref () .map (SharedSecret::access_bytes) .map (|_bytes| _bytes.as_slice ());
	let _secret : Vec<_> = _secret.into_iter () .collect ();
	
	let _pin = _pin.filter (|_pin| ! (_pin.is_empty () && _empty_is_missing));
	let _pin = _pin.as_ref () .map (String::as_bytes);
	let _pin : Vec<_> = _pin.into_iter () .collect ();
	
	let _encrypted = read_at_most (stdin_locked (), CRYPTO_ENCRYPTED_SIZE_MAX) .else_wrap (0xf71cef7e) ?;
	
	let mut _decrypted = Vec::new ();
	decrypt (_recipient_private, _sender_public, &_secret, &_pin, &_encrypted, &mut _decrypted, _ssh_wrapper) .else_wrap (0x95273e1d) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_decrypted) .else_wrap (0x19352ca2) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_armor (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	{
		let mut _parser = create_parser () .else_wrap (0x9deb1736) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0xa38080cc) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _decoded = read_at_most (stdin_locked (), ARMOR_DECODED_SIZE_MAX) .else_wrap (0xaf8bf415) ?;
	
	let mut _encoded = Vec::new ();
	armor (&_decoded, &mut _encoded) .else_wrap (0x7f3ed3ae) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_encoded) .else_wrap (0x2d673134) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_dearmor (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	{
		let mut _parser = create_parser () .else_wrap (0xe46fc464) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0x222a3894) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _encoded = read_at_most (stdin_locked (), ARMOR_ENCODED_SIZE_MAX) .else_wrap (0x7657c246) ?;
	
	let mut _decoded = Vec::new ();
	dearmor (&_encoded, &mut _decoded) .else_wrap (0x069245f3) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_decoded) .else_wrap (0x2d7f55d6) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_ssh_keys (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _agent = SshWrapperAgent::connect () .else_wrap (0x4e058c28) ?;
	
	let _keys = _agent.keys () .else_wrap (0x63ecbf4e) ?;
	
	let mut _output = BufWriter::with_capacity (STDOUT_BUFFER_SIZE, stdout_locked ());
	
	for _key in _keys.iter () {
		
		let _key_handle = _key.handle () .else_wrap (0x77ed1b9e) ?;
		let _key_encoded = _key.encode () .else_wrap (0xe0a1a54a) ?;
		
		writeln! (&mut _output) .else_wrap (0x4d2a1a9f) ?;
		
		writeln! (&mut _output, "## {}", _key_handle.deref ()) .else_wrap (0x2fc50e68) ?;
		writeln! (&mut _output, "{}", _key_encoded.deref ()) .else_wrap (0xeb977277) ?;
		
		writeln! (&mut _output) .else_wrap (0x387d7ec5) ?;
	}
	
	drop (_output.into_inner () .else_replace (0xc1441245) ?);
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_ssh_wrap (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _agent = SshWrapperAgent::connect () .else_wrap (0x3031a84a) ?;
	
	let mut _key : Option<String> = None;
	let mut _empty_is_missing : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0xeead67de) ?;
		
		_parser.refer (&mut _key)
				.metavar ("{key}")
				.add_option (&["-k", "--key"], ArgStoreOption, "(shared SSH agent key handle)");
		
		_parser.refer (&mut _empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x596c8a62) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _empty_is_missing = _empty_is_missing.unwrap_or (false);
	
	let _key = _key.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _key = _key.else_wrap (0x76dd6a4e) ?;
	
	let _key = SshWrapperKey::decode_and_zeroize (_key) .else_wrap (0xf183991d) ?;
	let mut _wrapper = SshWrapper::new (Rb::new (_key), _agent) .else_wrap (0x373fe104) ?;
	
	let _wrap_input = read_at_most (stdin_locked (), CRYPTO_DECRYPTED_SIZE_MAX) .else_wrap (0x3be3690b) ?;
	
	let mut _wrap_output = [0u8; 32];
	_wrapper.wrap (&_wrap_input, &mut _wrap_output) .else_wrap (0xe5926524) ?;
	
	let mut _wrap_buffer = String::with_capacity (_wrap_output.len () * 2 + 1);
	for _wrap_output_byte in _wrap_output {
		_wrap_buffer.write_fmt (format_args! ("{:02x}", _wrap_output_byte)) .else_wrap (0xbae3a03b) ?;
	}
	_wrap_buffer.push ('\n');
	
	let mut _stream = stdout_locked ();
	_stream.write (_wrap_buffer.as_bytes ()) .else_wrap (0x245ce871) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}


