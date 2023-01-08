

use crate::prelude::*;

use crate::tools::*;
use crate::tools_flags::*;




const DEFAULT_LENGTH_MAXIMUM : usize = 40;
const DEFAULT_DISPLAY_TRIM : usize = 80;
const DEFAULT_CLASSIFY_TRIES : usize = 16;




pub fn main (_arguments : Vec<String>) -> MainResult<ExitCode> {
	
	
	let mut _output_flags = OutputFlags::new () .else_wrap (0x9b1b7b70) ?;
	let mut _randomizer_flags = RandomizerFlags::new () .else_wrap (0x839efea4) ?;
	
	let mut _identifiers_only : Option<bool> = None;
	let mut _all : Option<bool> = None;
	
	let mut _identifier_prefix : Option<String> = None;
	let mut _identifier_suffix : Option<String> = None;
	let mut _identifier_contains : Option<String> = None;
	
	let mut _entropy_minimum : Option<usize> = None;
	let mut _entropy_maximum : Option<usize> = None;
	let mut _length_minimum : Option<usize> = None;
	let mut _length_maximum : Option<usize> = None;
	
	let mut _has_all : Option<usize> = None;
	let mut _has_letters : Option<usize> = None;
	let mut _has_letters_upper : Option<usize> = None;
	let mut _has_letters_lower : Option<usize> = None;
	let mut _has_digits : Option<usize> = None;
	let mut _has_symbols : Option<usize> = None;
	
	{
		let mut _parser = ArgParser::new ();
		
		_parser.refer (&mut _identifiers_only)
				.add_option (&["-i", "--identifiers-only"], ArgStoreConst (Some (true)), "(list only identifiers)");
		
		_parser.refer (&mut _all)
				.add_option (&["-a", "--all"], ArgStoreConst (Some (true)), "(list all patterns, including aliases)");
		
		_parser.refer (&mut _identifier_prefix)
				.metavar ("{prefix}")
				.add_option (&["--identifier-prefix"], ArgStoreOption, "(filter if identifier has prefix)");
		_parser.refer (&mut _identifier_suffix)
				.metavar ("{suffix}")
				.add_option (&["--identifier-suffix"], ArgStoreOption, "(filter if identifier has suffix)");
		_parser.refer (&mut _identifier_contains)
				.metavar ("{string}")
				.add_option (&["--identifier-contains"], ArgStoreOption, "(filter if identifier contains string)");
		
		_parser.refer (&mut _entropy_minimum)
				.metavar ("{bits}")
				.add_option (&["-b", "--entropy-min"], ArgStoreOption, "(filter by minimum entropy in bits)");
		_parser.refer (&mut _entropy_maximum)
				.metavar ("{bits}")
				.add_option (&["-B", "--entropy-max"], ArgStoreOption, "(filter by maximum entropy in bits)");
		
		_parser.refer (&mut _length_minimum)
				.metavar ("{length}")
				.add_option (&["-s", "--length-min"], ArgStoreOption, "(filter by minimum output length)");
		_parser.refer (&mut _length_maximum)
				.metavar ("{length}")
				.add_option (&["-S", "--length-max"], ArgStoreOption, "(filter by maximum output length)");
		
		_parser.refer (&mut _has_all)
				.metavar ("{count}")
				.add_option (&["-A", "--has-all"], ArgStoreConst (Some (1)), "(require letters, digits and symbols)")
				.add_option (&["--all-min"], ArgStoreOption, "");
		_parser.refer (&mut _has_letters)
				.metavar ("{count}")
				.add_option (&["-l", "--has-letters"], ArgStoreConst (Some (1)), "(require letters)")
				.add_option (&["--letters-min"], ArgStoreOption, "");
		_parser.refer (&mut _has_letters_upper)
				.metavar ("{count}")
				.add_option (&["-U", "--has-letters-upper"], ArgStoreConst (Some (1)), "(require upper letters)")
				.add_option (&["--letters-upper-min"], ArgStoreOption, "");
		_parser.refer (&mut _has_letters_lower)
				.metavar ("{count}")
				.add_option (&["-L", "--has-letters-lower"], ArgStoreConst (Some (1)), "(require lower letters)")
				.add_option (&["--letters-lower-min"], ArgStoreOption, "");
		_parser.refer (&mut _has_digits)
				.metavar ("{count}")
				.add_option (&["-D", "--has-digits"], ArgStoreConst (Some (1)), "(require digits)")
				.add_option (&["--digits-min"], ArgStoreOption, "");
		_parser.refer (&mut _has_symbols)
				.metavar ("{count}")
				.add_option (&["-Y", "--has-symbols"], ArgStoreConst (Some (1)), "(require symbols)")
				.add_option (&["--symbols-min"], ArgStoreOption, "");
		
		_output_flags.parser (&mut _parser) .else_wrap (0x2dbc1e80) ?;
		_randomizer_flags.parser (&mut _parser) .else_wrap (0x7a560f7c) ?;
		
		if execute_parser (_parser, _arguments) .else_wrap (0xf1ae4cdd) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	if (_length_minimum.is_some () || _length_maximum.is_some ()) && _output_flags.compact.is_none () {
		_output_flags.compact = Some (true);
	}
	
	let _output_options = _output_flags.build () .else_wrap (0x3c0d75d6) ?;
	
	
	
	
	let _identifiers_only = _identifiers_only.unwrap_or (false);
	let _all = _all.unwrap_or (false);
	
	let _skip_upper =
			(! _identifiers_only) && (! _all)
			&& _identifier_prefix.is_none ()
			&& _identifier_suffix.is_none ()
			&& _identifier_contains.is_none ()
			&& _has_all.is_none ()
			&& _has_letters.is_none ()
			&& _has_letters_upper.is_none ()
			&& _has_letters_lower.is_none ()
			&& _has_digits.is_none ()
			&& _has_symbols.is_none ();
	
	let _length_maximum = if
				(! _identifiers_only) && (! _all)
				&& _identifier_prefix.is_none ()
				&& _identifier_suffix.is_none ()
				&& _identifier_contains.is_none ()
				&& _entropy_minimum.is_none ()
				&& _entropy_maximum.is_none ()
				&& _length_minimum.is_none ()
				&& _length_minimum.is_none ()
		{
			Some (DEFAULT_LENGTH_MAXIMUM)
		} else {
			_length_maximum
		};
	
	let _entropy_minimum = _entropy_minimum.unwrap_or (0);
	let _entropy_maximum = _entropy_maximum.unwrap_or (usize::MAX);
	let _length_minimum = _length_minimum.unwrap_or (0);
	let _length_maximum = _length_maximum.unwrap_or (usize::MAX);
	
	let _classify_chars =
			_has_all.is_some () ||
			_has_letters.is_some () ||
			_has_letters_upper.is_some () ||
			_has_letters_lower.is_some () ||
			_has_digits.is_some () ||
			_has_symbols.is_some ();
	
	let _display_aliases = _all;
	
	
	
	
	let mut _randomizer = _randomizer_flags.build () .else_wrap (0xa43471c4) ?;
	let _randomizer = _randomizer.deref_mut ();
	
	let mut _stream = BufWriter::with_capacity (IO_BUFFER_SIZE, stdout_locked ());
	
	'_loop : for _pattern in patterns::all_token_patterns () .into_iter () {
		let &(ref _identifier, ref _pattern) = _pattern.as_ref ();
		let _identifier = _identifier.as_ref ();
		let _pattern = _pattern.as_ref ();
		
		let (_aliases, _labels) = if let TokenPattern::Tagged (_, _tags) = _pattern {
				let _aliases = _tags.aliases.as_deref () .unwrap_or (&[]);
				let _labels = _tags.labels.as_deref () .unwrap_or (&[]);
				(_aliases, _labels)
			} else {
				(&[][..], &[][..])
			};
		
		{
			let mut _skip_any = false;
			let mut _matched_any = false;
			for _identifier in Some (Cow::Borrowed (_identifier)) .iter () .cloned () .chain (_aliases.iter () .map (|_alias| _alias.to_string ())) {
				let mut _skip = false;
				let mut _matched = true;
				_skip = _skip || if _skip_upper {
						_identifier.contains ("-upper-") || _identifier.contains ("-upper:")
					} else { false };
				_matched = _matched && if let Some (ref _string) = _identifier_prefix {
						_identifier.starts_with (_string)
					} else { true };
				_matched = _matched && if let Some (ref _string) = _identifier_suffix {
						_identifier.ends_with (_string)
					} else { true };
				_matched = _matched && if let Some (ref _string) = _identifier_contains {
						_identifier.contains (_string)
					} else { true };
				_skip_any = _skip_any || _skip;
				_matched_any = _matched_any || _matched;
				if _skip_any || _matched_any {
					break;
				}
			}
			if _skip_any || ! _matched_any {
				continue '_loop;
			}
		}
		
		_randomizer.reset () .else_wrap (0xb2fb5275) ?;
		
		let _entropy = entropy_token (&_pattern) .else_wrap (0x6374858a) ?;
		let (_bits, _bits_exact) = _entropy.bits_exact ();
		
		if _bits < (_entropy_minimum as f64) {
			continue '_loop;
		}
		if _bits > (_entropy_maximum as f64) {
			continue '_loop;
		}
		
		let _token = generate_token (&_pattern, _randomizer) .else_wrap (0xef0a3430) ?;
		_randomizer.advance () .else_wrap (0x1307c15b) ?;
		let _string = output_token_to_string (&_token, &_output_options) .else_wrap (0x36471fa6) ?;
		let _string_length = _string.len ();
		
		if _string_length < _length_minimum {
			continue '_loop;
		}
		if _string_length > _length_maximum {
			continue '_loop;
		}
		
		if _classify_chars {
			let mut _string = Some (Cow::Borrowed (&_string));
			let mut _matched_any = false;
			for _try in 0 ..= DEFAULT_CLASSIFY_TRIES {
				let mut _letters = 0;
				let mut _letters_upper = 0;
				let mut _letters_lower = 0;
				let mut _digits = 0;
				let mut _symbols = 0;
				let _string = if let Some (_string) = _string.take () {
						_string
					} else {
						let _token = generate_token (&_pattern, _randomizer) .else_wrap (0xa4cd2699) ?;
						_randomizer.advance () .else_wrap (0xbd368c66) ?;
						let _string = output_token_to_string (&_token, &_output_options) .else_wrap (0xde6323af) ?;
						Cow::Owned (_string)
					};
				for _char in _string.chars () {
					match _char {
						'a' ..= 'z' => {
								_letters += 1;
								_letters_lower += 1;
							}
						'A' ..= 'Z' => {
								_letters += 1;
								_letters_upper += 1;
							}
						'0' ..= '9' =>
							_digits += 1,
						'!' ..= '~' =>
							_symbols += 1,
						_ =>
							(),
					}
				}
				let _matched = true
						&& _letters >= usize::max (_has_letters.unwrap_or (0), _has_all.unwrap_or (0))
						&& _letters_upper >= usize::max (_has_letters_upper.unwrap_or (0), _has_all.unwrap_or (0))
						&& _letters_lower >= usize::max (_has_letters_lower.unwrap_or (0), _has_all.unwrap_or (0))
						&& _digits >= usize::max (_has_digits.unwrap_or (0), _has_all.unwrap_or (0))
						&& _symbols >= usize::max (_has_symbols.unwrap_or (0), _has_all.unwrap_or (0));
				_matched_any = _matched_any || _matched;
				if _matched_any {
					break;
				}
			}
			if ! _matched_any {
				continue '_loop;
			}
		}
		
		if _identifiers_only {
			writeln! (&mut _stream, "{}", _identifier) .else_wrap (0xfcdcb2ff) ?;
			if _display_aliases && ! _aliases.is_empty () {
				for _alias in _aliases {
					writeln! (&mut _stream, "{}", _alias.as_ref ()) .else_wrap (0xffe94769) ?;
				}
			}
			continue '_loop;
		}
		
		let _display_string_max = DEFAULT_DISPLAY_TRIM;
		let _display_string = if (_string_length <= _display_string_max) {
				_string
			} else {
				let mut _buffer = String::with_capacity (_display_string_max + 10);
				_buffer.push_str (&_string[0 .. _display_string_max]);
				_buffer.push_str (" [...]");
				_buffer
			};
		
		if _bits_exact {
			writeln! (&mut _stream, "| {:22} | b {:4.0}   | c {:4} ||  {}", _identifier, _bits, _string_length, _display_string) .else_wrap (0x737c2a4f) ?;
		} else {
			let _display_bits = (_bits * 10.0) .floor () / 10.0;
			writeln! (&mut _stream, "| {:22} | b {:6.1} | c {:4} ||  {}", _identifier, _display_bits, _string_length, _display_string) .else_wrap (0xd141c5ef) ?;
		}
		
		if _display_aliases && (! _aliases.is_empty () || ! _labels.is_empty ()) {
			write! (&mut _stream, "\\__ ") .else_wrap (0x4b3973c7) ?;
			if ! _aliases.is_empty () {
				for _alias in _aliases {
					write! (&mut _stream, " {}", _alias.as_ref ()) .else_wrap (0x7275b085) ?;
				}
			} else {
				write! (&mut _stream, " (no aliases)") .else_wrap (0xf0b72d32) ?;
			}
			write! (&mut _stream, " ~~") .else_wrap (0x5d6b9bb3) ?;
			if ! _labels.is_empty () {
				for _label in _labels {
					write! (&mut _stream, " {}", _label.as_ref ()) .else_wrap (0xc3ff5175) ?;
				}
			} else {
				write! (&mut _stream, " (no labels)") .else_wrap (0x4c561520) ?;
			}
			writeln! (&mut _stream) .else_wrap (0x8dfe1e4d) ?;
		}
	}
	
	drop (_stream.into_inner () .else_replace (0xb10d6da8) ?);
	
	Ok (ExitCode::SUCCESS)
}


