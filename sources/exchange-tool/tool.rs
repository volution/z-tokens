

use crate::prelude::*;




define_error! (pub MainError, result : MainResult);








pub fn main_keys <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	let mut _sender_count : Option<usize> = None;
	let mut _recipient_count : Option<usize> = None;
	let mut _secret_count : Option<usize> = None;
	let mut _seed_count : Option<usize> = None;
	let mut _ballast_count : Option<usize> = None;
	let mut _pin_count : Option<usize> = None;
	let mut _self_generate : Option<bool> = None;
	let mut _write_comments : Option<bool> = None;
	let mut _write_separators : Option<bool> = None;
	
	{
		let mut _flags = create_flags () .else_wrap (0xd885e228) ?;
		
		let _flag = _flags.define_complex (&mut _sender_count);
		_flag.define_switch ('s', "sender", 1);
		_flag.define_flag_0 ()
				.with_flag ((), "senders")
				.with_placeholder ("count")
				.with_description ("generate sender key pair");
		
		let _flag = _flags.define_complex (&mut _recipient_count);
		_flag.define_switch ('r', "recipient", 1);
		_flag.define_flag_0 ()
				.with_flag ((), "recipients")
				.with_placeholder ("count")
				.with_description ("generate recipient key pair");
		
		let _flag = _flags.define_complex (&mut _secret_count);
		_flag.define_switch ('x', "secret", 1);
		_flag.define_flag_0 ()
				.with_flag ((), "secrets")
				.with_placeholder ("count")
				.with_description ("generate shared secret");
		
		let _flag = _flags.define_complex (&mut _seed_count);
		_flag.define_switch ('e', "seed", 1);
		_flag.define_flag_0 ()
				.with_flag ((), "seeds")
				.with_placeholder ("count")
				.with_description ("generate shared seed");
		
		let _flag = _flags.define_complex (&mut _ballast_count);
		_flag.define_switch ('b', "ballast", 1);
		_flag.define_flag_0 ()
				.with_flag ((), "ballasts")
				.with_placeholder ("count")
				.with_description ("generate shared ballast");
		
		let _flag = _flags.define_complex (&mut _pin_count);
		_flag.define_switch ('p', "pin", 1);
		_flag.define_flag_0 ()
				.with_flag ((), "pins")
				.with_placeholder ("count")
				.with_description ("generate shared PIN");
		
		let _flag = _flags.define_complex (&mut _self_generate);
		_flag.define_switch ('o', (), true);
		_flag.define_flag_0 ()
				.with_flag ((), "self")
				.with_placeholder ("enabled")
				.with_description ("generate one key pair, and encode it both for sending and receiving")
				.with_warning ("CAUTION");
		
		let _flag = _flags.define_complex (&mut _write_comments);
		_flag.define_switch ('q', "no-comments", false);
		_flag.define_flag_0 ()
				.with_flag ((), "comments")
				.with_placeholder ("enabled")
				.with_description ("output comments");
		
		let _flag = _flags.define_complex (&mut _write_separators);
		_flag.define_switch ((), "no-separators", false);
		_flag.define_flag_0 ()
				.with_flag ((), "separators")
				.with_placeholder ("enabled")
				.with_description ("output separators");
		
		if execute_flags (_flags, _arguments) .else_wrap (0x082760e4) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _any_explicit =
			_self_generate.is_some ()
			|| _sender_count.is_some ()
			|| _recipient_count.is_some ()
			|| _secret_count.is_some ()
			|| _seed_count.is_some ()
			|| _ballast_count.is_some ()
			|| _pin_count.is_some ();
	
	let _self_generate = _self_generate.unwrap_or (false);
	let _sender_count = _sender_count.unwrap_or (if ! _any_explicit || _self_generate { 1 } else { 0 });
	let _recipient_count = _recipient_count.unwrap_or (if ! _any_explicit || _self_generate { 1 } else { 0 });
	if _self_generate && (_sender_count != _recipient_count) {
		fail! (0xb3673cb1);
	}
	let _keys_count = if _self_generate { _sender_count } else { _sender_count + _recipient_count };
	
	let _secret_count = _secret_count.unwrap_or (if ! _any_explicit { 1 } else { 0 });
	let _seed_count = _seed_count.unwrap_or (if ! _any_explicit { 1 } else { 0 });
	let _ballast_count = _ballast_count.unwrap_or (if ! _any_explicit { 1 } else { 0 });
	let _pin_count = _pin_count.unwrap_or (if ! _any_explicit { 1 } else { 0 });
	
	let _all_count = _keys_count + _secret_count + _seed_count + _ballast_count + _pin_count;
	if _all_count == 0 {
		fail! (0x3c773ab0);
	}
	
	let _group_count =
			if _self_generate { 1 } else { (if _sender_count > 0 { 1 } else { 0 }) + (if _recipient_count > 0 { 1 } else { 0 }) }
			+ if _secret_count > 0 { 1 } else { 0 }
			+ if _seed_count > 0 { 1 } else { 0 }
			+ if _ballast_count > 0 { 1 } else { 0 }
			+ if _pin_count > 0 { 1 } else { 0 };
	
	let _write_comments = _write_comments.unwrap_or (_group_count > 1);
	let _write_separators = _write_separators.unwrap_or (_write_comments);
	let _write_groups = _write_separators && (_group_count > 1);
	
	let mut _output = Vec::with_capacity (STDOUT_BUFFER_SIZE);
	
	if _write_separators {
		writeln! (&mut _output) .else_wrap (0x8f91ace9) ?;
	}
	if _write_groups {
		writeln! (&mut _output) .else_wrap (0x43513292) ?;
	}
	
	let (_sender_pairs, _recipient_pairs) = if _self_generate {
			
			let mut _sender_pairs = Vec::with_capacity (_sender_count);
			let mut _recipient_pairs = Vec::with_capacity (_sender_count);
			
			for _ in 0 .. _sender_count {
				
				let (_sender_private, _sender_public) = create_sender_pair () .else_wrap (0x82797f52) ?;
				
				let _recipient_private = _sender_private.to_recipient ();
				let _recipient_public = _sender_public.to_recipient ();
				
				_sender_pairs.push ((_sender_private, _sender_public));
				_recipient_pairs.push ((_recipient_private, _recipient_public));
			}
			
			(_sender_pairs, _recipient_pairs)
			
		} else {
			
			let mut _sender_pairs = Vec::with_capacity (_sender_count);
			let mut _recipient_pairs = Vec::with_capacity (_recipient_count);
			
			for _ in 0 .. _sender_count {
				let _sender_pair = create_sender_pair () .else_wrap (0xd13990c4) ?;
				_sender_pairs.push (_sender_pair);
			}
			
			for _ in 0 .. _recipient_count {
				let _recipient_pair = create_recipient_pair () .else_wrap (0x32a9769f) ?;
				_recipient_pairs.push (_recipient_pair);
			}
			
			(_sender_pairs, _recipient_pairs)
		};
	
	for (_index, (_sender_private, _sender_public)) in _sender_pairs.into_iter () .enumerate () {
		
		let _sender_private = _sender_private.encode () .else_wrap (0xa52ca3bc) ?;
		let _sender_public = _sender_public.encode () .else_wrap (0x92094072) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## sender private key ({})", _index + 1) .else_wrap (0x75658618) ?;
		}
		writeln! (&mut _output, "{}", _sender_private.deref ()) .else_wrap (0x91a2fad1) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## sender public key ({})", _index + 1) .else_wrap (0x6cfa2380) ?;
		}
		writeln! (&mut _output, "{}", _sender_public.deref ()) .else_wrap (0xd2699fde) ?;
		
		if _write_separators {
			writeln! (&mut _output) .else_wrap (0xd2b185da) ?;
		}
	}
	
	if _write_groups && (_sender_count >= 1) {
		writeln! (&mut _output) .else_wrap (0xae1f1369) ?;
	}
	
	for (_index, (_recipient_private, _recipient_public)) in _recipient_pairs.into_iter () .enumerate () {
		
		let _recipient_private = _recipient_private.encode () .else_wrap (0x9845b620) ?;
		let _recipient_public = _recipient_public.encode () .else_wrap (0x7262954a) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## recipient private key ({})", _index + 1) .else_wrap (0xad864cff) ?;
		}
		writeln! (&mut _output, "{}", _recipient_private.deref ()) .else_wrap (0x8f499bee) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## recipient public key ({})", _index + 1) .else_wrap (0xc7fa9e1b) ?;
		}
		writeln! (&mut _output, "{}", _recipient_public.deref ()) .else_wrap (0x71da88be) ?;
		
		if _write_separators {
			writeln! (&mut _output) .else_wrap (0xf9be83e3) ?;
		}
	}
	
	if _write_groups && (_recipient_count >= 1) {
		writeln! (&mut _output) .else_wrap (0x68cdf030) ?;
	}
	
	for _index in 0 .. _secret_count {
		
		let _secret = create_shared_secret () .else_wrap (0xf61895cb) ?;
		
		let _secret = _secret.encode () .else_wrap (0x1a9d778c) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## shared secret ({}) (optional)", _index + 1) .else_wrap (0xa95dbf57) ?;
		}
		writeln! (&mut _output, "{}", _secret.deref ()) .else_wrap (0x6c8c9dd9) ?;
		
		if _write_separators {
			writeln! (&mut _output) .else_wrap (0x5cd3e5be) ?;
		}
	}
	
	if _write_groups && (_secret_count >= 1) {
		writeln! (&mut _output) .else_wrap (0xd4a7ca9b) ?;
	}
	
	for _index in 0 .. _seed_count {
		
		let _seed = create_shared_seed () .else_wrap (0xacea5a06) ?;
		
		let _seed = _seed.encode () .else_wrap (0x4041ce91) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## shared seed ({}) (optional)", _index + 1) .else_wrap (0xefc86968) ?;
		}
		writeln! (&mut _output, "{}", _seed.deref ()) .else_wrap (0x499fe0ab) ?;
		
		if _write_separators {
			writeln! (&mut _output) .else_wrap (0x9ebb97fe) ?;
		}
	}
	
	if _write_groups && (_seed_count >= 1) {
		writeln! (&mut _output) .else_wrap (0x6b96f588) ?;
	}
	
	for _index in 0 .. _ballast_count {
		
		let _ballast = create_shared_ballast () .else_wrap (0x19447431) ?;
		
		let _ballast = _ballast.encode () .else_wrap (0xb0ec6fff) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## shared ballast ({}) (optional)", _index + 1) .else_wrap (0xca1ddde6) ?;
		}
		writeln! (&mut _output, "{}", _ballast.deref ()) .else_wrap (0xf4f11e97) ?;
		
		if _write_separators {
			writeln! (&mut _output) .else_wrap (0x5f77d760) ?;
		}
	}
	
	if _write_groups && (_ballast_count >= 1) {
		writeln! (&mut _output) .else_wrap (0x88f90df3) ?;
	}
	
	for _index in 0 .. _pin_count {
		
		let _pin = create_shared_pin () .else_wrap (0xcee02c7f) ?;
		
		if _write_comments {
			writeln! (&mut _output, "## shared pin ({}) (optional)", _index + 1) .else_wrap (0x4ba07df1) ?;
		}
		writeln! (&mut _output, "{}", _pin.deref ()) .else_wrap (0x61fd4511) ?;
		
		if _write_separators {
			writeln! (&mut _output) .else_wrap (0x10c04432) ?;
		}
	}
	
	if _write_groups && (_pin_count >= 1) {
		writeln! (&mut _output) .else_wrap (0x64ce63bd) ?;
	}
	
	write_output (stdout_locked (), _output) .else_wrap (0x49532780) ?;
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_password <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	let mut _flags = PasswordFlags::new ();
	
	let mut _parser = create_flags () .else_wrap (0xf6c152eb) ?;
	_flags.flags (&mut _parser) .else_wrap (0x67d0bd69) ?;
	if execute_flags (_parser, _arguments) .else_wrap (0x87bc5c64) ? {
		return Ok (ExitCode::SUCCESS);
	}
	
	// FIXME:  Fix exclusivity of `stdin` for flags and input!
	let _input = stdin ();
	let _output = stdout_locked ();
	
	main_password_with_flags (_flags, _input, _output)
}


pub(crate) fn main_password_with_flags (_flags : PasswordFlags, _input : impl Read, _output : impl Write) -> MainResult<ExitCode> {
	let _arguments = _flags.arguments () .else_wrap (0x6a2e6e49) ?;
	main_password_with_arguments (_arguments, _input, _output)
}


pub(crate) fn main_password_with_arguments (_arguments : PasswordArguments, _input : impl Read, _output : impl Write) -> MainResult<ExitCode> {
	
	let _senders = _arguments.senders.decode () .else_wrap (0xd5bc0620) ?;
	let _recipients = _arguments.recipients.decode () .else_wrap (0xc20dc7fd) ?;
	let _associated = _arguments.shared.associated.decode () .else_wrap (0x5b75b627) ?;
	let _secrets = _arguments.shared.secrets.decode () .else_wrap (0x80159d9a) ?;
	let _pins = _arguments.shared.pins.decode () .else_wrap (0x12ba09e4) ?;
	let _seeds = _arguments.shared.seeds.decode () .else_wrap (0x4a0b1e18) ?;
	let _ballasts = _arguments.shared.ballasts.decode () .else_wrap (0x7ab43c45) ?;
	let _derivation_loops = _arguments.shared.derivation_loops;
	
	let mut _ssh_wrappers = _arguments.ssh_wrappers.wrappers () .else_wrap (0x3cae7413) ?;
	
	let _password_input = _arguments.inputs.data () .else_wrap (0x5d90db8d) ?;
	
	let mut _password_output = [0; 32];
	
	password (
			_senders.iter (),
			_recipients.iter (),
			_associated.iter (),
			_secrets.iter (),
			_pins.iter (),
			_seeds.iter (),
			_ballasts.iter (),
			_derivation_loops,
			&_password_input,
			&mut _password_output,
			_ssh_wrappers.iter_mut (),
		) .else_wrap (0xbfae6a34) ?;
	
	let _password_output = PasswordOutput::new (_password_output);
	
	let _password_output = _password_output.access_bytes ();
	let mut _password_encoded = String::with_capacity (_password_output.len () * 2 + 1);
	for _password_output_byte in _password_output {
		_password_encoded.write_fmt (format_args! ("{:02x}", _password_output_byte)) .else_wrap (0x7ed2faeb) ?;
	}
	_password_encoded.push ('\n');
	
	write_output (_output, _password_encoded.into_bytes ()) .else_wrap (0x74075ccd) ?;
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_encrypt <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	let mut _flags = EncryptFlags::new ();
	
	let mut _parser = create_flags () .else_wrap (0x547e4207) ?;
	_flags.flags (&mut _parser) .else_wrap (0x93da5c86) ?;
	if execute_flags (_parser, _arguments) .else_wrap (0x2d7d23e5) ? {
		return Ok (ExitCode::SUCCESS);
	}
	
	// FIXME:  Fix exclusivity of `stdin` for flags and input!
	let _input = stdin ();
	let _output = stdout_locked ();
	
	main_encrypt_with_flags (_flags, _input, _output)
}


pub(crate) fn main_encrypt_with_flags (_flags : EncryptFlags, _input : impl Read, _output : impl Write) -> MainResult<ExitCode> {
	let _arguments = _flags.arguments () .else_wrap (0xe512fd1c) ?;
	main_encrypt_with_arguments (_arguments, _input, _output)
}


pub(crate) fn main_encrypt_with_arguments (_arguments : EncryptArguments, _input : impl Read, _output : impl Write) -> MainResult<ExitCode> {
	
	let _senders = _arguments.senders.decode () .else_wrap (0xd0c0a0b3) ?;
	let _recipients = _arguments.recipients.decode () .else_wrap (0x7ddeeef6) ?;
	let _associated = _arguments.shared.associated.decode () .else_wrap (0x44a02372) ?;
	let _secrets = _arguments.shared.secrets.decode () .else_wrap (0x1df8ef79) ?;
	let _pins = _arguments.shared.pins.decode () .else_wrap (0xcc5678f0) ?;
	let _seeds = _arguments.shared.seeds.decode () .else_wrap (0x72e6b8f5) ?;
	let _ballasts = _arguments.shared.ballasts.decode () .else_wrap (0x92846ef7) ?;
	let _derivation_loops = _arguments.shared.derivation_loops;
	let _deterministic = _arguments.deterministic;
	
	let mut _ssh_wrappers = _arguments.ssh_wrappers.wrappers () .else_wrap (0x6849e6bd) ?;
	
	let _decrypted = read_at_most (_input, CRYPTO_DECRYPTED_SIZE_MAX) .else_wrap (0x12c4b741) ?;
	
	let mut _encrypted = Vec::with_capacity (STDOUT_BUFFER_SIZE);
	
	encrypt (
			_senders.iter (),
			_recipients.iter (),
			_associated.iter (),
			_secrets.iter (),
			_pins.iter (),
			_seeds.iter (),
			_ballasts.iter (),
			_derivation_loops,
			&_decrypted,
			&mut _encrypted,
			_ssh_wrappers.iter_mut (),
			_deterministic,
		) .else_wrap (0xa3032303) ?;
	
	write_output (_output, _encrypted) .else_wrap (0x4bee2603) ?;
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_decrypt <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	let mut _flags = DecryptFlags::new ();
	
	let mut _parser = create_flags () .else_wrap (0x48431e88) ?;
	_flags.flags (&mut _parser) .else_wrap (0xe7a4f28c) ?;
	if execute_flags (_parser, _arguments) .else_wrap (0xfc8a8c23) ? {
		return Ok (ExitCode::SUCCESS);
	}
	
	// FIXME:  Fix exclusivity of `stdin` for flags and input!
	let _input = stdin ();
	let _output = stdout_locked ();
	
	main_decrypt_with_flags (_flags, _input, _output)
}


pub(crate) fn main_decrypt_with_flags (_flags : DecryptFlags, _input : impl Read, _output : impl Write) -> MainResult<ExitCode> {
	let _arguments = _flags.arguments () .else_wrap (0xf0d0dcf3) ?;
	main_decrypt_with_arguments (_arguments, _input, _output)
}


pub(crate) fn main_decrypt_with_arguments (_arguments : DecryptArguments, _input : impl Read, _output : impl Write) -> MainResult<ExitCode> {
	
	let _recipients = _arguments.recipients.decode () .else_wrap (0xbcc50e84) ?;
	let _senders = _arguments.senders.decode () .else_wrap (0x3a858679) ?;
	let _associated = _arguments.shared.associated.decode () .else_wrap (0xad23f6ba) ?;
	let _secrets = _arguments.shared.secrets.decode () .else_wrap (0x9f36e262) ?;
	let _pins = _arguments.shared.pins.decode () .else_wrap (0x6a7572cb) ?;
	let _seeds = _arguments.shared.seeds.decode () .else_wrap (0x22c67f58) ?;
	let _ballasts = _arguments.shared.ballasts.decode () .else_wrap (0x11d94d7f) ?;
	let _derivation_loops = _arguments.shared.derivation_loops;
	
	let mut _ssh_wrappers = _arguments.ssh_wrappers.wrappers () .else_wrap (0x6a4b6c2d) ?;
	
	let _encrypted = read_at_most (_input, CRYPTO_ENCRYPTED_SIZE_MAX) .else_wrap (0x70fa9ce3) ?;
	
	let mut _decrypted = Vec::with_capacity (STDOUT_BUFFER_SIZE);
	
	decrypt (
			_recipients.iter (),
			_senders.iter (),
			_associated.iter (),
			_secrets.iter (),
			_pins.iter (),
			_seeds.iter (),
			_ballasts.iter (),
			_derivation_loops,
			&_encrypted,
			&mut _decrypted,
			_ssh_wrappers.iter_mut (),
		) .else_wrap (0x85879b4e) ?;
	
	write_output (_output, _decrypted) .else_wrap (0xf6b34f47) ?;
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_armor <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	{
		let mut _flags = create_flags () .else_wrap (0x9deb1736) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0xa38080cc) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _decoded = read_at_most (stdin_locked (), ARMOR_DECODED_SIZE_MAX) .else_wrap (0xaf8bf415) ?;
	
	let mut _encoded = Vec::new ();
	armor (&_decoded, &mut _encoded) .else_wrap (0x7f3ed3ae) ?;
	
	write_output (stdout_locked (), _encoded) .else_wrap (0x2d673134) ?;
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_dearmor <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	{
		let mut _flags = create_flags () .else_wrap (0xe46fc464) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0x222a3894) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _encoded = read_at_most (stdin_locked (), ARMOR_ENCODED_SIZE_MAX) .else_wrap (0x7657c246) ?;
	
	let mut _decoded = Vec::new ();
	dearmor (&_encoded, &mut _decoded) .else_wrap (0x069245f3) ?;
	
	write_output (stdout_locked (), _decoded) .else_wrap (0x2d7f55d6) ?;
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_encode <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	{
		let mut _flags = create_flags () .else_wrap (0xcb1b3482) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0xad08f353) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _decoded = read_at_most (stdin_locked (), ARMOR_DECODED_SIZE_MAX) .else_wrap (0xba262231) ?;
	
	let _encoded_capacity = encode_capacity_max (_decoded.len ()) .else_wrap (0x7837b91c) ?;
	let mut _encoded = Vec::with_capacity (_encoded_capacity);
	
	encode (&_decoded, &mut _encoded) .else_wrap (0xef489995) ?;
	
	write_output (stdout_locked (), _encoded) .else_wrap (0x9ce3a5fa) ?;
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_decode <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	{
		let mut _flags = create_flags () .else_wrap (0x9235af69) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0xe0737dae) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _encoded = read_at_most (stdin_locked (), ARMOR_ENCODED_SIZE_MAX) .else_wrap (0x7d1f6a2d) ?;
	
	let _decoded_capacity = decode_capacity_max (_encoded.len ()) .else_wrap (0x7fce69ef) ?;
	let mut _decoded = Vec::with_capacity (_decoded_capacity);
	
	decode (&_encoded, &mut _decoded) .else_wrap (0xcbf8a0fd) ?;
	
	write_output (stdout_locked (), _decoded) .else_wrap (0xb35cf6db) ?;
	
	Ok (ExitCode::SUCCESS)
}








pub fn main_ssh_keys <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	{
		let mut _flags = create_flags () .else_wrap (0xc0f96685) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0xf17bd371) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let mut _agent = SshWrapperAgent::connect () .else_wrap (0x4e058c28) ?;
	
	let _keys = _agent.keys () .else_wrap (0x63ecbf4e) ?;
	
	let mut _output = Vec::with_capacity (STDOUT_BUFFER_SIZE);
	
	for _key in _keys.iter () {
		
		let _key_handle = _key.description () .else_wrap (0x77ed1b9e) ?;
		let _key_encoded = _key.encode () .else_wrap (0xe0a1a54a) ?;
		
		writeln! (&mut _output) .else_wrap (0x4d2a1a9f) ?;
		
		writeln! (&mut _output, "## {}", _key_handle.deref ()) .else_wrap (0x2fc50e68) ?;
		writeln! (&mut _output, "{}", _key_encoded.deref ()) .else_wrap (0xeb977277) ?;
		
		writeln! (&mut _output) .else_wrap (0x387d7ec5) ?;
	}
	
	write_output (stdout_locked (), _output) .else_wrap (0xb6789fe6) ?;
	
	Ok (ExitCode::SUCCESS)
}




pub fn main_ssh_wrap <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	let (mut _wrapper, _inputs) = {
		
		let mut _ssh_wrappers = SshWrappersFlags::new ();
		let mut _inputs = InputsFlags::new ();
		let mut _common = CommonFlags::new ();
		
		let mut _flags = create_flags () .else_wrap (0xeead67de) ?;
		
		_ssh_wrappers.flags (&mut _flags) .else_wrap (0x0f0189b0) ?;
		_inputs.flags (&mut _flags) .else_wrap (0x05442fc2) ?;
		_common.flags (&mut _flags) .else_wrap (0xe2f22706) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0x596c8a62) ? {
			return Ok (ExitCode::SUCCESS);
		}
		
		let _common = _common.arguments () .else_wrap (0xa66f4eb3) ?;
		let _ssh_wrappers = _ssh_wrappers.arguments (_common.empty_is_missing) .else_wrap (0xc867ac9a) ?;
		let _inputs = _inputs.arguments (_common.empty_is_missing) .else_wrap (0xc78c58a6) ?;
		
		let _ssh_wrappers = _ssh_wrappers.wrappers () .else_wrap (0xc76d8940) ?;
		let _ssh_wrapper = if _ssh_wrappers.len () != 1 {
				fail! (0x763fbeb8);
			} else {
				_ssh_wrappers.into_iter () .next () .infallible (0x6557ad56)
			};
		
		(_ssh_wrapper, _inputs)
	};
	
	let _wrap_input = _inputs.data () .else_wrap (0x99ffad05) ?;
	
	let mut _wrap_output = [0u8; 32];
	_wrapper.wrap (None, &_wrap_input, &mut _wrap_output) .else_wrap (0xe5926524) ?;
	
	let mut _wrap_buffer = String::with_capacity (_wrap_output.len () * 2 + 1);
	for _wrap_output_byte in _wrap_output {
		_wrap_buffer.write_fmt (format_args! ("{:02x}", _wrap_output_byte)) .else_wrap (0xbae3a03b) ?;
	}
	_wrap_buffer.push ('\n');
	
	write_output (stdout_locked (), _wrap_buffer.into_bytes ()) .else_wrap (0x245ce871) ?;
	
	Ok (ExitCode::SUCCESS)
}


