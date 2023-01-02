

use crate::prelude::*;

use crate::tools::*;
use crate::tools_flags::*;




pub fn main (_arguments : Vec<String>) -> MainResult {
	
	
	let mut _randomizer_flags = RandomizerFlags::new () .else_wrap (0x585756f3) ?;
	let mut _output_options = OutputOptions::default ();
	
	let mut _pattern = String::from ("cv-lower:4");
	let mut _token_count = 1 as usize;
	let mut _token_separator = String::from ("\n");
	let mut _group_size = 4 as usize;
	let mut _group_separator : Option<String> = None;
	let mut _compact = None;
	
	{
		let mut _parser = ArgParser::new ();
		
		_parser.refer (&mut _pattern)
				.metavar ("{token-pattern}")
				.add_option (&["-p", "--token-pattern"], ArgStore, "(see the `patterns` command for available identifiers)");
		
		_parser.refer (&mut _token_count)
				.metavar ("{token-count}")
				.add_option (&["-c", "--token-count"], ArgStore, "(generate more than one token)");
		
		_parser.refer (&mut _token_separator)
				.metavar ("{token-separator}")
				.add_option (&["-s", "--token-separator"], ArgStore, "(separator after each token)")
				.add_option (&["-n", "--token-separator-none"], ArgStoreConst (String::new ()), "(no separator after each token)")
				.add_option (&["-z", "--token-separator-null"], ArgStoreConst (String::from ("\0")), "(`\\0` separator after each token)");
		
		_parser.refer (&mut _group_size)
				.metavar ("{group-size}")
				.add_option (&["-g", "--group-size"], ArgStore, "(output tokens in groups)");
		
		_parser.refer (&mut _group_separator)
				.metavar ("{group-separator}")
				.add_option (&["--group-separator"], ArgStoreOption, "(separator between each group)");
		
		_parser.refer (&mut _compact)
				.metavar ("{compact}")
				.add_option (&["-C"], ArgStoreConst (Some (true)), "(compact output, skip optional separators and groups)")
				.add_option (&["--compact"], ArgStoreOption, "");
		
		_parser.refer (&mut _output_options.output_separators_mandatory)
				.add_option (&["--token-skip-mandatory-separators"], ArgStoreConst (false), "(skip token mandatory separators)");
		_parser.refer (&mut _output_options.output_separators_optional)
				.add_option (&["--token-skip-optional-separators"], ArgStoreConst (false), "(skip token optional separators)");
		
		_randomizer_flags.parser (&mut _parser) .else_wrap (0x6d197cc8) ?;
		
		_parser.parse (_arguments, &mut stdout_locked (), &mut stderr_locked ()) .else_replace (0x0f71ad86) ?;
	}
	
	if let Some (_compact) = _compact {
		if _compact {
			_output_options.output_separators_optional = false;
			_group_size = 0;
		}
	}
	
	let mut _randomizer = _randomizer_flags.build () .else_wrap (0x8de520fa) ?;
	let _randomizer = _randomizer.deref_mut ();
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
	let _pattern = if let Some (_pattern) = patterns::get_token_pattern (&_pattern) {
			_pattern
		} else {
			fail! (0x74ca2a5f);
		};
	
	let mut _stream = BufWriter::with_capacity (1024 * 1024, stdout_locked ());
	
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
	}
	
	_stream.into_inner () .else_replace (0x96af9244) ?;
	
	Ok (())
}


