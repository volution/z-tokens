

use crate::prelude::*;

use crate::tools::*;
use crate::tools_flags::*;




pub fn main (_arguments : Vec<String>) -> MainResult {
	
	
	let mut _output_flags = OutputFlags::new () .else_wrap (0x86244665) ?;
	let mut _randomizer_flags = RandomizerFlags::new () .else_wrap (0x585756f3) ?;
	
	let mut _pattern : Option<String> = None;
	let mut _token_count : Option<usize> = None;
	let mut _token_separator : Option<String> = None;
	let mut _group_size : Option<usize> = None;
	let mut _group_separator : Option<String> = None;
	
	{
		let mut _parser = ArgParser::new ();
		
		_parser.refer (&mut _pattern)
				.metavar ("{token-pattern}")
				.add_option (&["-p", "--token-pattern"], ArgStoreOption, "(see the `patterns` command for available identifiers)");
		
		_parser.refer (&mut _token_count)
				.metavar ("{token-count}")
				.add_option (&["-c", "--token-count"], ArgStoreOption, "(generate more than one token)");
		
		_parser.refer (&mut _token_separator)
				.metavar ("{token-separator}")
				.add_option (&["-s", "--token-separator"], ArgStoreOption, "(separator after each token)")
				.add_option (&["-n", "--token-separator-none"], ArgStoreConst (Some (String::new ())), "(no separator after each token)")
				.add_option (&["-z", "--token-separator-null"], ArgStoreConst (Some (String::from ("\0"))), "(`\\0` separator after each token)");
		
		_parser.refer (&mut _group_size)
				.metavar ("{group-size}")
				.add_option (&["-g", "--group-size"], ArgStoreOption, "(output tokens in groups)");
		
		_parser.refer (&mut _group_separator)
				.metavar ("{group-separator}")
				.add_option (&["--group-separator"], ArgStoreOption, "(separator between each group)");
		
		_output_flags.parser (&mut _parser) .else_wrap (0xc06bf3db) ?;
		_randomizer_flags.parser (&mut _parser) .else_wrap (0x6d197cc8) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0xb77c0d40) ? {
			return Ok (())
		}
	}
	
	if let Some (_compact) = _output_flags.compact {
		if _compact {
			if _group_size.is_none () {
				_group_size = Some (0);
			}
		}
	}
	if let Some (ref _token_separator) = _token_separator {
		if _token_separator.is_empty () {
			if _token_count.is_none () {
				_token_count = Some (1)
			}
		}
	}
	
	let _output_options = _output_flags.build () .else_wrap (0xd749e3b0) ?;
	
	let _pattern = _pattern.unwrap_or (String::from ("cv-lower:4"));
	let _token_count = _token_count.unwrap_or (10);
	let _token_separator = _token_separator.unwrap_or (String::from ("\n"));
	let _group_size = _group_size.unwrap_or (10);
	
	let _pattern = if let Some (_pattern) = patterns::get_token_pattern (&_pattern) {
			_pattern
		} else {
			fail! (0x74ca2a5f);
		};
	
	let mut _randomizer = _randomizer_flags.build () .else_wrap (0x8de520fa) ?;
	let _randomizer = _randomizer.deref_mut ();
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
	_randomizer.reset ();
	
	for _index in 0 .. _token_count {
		
		if (_group_size > 0) && (_index > 0) && ((_index % _group_size) == 0) {
			let _separator = _group_separator.as_ref () .unwrap_or (&_token_separator);
			if ! _separator.is_empty () {
				_stream.write (_separator.as_bytes ()) .else_wrap (0x76565a9f) ?;
			}
		}
		
		let _token = generate_token (&_pattern, _randomizer) .else_wrap (0xf2ccbc70) ?;
		
		output_token (&_token, &mut _stream, &_output_options) .else_wrap (0x9c0fbf4f) ?;
		
		if ! _token_separator.is_empty () {
			_stream.write (_token_separator.as_bytes ()) .else_wrap (0xdd5337ae) ?;
		}
		
		_randomizer.advance ();
	}
	
	_stream.into_inner () .else_replace (0x96af9244) ?;
	
	Ok (())
}


