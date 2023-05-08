

use ::vrl_preludes::std_plus_extras::*;
use ::vrl_errors::*;
use ::z_tokens_runtime::flags::*;


use ::z_tokens_exchange::keys::*;
use ::z_tokens_exchange::crypto::*;
use ::z_tokens_exchange::armor::*;
use ::z_tokens_exchange::coding::*;
use ::z_tokens_exchange::ssh::*;


use crate::io::*;








define_error! (pub MainError, result : MainResult);








pub fn main_keys (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _sender_generate : Option<bool> = None;
	let mut _recipient_generate : Option<bool> = None;
	let mut _secret_generate : Option<bool> = None;
	let mut _ballast_generate : Option<bool> = None;
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
		
		_parser.refer (&mut _ballast_generate)
				.metavar ("{enabled}")
				.add_option (&["-b"], ArgStoreConst (Some (true)), "(generate shared ballast)")
				.add_option (&["--ballast"], ArgStoreOption, "");
		
		_parser.refer (&mut _pin_generate)
				.metavar ("{enabled}")
				.add_option (&["-p"], ArgStoreConst (Some (true)), "(generate shared PIN)")
				.add_option (&["--pin"], ArgStoreOption, "");
		
		_parser.refer (&mut _self_generate)
				.metavar ("{enabled}")
				.add_option (&["-o"], ArgStoreConst (Some (true)), "generate one key, and encode it both for sending and receiving)  (!!! CAUTION !!!)")
				.add_option (&["--self"], ArgStoreOption, "");
		
		_parser.refer (&mut _write_comments)
				.metavar ("{enabled}")
				.add_option (&["-c"], ArgStoreConst (Some (true)), "(output comments)")
				.add_option (&["--comments"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x082760e4) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _any_generate_explicit = _sender_generate.is_some () || _recipient_generate.is_some () || _self_generate.is_some () || _secret_generate.is_some () || _ballast_generate.is_some () || _pin_generate.is_some ();
	let _self_generate = _self_generate.unwrap_or (false);
	let _sender_generate = _sender_generate.unwrap_or (! _any_generate_explicit || _self_generate);
	let _recipient_generate = _recipient_generate.unwrap_or (! _any_generate_explicit || _self_generate);
	let _secret_generate = _secret_generate.unwrap_or (! _any_generate_explicit);
	let _ballast_generate = _ballast_generate.unwrap_or (! _any_generate_explicit);
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
	
	if _ballast_generate {
		
		let _ballast = create_shared_ballast () .else_wrap (0x19447431) ?;
		
		let _ballast = _ballast.encode () .else_wrap (0xb0ec6fff) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## shared ballast (optional)") .else_wrap (0xca1ddde6) ?;
		}
		writeln! (&mut _output, "{}", _ballast.deref ()) .else_wrap (0xf4f11e97) ?;
		
		writeln! (&mut _output) .else_wrap (0x5f77d760) ?;
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
	
	let mut _senders_private : Vec<String> = Vec::new ();
	let mut _recipients_public : Vec<String> = Vec::new ();
	let mut _associated : Vec<String> = Vec::new ();
	let mut _secrets : Vec<String> = Vec::new ();
	let mut _ballasts : Vec<String> = Vec::new ();
	let mut _pins : Vec<String> = Vec::new ();
	let mut _ssh_wrappers : Vec<String> = Vec::new ();
	let mut _empty_is_missing : Option<bool> = None;
	let mut _deterministic : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0x93d41b76) ?;
		
		_parser.refer (&mut _senders_private)
				.metavar ("{sender}")
				.add_option (&["-s", "--sender"], ArgPush, "(sender private key) (multiple allowed, in any order)");
		
		_parser.refer (&mut _recipients_public)
				.metavar ("{recipient}")
				.add_option (&["-r", "--recipient"], ArgPush, "(recipient public key) (multiple allowed, in any order)");
		
		_parser.refer (&mut _associated)
				.metavar ("{associated}")
				.add_option (&["-a", "--associated"], ArgPush, "(associated data) (multiple allowed, order is important)");
		
		_parser.refer (&mut _secrets)
				.metavar ("{secret}")
				.add_option (&["-x", "--secret"], ArgPush, "(shared secret, for additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _pins)
				.metavar ("{pin}")
				.add_option (&["-p", "--pin"], ArgPush, "(shared PIN, for **WEAK** additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _ssh_wrappers)
				.metavar ("{key}")
				.add_option (&["--ssh-wrap"], ArgPush, "(shared SSH agent key handle) (multiple allowed, in any order)");
		
		_parser.refer (&mut _ballasts)
				.metavar ("{ballast}")
				.add_option (&["-b", "--ballast"], ArgPush, "(shared ballast, for additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified) (!!! CAUTION !!!)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		_parser.refer (&mut _deterministic)
				.metavar ("{bool}")
				.add_option (&["--siv"], ArgStoreConst (Some (true)), "(deterministic output, based on SIV) (!!! CAUTION !!!)");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x8a373e9a) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _empty_is_missing = _empty_is_missing.unwrap_or (false);
	
	let _senders_private = _senders_private.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _senders_private = _senders_private.into_iter () .map (SenderPrivateKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x750a42c0) ?;
	let _senders_private = _senders_private.iter () .collect::<Vec<_>> ();
	
	let _recipients_public = _recipients_public.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _recipients_public = _recipients_public.into_iter () .map (RecipientPublicKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x233175e9) ?;
	let _recipients_public = _recipients_public.iter () .collect::<Vec<_>> ();
	
	let _ssh_wrappers = _ssh_wrappers.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _ssh_wrappers = _ssh_wrappers.into_iter () .map (SshWrapperKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x6d68c3c2) ?;
	let mut _ssh_wrappers = _ssh_wrappers.into_iter () .map (SshWrapper::connect) .collect::<Result<Vec<_>, _>> () .else_wrap (0xe1f9e4bf) ?;
	let _ssh_wrappers = _ssh_wrappers.iter_mut () .collect::<Vec<_>> ();
	
	let _associated = _associated.into_iter () .filter (|_pin| ! (_pin.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _associated = _associated.iter () .map (String::as_bytes) .collect::<Vec<_>> ();
	
	let _secrets = _secrets.into_iter () .filter (|_secret| ! (_secret.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _secrets = _secrets.into_iter () .map (SharedSecret::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0xab68aede) ?;
	let _secrets = _secrets.iter () .map (SharedSecret::access_bytes) .map (|_bytes| _bytes.as_slice ()) .collect::<Vec<_>> ();
	
	let _ballasts = _ballasts.into_iter () .filter (|_ballast| ! (_ballast.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _ballasts = _ballasts.into_iter () .map (SharedBallast::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x125f8f44) ?;
	let _ballasts = _ballasts.iter () .map (SharedBallast::access_bytes) .map (|_bytes| _bytes.as_slice ()) .collect::<Vec<_>> ();
	
	let _pins = _pins.into_iter () .filter (|_pin| ! (_pin.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _pins = _pins.iter () .map (String::as_bytes) .collect::<Vec<_>> ();
	
	let _deterministic = _deterministic.unwrap_or (false);
	
	let _decrypted = read_at_most (stdin_locked (), CRYPTO_DECRYPTED_SIZE_MAX) .else_wrap (0xb0e8db93) ?;
	
	let mut _encrypted = Vec::new ();
	encrypt (&_senders_private, &_recipients_public, &_associated, &_secrets, &_pins, &_ballasts, &_decrypted, &mut _encrypted, _ssh_wrappers, _deterministic) .else_wrap (0x38d2ce1e) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_encrypted) .else_wrap (0x815d15bc) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_decrypt (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _recipients_private : Vec<String> = Vec::new ();
	let mut _senders_public : Vec<String> = Vec::new ();
	let mut _associated : Vec<String> = Vec::new ();
	let mut _secrets : Vec<String> = Vec::new ();
	let mut _ballasts : Vec<String> = Vec::new ();
	let mut _pins : Vec<String> = Vec::new ();
	let mut _ssh_wrappers : Vec<String> = Vec::new ();
	let mut _empty_is_missing : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0x608139b1) ?;
		
		_parser.refer (&mut _recipients_private)
				.metavar ("{sender}")
				.add_option (&["-r", "--recipient"], ArgPush, "(recipient private key) (multiple allowed, in any order)");
		
		_parser.refer (&mut _senders_public)
				.metavar ("{recipient}")
				.add_option (&["-s", "--sender"], ArgPush, "(sender public key) (multiple allowed, in any order)");
		
		_parser.refer (&mut _associated)
				.metavar ("{associated}")
				.add_option (&["-a", "--associated"], ArgPush, "(associated data) (multiple allowed, order is important)");
		
		_parser.refer (&mut _secrets)
				.metavar ("{secret}")
				.add_option (&["-x", "--secret"], ArgPush, "(shared secret, for additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _pins)
				.metavar ("{pin}")
				.add_option (&["-p", "--pin"], ArgPush, "(shared PIN, for **WEAK** additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _ssh_wrappers)
				.metavar ("{key}")
				.add_option (&["--ssh-wrap"], ArgPush, "(shared SSH agent key handle) (multiple allowed, in any order)");
		
		_parser.refer (&mut _ballasts)
				.metavar ("{ballast}")
				.add_option (&["-b", "--ballast"], ArgPush, "(shared ballast, for additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified) (!!! CAUTION !!!)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0xe3a49130) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _empty_is_missing = _empty_is_missing.unwrap_or (false);
	
	let _recipients_private = _recipients_private.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _recipients_private = _recipients_private.into_iter () .map (RecipientPrivateKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0xd58c9ad4) ?;
	let _recipients_private = _recipients_private.iter () .collect::<Vec<_>> ();
	
	let _senders_public = _senders_public.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _senders_public = _senders_public.into_iter () .map (SenderPublicKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0xbb6f004f) ?;
	let _senders_public = _senders_public.iter () .collect::<Vec<_>> ();
	
	let _ssh_wrappers = _ssh_wrappers.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _ssh_wrappers = _ssh_wrappers.into_iter () .map (SshWrapperKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x1fea5617) ?;
	let mut _ssh_wrappers = _ssh_wrappers.into_iter () .map (SshWrapper::connect) .collect::<Result<Vec<_>, _>> () .else_wrap (0xfeda4c77) ?;
	let _ssh_wrappers = _ssh_wrappers.iter_mut () .collect::<Vec<_>> ();
	
	let _associated = _associated.into_iter () .filter (|_pin| ! (_pin.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _associated = _associated.iter () .map (String::as_bytes) .collect::<Vec<_>> ();
	
	let _secrets = _secrets.into_iter () .filter (|_ballast| ! (_ballast.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _secrets = _secrets.into_iter () .map (SharedSecret::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x07d3b030) ?;
	let _secrets = _secrets.iter () .map (SharedSecret::access_bytes) .map (|_bytes| _bytes.as_slice ()) .collect::<Vec<_>> ();
	
	let _ballasts = _ballasts.into_iter () .filter (|_ballast| ! (_ballast.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _ballasts = _ballasts.into_iter () .map (SharedBallast::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0xb0bc927a) ?;
	let _ballasts = _ballasts.iter () .map (SharedBallast::access_bytes) .map (|_bytes| _bytes.as_slice ()) .collect::<Vec<_>> ();
	
	let _pins = _pins.into_iter () .filter (|_pin| ! (_pin.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _pins = _pins.iter () .map (String::as_bytes) .collect::<Vec<_>> ();
	
	let _encrypted = read_at_most (stdin_locked (), CRYPTO_ENCRYPTED_SIZE_MAX) .else_wrap (0xf71cef7e) ?;
	
	let mut _decrypted = Vec::new ();
	decrypt (&_recipients_private, &_senders_public, &_associated, &_secrets, &_pins, &_ballasts, &_encrypted, &mut _decrypted, _ssh_wrappers) .else_wrap (0x95273e1d) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_decrypted) .else_wrap (0x19352ca2) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_password (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	let mut _senders_private : Vec<String> = Vec::new ();
	let mut _recipients_public : Vec<String> = Vec::new ();
	let mut _associated : Vec<String> = Vec::new ();
	let mut _secrets : Vec<String> = Vec::new ();
	let mut _ballasts : Vec<String> = Vec::new ();
	let mut _pins : Vec<String> = Vec::new ();
	let mut _ssh_wrappers : Vec<String> = Vec::new ();
	let mut _empty_is_missing : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0xb2fd613d) ?;
		
		_parser.refer (&mut _senders_private)
				.metavar ("{sender}")
				.add_option (&["-s", "--sender"], ArgPush, "(sender private key) (multiple allowed, in any order)");
		
		_parser.refer (&mut _recipients_public)
				.metavar ("{recipient}")
				.add_option (&["-r", "--recipient"], ArgPush, "(recipient public key) (multiple allowed, in any order)");
		
		_parser.refer (&mut _associated)
				.metavar ("{associated}")
				.add_option (&["-a", "--associated"], ArgPush, "(associated data) (multiple allowed, order is important)");
		
		_parser.refer (&mut _secrets)
				.metavar ("{secret}")
				.add_option (&["-x", "--secret"], ArgPush, "(shared secret, for additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _pins)
				.metavar ("{pin}")
				.add_option (&["-p", "--pin"], ArgPush, "(shared PIN, for **WEAK** additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _ssh_wrappers)
				.metavar ("{key}")
				.add_option (&["--ssh-wrap"], ArgPush, "(shared SSH agent key handle) (multiple allowed, in any order)");
		
		_parser.refer (&mut _ballasts)
				.metavar ("{ballast}")
				.add_option (&["-b", "--ballast"], ArgPush, "(shared ballast, for additional security) (multiple allowed, in any order)");
		
		_parser.refer (&mut _empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified) (!!! CAUTION !!!)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0xd3606aac) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _empty_is_missing = _empty_is_missing.unwrap_or (false);
	
	let _senders_private = _senders_private.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _senders_private = _senders_private.into_iter () .map (SenderPrivateKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0xa3edd671) ?;
	let _senders_private = _senders_private.iter () .collect::<Vec<_>> ();
	
	let _recipients_public = _recipients_public.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _recipients_public = _recipients_public.into_iter () .map (RecipientPublicKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x9c5d68bf) ?;
	let _recipients_public = _recipients_public.iter () .collect::<Vec<_>> ();
	
	let _ssh_wrappers = _ssh_wrappers.into_iter () .filter (|_key| ! (_key.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _ssh_wrappers = _ssh_wrappers.into_iter () .map (SshWrapperKey::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x535691ae) ?;
	let mut _ssh_wrappers = _ssh_wrappers.into_iter () .map (SshWrapper::connect) .collect::<Result<Vec<_>, _>> () .else_wrap (0x2c549ad6) ?;
	let _ssh_wrappers = _ssh_wrappers.iter_mut () .collect::<Vec<_>> ();
	
	let _associated = _associated.into_iter () .filter (|_pin| ! (_pin.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _associated = _associated.iter () .map (String::as_bytes) .collect::<Vec<_>> ();
	
	let _secrets = _secrets.into_iter () .filter (|_secret| ! (_secret.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _secrets = _secrets.into_iter () .map (SharedSecret::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0x20de63fc) ?;
	let _secrets = _secrets.iter () .map (SharedSecret::access_bytes) .map (|_bytes| _bytes.as_slice ()) .collect::<Vec<_>> ();
	
	let _ballasts = _ballasts.into_iter () .filter (|_secret| ! (_secret.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _ballasts = _ballasts.into_iter () .map (SharedBallast::decode_and_zeroize) .collect::<Result<Vec<_>, _>> () .else_wrap (0xbadcd29b) ?;
	let _ballasts = _ballasts.iter () .map (SharedBallast::access_bytes) .map (|_bytes| _bytes.as_slice ()) .collect::<Vec<_>> ();
	
	let _pins = _pins.into_iter () .filter (|_pin| ! (_pin.is_empty () && _empty_is_missing)) .collect::<Vec<_>> ();
	let _pins = _pins.iter () .map (String::as_bytes) .collect::<Vec<_>> ();
	
	let _password_input = read_at_most (stdin_locked (), CRYPTO_DECRYPTED_SIZE_MAX) .else_wrap (0x6772b7e4) ?;
	
	let mut _password_output = [0u8; 32];
	password (&_senders_private, &_recipients_public, &_associated, &_secrets, &_pins, &_ballasts, &_password_input, &mut _password_output, _ssh_wrappers) .else_wrap (0xec55f5c3) ?;
	
	let mut _password_buffer = String::with_capacity (_password_output.len () * 2 + 1);
	for _password_output_byte in _password_output {
		_password_buffer.write_fmt (format_args! ("{:02x}", _password_output_byte)) .else_wrap (0x3c7b44ec) ?;
	}
	_password_buffer.push ('\n');
	
	let mut _stream = stdout_locked ();
	_stream.write (_password_buffer.as_bytes ()) .else_wrap (0x06a66fe0) ?;
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








pub fn main_encode (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	{
		let mut _parser = create_parser () .else_wrap (0xcb1b3482) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0xad08f353) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _decoded = read_at_most (stdin_locked (), ARMOR_DECODED_SIZE_MAX) .else_wrap (0xba262231) ?;
	
	let _encoded_capacity = encode_capacity_max (_decoded.len ()) .else_wrap (0x7837b91c) ?;
	let mut _encoded = Vec::with_capacity (_encoded_capacity);
	
	encode (&_decoded, &mut _encoded) .else_wrap (0xef489995) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_encoded) .else_wrap (0x9ce3a5fa) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_decode (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	{
		let mut _parser = create_parser () .else_wrap (0x9235af69) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0xe0737dae) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _encoded = read_at_most (stdin_locked (), ARMOR_ENCODED_SIZE_MAX) .else_wrap (0x7d1f6a2d) ?;
	
	let _decoded_capacity = decode_capacity_max (_encoded.len ()) .else_wrap (0x7fce69ef) ?;
	let mut _decoded = Vec::with_capacity (_decoded_capacity);
	
	decode (&_encoded, &mut _decoded) .else_wrap (0xcbf8a0fd) ?;
	
	let mut _stream = stdout_locked ();
	_stream.write (&_decoded) .else_wrap (0xb35cf6db) ?;
	mem::drop (_stream);
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_ssh_keys (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	{
		let mut _parser = create_parser () .else_wrap (0xc0f96685) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0xf17bd371) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let mut _agent = SshWrapperAgent::connect () .else_wrap (0x4e058c28) ?;
	
	let _keys = _agent.keys () .else_wrap (0x63ecbf4e) ?;
	
	let mut _output = BufWriter::with_capacity (STDOUT_BUFFER_SIZE, stdout_locked ());
	
	for _key in _keys.iter () {
		
		let _key_handle = _key.description () .else_wrap (0x77ed1b9e) ?;
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
	
	let mut _key : Option<String> = None;
	let mut _empty_is_missing : Option<bool> = None;
	
	{
		let mut _parser = create_parser () .else_wrap (0xeead67de) ?;
		
		_parser.refer (&mut _key)
				.metavar ("{key}")
				.add_option (&["-k", "--key"], ArgStoreOption, "(shared SSH agent key handle)");
		
		_parser.refer (&mut _empty_is_missing)
				.metavar ("{bool}")
				.add_option (&["-M"], ArgStoreConst (Some (true)), "(treat empty arguments as unspecified) (!!! CAUTION !!!)")
				.add_option (&["--empty-is-missing"], ArgStoreOption, "");
		
		if execute_parser (_parser, _arguments) .else_wrap (0x596c8a62) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let mut _agent = SshWrapperAgent::connect () .else_wrap (0x3031a84a) ?;
	
	let _empty_is_missing = _empty_is_missing.unwrap_or (false);
	
	let _key = _key.filter (|_key| ! (_key.is_empty () && _empty_is_missing));
	let _key = _key.else_wrap (0x76dd6a4e) ?;
	
	let _key = SshWrapperKey::decode_and_zeroize (_key) .else_wrap (0xf183991d) ?;
	let mut _wrapper = SshWrapper::new (_key, _agent) .else_wrap (0x373fe104) ?;
	
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


