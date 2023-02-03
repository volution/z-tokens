

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;


use crate::keys::*;
use crate::crypto::*;
use crate::armor::*;
use crate::io::*;








define_error! (pub MainError, result : MainResult);








pub fn main_keys (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _sender_generate : Option<bool> = None;
	let mut _recipient_generate : Option<bool> = None;
	let mut _secret_generate : Option<bool> = None;
	let mut _pin_generate : Option<bool> = None;
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
		
		_parser.refer (&mut _write_comments)
				.metavar ("{enabled}")
				.add_option (&["-c"], ArgStoreConst (Some (true)), "(output comments)")
				.add_option (&["--comments"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x082760e4) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _any_generate_explicit = _sender_generate.is_some () || _recipient_generate.is_some () || _secret_generate.is_some () || _pin_generate.is_some ();
	let _sender_generate = _sender_generate.unwrap_or (! _any_generate_explicit);
	let _recipient_generate = _recipient_generate.unwrap_or (! _any_generate_explicit);
	let _secret_generate = _secret_generate.unwrap_or (! _any_generate_explicit);
	let _pin_generate = _pin_generate.unwrap_or (! _any_generate_explicit);
	let _write_comments = _write_comments.unwrap_or (true);
	
	let mut _output = BufWriter::with_capacity (STDOUT_BUFFER_SIZE, stdout_locked ());
	
	if _sender_generate {
		
		let (_sender_private, _sender_public) = create_sender_pair () .else_wrap (0xd13990c4) ?;
		
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
	
	if _recipient_generate {
		
		let (_recipient_private, _recipient_public) = create_recipient_pair () .else_wrap (0x32a9769f) ?;
		
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
		
		if execute_parser (_parser, _arguments) .else_wrap (0x8a373e9a) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _sender_private = _sender_private.filter (|_key| ! _key.is_empty ());
	let _sender_private = _sender_private.else_wrap (0x11dddce5) ?;
	
	let _recipient_public = _recipient_public.filter (|_key| ! _key.is_empty ());
	let _recipient_public = _recipient_public.else_wrap (0xd94ce2c5) ?;
	
	let _sender_private = SenderPrivateKey::decode_and_zeroize (_sender_private) .else_wrap (0x750a42c0) ?;
	let _recipient_public = RecipientPublicKey::decode_and_zeroize (_recipient_public) .else_wrap (0x233175e9) ?;
	
	let _secret = _secret.filter (|_secret| ! _secret.is_empty ());
	let _secret = _secret.map (SharedSecret::decode_and_zeroize) .transpose () .else_wrap (0xab68aede) ?;
	let _secret = _secret.as_ref () .map (SharedSecret::as_bytes);
	
	let _pin = _pin.filter (|_pin| ! _pin.is_empty ());
	let _pin = _pin.as_ref () .map (String::as_bytes);
	
	let _decrypted = read_at_most (stdin_locked (), CRYPTO_DECRYPTED_SIZE_MAX) .else_wrap (0xb0e8db93) ?;
	
	let mut _encrypted = Vec::new ();
	encrypt (&_sender_private, &_recipient_public, _secret, _pin, &_decrypted, &mut _encrypted) .else_wrap (0x38d2ce1e) ?;
	
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
		
		if execute_parser (_parser, _arguments) .else_wrap (0xe3a49130) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _recipient_private = _recipient_private.filter (|_key| ! _key.is_empty ());
	let _recipient_private = _recipient_private.else_wrap (0xc9683cf5) ?;
	
	let _sender_public = _sender_public.filter (|_key| ! _key.is_empty ());
	let _sender_public = _sender_public.else_wrap (0xdb9a095f) ?;
	
	let _recipient_private = RecipientPrivateKey::decode_and_zeroize (_recipient_private) .else_wrap (0xd58c9ad4) ?;
	let _sender_public = SenderPublicKey::decode_and_zeroize (_sender_public) .else_wrap (0xbb6f004f) ?;
	
	let _secret = _secret.filter (|_secret| ! _secret.is_empty ());
	let _secret = _secret.map (SharedSecret::decode_and_zeroize) .transpose () .else_wrap (0x07d3b030) ?;
	let _secret = _secret.as_ref () .map (SharedSecret::as_bytes);
	
	let _pin = _pin.filter (|_pin| ! _pin.is_empty ());
	let _pin = _pin.as_ref () .map (String::as_bytes);
	
	let _encrypted = read_at_most (stdin_locked (), CRYPTO_ENCRYPTED_SIZE_MAX) .else_wrap (0xf71cef7e) ?;
	
	let mut _decrypted = Vec::new ();
	decrypt (&_recipient_private, &_sender_public, _secret, _pin, &_encrypted, &mut _decrypted) .else_wrap (0x95273e1d) ?;
	
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


