

use crate::prelude::*;




const DEFAULT_PATTERN : &str = "cvs-lower:4";
const DEFAULT_TOKEN_COUNT : usize = 10;
const DEFAULT_GROUP_SIZE : usize = 10;




pub fn main_generate <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	
	let mut _output_flags = OutputFlags::new () .else_wrap (0x86244665) ?;
	let mut _randomizer_flags = RandomizerFlags::new () .else_wrap (0x585756f3) ?;
	
	let mut _pattern : Option<String> = None;
	let mut _token_count : Option<usize> = None;
	let mut _token_separator : Option<String> = None;
	let mut _group_size : Option<usize> = None;
	let mut _group_separator : Option<String> = None;
	let mut _output_flush : Option<bool> = None;
	
	let mut _hash_filter : Option<bool> = None;
	let mut _hash_seed : Option<u64> = None;
	let mut _hash_mask_value : Option<u64> = None;
	let mut _hash_mask_bits : Option<u8> = None;
	let mut _hash_expected : Option<u64> = None;
	
	let mut _describe : Option<bool> = None;
	
	{
		let mut _flags = create_flags () .else_wrap (0x69fe2749) ?;
		
		_flags.define_single_flag_0 (&mut _pattern)
				.with_flag ('p', "token-pattern")
				.with_placeholder ("pattern")
				.with_description ("see the `patterns` command for available identifiers");
		
		_flags.define_single_flag_0 (&mut _token_count)
				.with_flag ('c', "token-count")
				.with_placeholder ("count")
				.with_description ("generate more than one token");
		
		let _flag = _flags.define_complex (&mut _token_separator);
		_flag.define_flag_0 ()
				.with_flag ('s', "token-separator")
				.with_placeholder ("separator")
				.with_description ("separator after each token");
		_flag.define_switch_0 (String::new ())
				.with_flag ('n', "token-separator-none")
				.with_description ("no separator after each token");
		_flag.define_switch_0 (String::from ("\0"))
				.with_flag ('z', "token-separator-null")
				.with_description ("`\\0` separator after each token");
		
		_flags.define_single_flag_0 (&mut _group_size)
				.with_flag ('g', "group-size")
				.with_placeholder ("count")
				.with_description ("output tokens in groups");
		
		_flags.define_single_flag_0 (&mut _group_separator)
				.with_flag ((), "group-separator")
				.with_placeholder ("separator")
				.with_description ("separator between each group");
		
		_flags.define_switch_0 (&mut _output_flush)
				.with_flag ((), "output-flush")
				.with_description ("flush output after each token");
		
		_flags.define_switch_0 (&mut _hash_filter)
				.with_flag ((), "hash-filter")
				.with_description ("acceptance hash enabled (currently xxh3-64)")
				.with_description ("UNSTABLE");
		
		_flags.define_single_flag_0 (&mut _hash_seed)
				.with_flag ((), "hash-seed")
				.with_placeholder ("hash-seed")
				.with_description ("acceptance hash seed")
				.with_description ("UNSTABLE");
		
		_flags.define_single_flag_0 (&mut _hash_mask_value)
				.with_flag ((), "hash-mask-value")
				.with_placeholder ("u64")
				.with_description ("acceptance hash mask value")
				.with_description ("UNSTABLE");
		
		_flags.define_single_flag_0 (&mut _hash_mask_bits)
				.with_flag ((), "hash-mask-bits")
				.with_placeholder ("u8")
				.with_description ("acceptance hash mask bits")
				.with_description ("UNSTABLE");
		
		_flags.define_single_flag_0 (&mut _hash_expected)
				.with_flag ((), "hash-expected")
				.with_placeholder ("hash-expected")
				.with_description ("acceptance hash expected")
				.with_description ("UNSTABLE");
		
		_output_flags.flags (&mut _flags) .else_wrap (0xc06bf3db) ?;
		_randomizer_flags.flags (&mut _flags) .else_wrap (0x6d197cc8) ?;
		
		_flags.define_switch_0 (&mut _describe)
				.with_flag ((), "describe")
				.with_description ("describe pattern characteristics");
		
		if execute_flags (_flags, _arguments) .else_wrap (0xb77c0d40) ? {
			return Ok (ExitCode::SUCCESS);
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
	
	let _pattern = _pattern.unwrap_or (String::from (DEFAULT_PATTERN));
	let _token_count = _token_count.unwrap_or (DEFAULT_TOKEN_COUNT);
	let _token_separator = _token_separator.unwrap_or (String::from ("\n"));
	let _group_size = _group_size.unwrap_or (DEFAULT_GROUP_SIZE);
	let _describe = _describe.unwrap_or (false);
	
	let _pattern = if let Some (_pattern) = get_token_pattern (&_pattern) {
			_pattern
		} else {
			fail! (0x74ca2a5f, "pattern not found");
		};
	
	let mut _randomizer = _randomizer_flags.build () .else_wrap (0x8de520fa) ?;
	let _randomizer = _randomizer.deref_mut ();
	
	let mut _hash_filter = _hash_filter.unwrap_or (false) || _hash_seed.is_some () || _hash_mask_value.is_some () || _hash_mask_bits.is_some () || _hash_expected.is_some ();
	let (_hash_seed, _hash_mask, _hash_expected) = if _hash_filter {
			let _hash_seed = _hash_seed.unwrap_or (0);
			let _hash_expected = _hash_expected.unwrap_or (0);
			let _hash_mask = match (_hash_mask_value, _hash_mask_bits) {
					(Some (_mask_value), None) =>
						_mask_value,
					(None, Some (_mask_bits)) =>
						(1 << _mask_bits) - 1,
					(None, None) =>
						0xffff,
					(Some (_), Some (_)) =>
						fail! (0x24df57e8),
				};
			(_hash_seed, _hash_mask, _hash_expected)
		} else {
			Default::default ()
		};
	
	let _output_flush = _output_flush.unwrap_or (_hash_filter);
	
	let mut _stream = BufWriter::with_capacity (IO_BUFFER_SIZE, stdout_locked ());
	
	_randomizer.reset () .else_wrap (0x3e9a73ab) ?;
	
	for _index in 0 .. _token_count {
		
		if (_group_size > 0) && (_index > 0) && ((_index % _group_size) == 0) {
			let _separator = _group_separator.as_ref () .unwrap_or (&_token_separator);
			if ! _separator.is_empty () {
				_stream.write (_separator.as_bytes ()) .else_wrap (0x76565a9f) ?;
				if _output_flush {
					_stream.flush () .else_wrap (0x329a9fe3) ?;
				}
			}
		}
		
		let _token = loop {
			
			let _token = generate_token (&_pattern, _randomizer) .else_wrap (0xf2ccbc70) ?;
			
			if _hash_filter {
				let mut _hasher = xxhash::xxh3::Xxh3::with_seed (_hash_seed);
				output_token_to_hasher (&_token, &mut _hasher, &_output_options) .else_wrap (0x7be75b36) ?;
				let _hash_value = _hasher.digest ();
				let _hash_value = _hash_value & _hash_mask;
				if _hash_value != _hash_expected {
					continue;
				}
			}
			
			break _token;
		};
		
		if _describe && (_index == 0) {
			crate::patterns::pattern_describe (&_pattern, &_token, &_output_options, &mut _stream) ?;
			writeln! (&mut _stream) .else_wrap (0x028cc65f) ?;
			if _output_flush {
				_stream.flush () .else_wrap (0x2659c3ac) ?;
			}
		}
		
		output_token (&_token, &mut _stream, &_output_options) .else_wrap (0x9c0fbf4f) ?;
		
		if ! _token_separator.is_empty () {
			_stream.write (_token_separator.as_bytes ()) .else_wrap (0xdd5337ae) ?;
			if _output_flush {
				_stream.flush () .else_wrap (0x31b962eb) ?;
			}
		}
		
		_randomizer.advance () .else_wrap (0x39297684) ?;
	}
	
	drop (_stream.into_inner () .else_replace (0x96af9244) ?);
	
	Ok (ExitCode::SUCCESS)
}


