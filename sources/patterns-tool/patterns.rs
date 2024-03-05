

use crate::prelude::*;




const DEFAULT_LENGTH_MAXIMUM : usize = 40;
const DEFAULT_STRING_MAXIMUM : usize = 40;
const DEFAULT_DISPLAY_TRIM : usize = 80;
const DEFAULT_CLASSIFY_TRIES : usize = 16;




pub fn main_list <'a> (_arguments : Arguments<'a>) -> MainResult<ExitCode> {
	
	
	let mut _output_flags = OutputFlags::new () .else_wrap (0x9b1b7b70) ?;
	let mut _randomizer_flags = RandomizerFlags::new () .else_wrap (0x839efea4) ?;
	
	let mut _select_all : Option<bool> = None;
	let mut _select_shortest : Option<bool> = None;
	let mut _identifiers_only : Option<bool> = None;
	
	let mut _display_all : Option<bool> = None;
	let mut _display_aliases : Option<bool> = None;
	let mut _display_labels : Option<bool> = None;
	let mut _display_characters : Option<bool> = None;
	let mut _display_security : Option<bool> = None;
	let mut _display_bruteforce : Option<bool> = None;
	let mut _display_examples : Option<usize> = None;
	let mut _display_trim : Option<usize> = None;
	
	let mut _identifier_prefix : Option<String> = None;
	let mut _identifier_suffix : Option<String> = None;
	let mut _identifier_contains : Option<String> = None;
	let mut _has_identifier : Option<String> = None;
	let mut _has_label : Option<String> = None;
	
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
	
	let mut _for_cryptography : Option<bool> = None;
	let mut _for_authentication : Option<bool> = None;
	let mut _for_archival_storage : Option<bool> = None;
	let mut _for_long_term_storage : Option<bool> = None;
	let mut _for_short_term_storage : Option<bool> = None;
	
	{
		let mut _flags = create_flags () .else_wrap (0x146c4b38) ?;
		
		_flags.define_switch_0 (&mut _select_all)
				.with_flag ('a', "all")
				.with_description ("select all patterns");
		_flags.define_switch_0 (&mut _select_shortest)
				.with_flag ((), "shortest")
				.with_description ("select shortest patterns");
		
		_flags.define_switch_0 (&mut _identifiers_only)
				.with_flag ('i', "identifiers-only")
				.with_description ("list only identifiers");
		
		_flags.define_switch_0 (&mut _display_all)
				.with_flag ('X', "show-all")
				.with_description ("show all details");
		
		_flags.define_switch_0 (&mut _display_aliases)
				.with_flag ((), "show-aliases")
				.with_description ("show aliases");
		_flags.define_switch_0 (&mut _display_labels)
				.with_flag ((), "show-labels")
				.with_description ("show labels");
		
		_flags.define_switch_0 (&mut _display_characters)
				.with_flag ((), "show-chars")
				.with_description ("show characters count");
		
		_flags.define_switch_0 (&mut _display_security)
				.with_flag ((), "show-security")
				.with_description ("show security guess-timates");
		_flags.define_switch_0 (&mut _display_bruteforce)
				.with_flag ((), "show-bruteforce")
				.with_description ("show bruteforce guess-timates");
		
		_flags.define_single_flag_0 (&mut _display_examples)
				.with_flag ('e', "show-examples")
				.with_placeholder ("count")
				.with_description ("show these many examples");
		
		_flags.define_single_flag_0 (&mut _display_trim)
				.with_flag ((), "trim-examples")
				.with_placeholder ("length")
				.with_description ("show these many characters for each example");
		
		_flags.define_single_flag_0 (&mut _identifier_prefix)
				.with_flag ((), "identifier-prefix")
				.with_placeholder ("prefix")
				.with_description ("filter if identifier has prefix");
		_flags.define_single_flag_0 (&mut _identifier_suffix)
				.with_flag ((), "identifier-suffix")
				.with_placeholder ("suffix")
				.with_description ("filter if identifier has suffix");
		_flags.define_single_flag_0 (&mut _identifier_contains)
				.with_flag ((), "identifier-contains")
				.with_placeholder ("string")
				.with_description ("filter if identifier contains string");
		
		_flags.define_single_flag_0 (&mut _has_identifier)
				.with_flag ('p', "identifier")
				.with_description ("filter by identifier");
		_flags.define_single_flag_0 (&mut _has_label)
				.with_flag ('f', "label")
				.with_description ("filter by label");
		
		_flags.define_single_flag_0 (&mut _entropy_minimum)
				.with_flag ('b', "entropy-min")
				.with_placeholder ("bits")
				.with_description ("filter by minimum entropy in bits");
		_flags.define_single_flag_0 (&mut _entropy_maximum)
				.with_flag ('B', "entropy-max")
				.with_placeholder ("bits")
				.with_description ("filter by maximum entropy in bits");
		
		_flags.define_single_flag_0 (&mut _length_minimum)
				.with_flag ('s', "length-min")
				.with_placeholder ("length")
				.with_description ("filter by minimum output length");
		_flags.define_single_flag_0 (&mut _length_maximum)
				.with_flag ('S', "length-max")
				.with_placeholder ("length")
				.with_description ("filter by maximum output length");
		
		let _flag = _flags.define_complex (&mut _has_all);
		_flag.define_switch_0 (1)
				.with_flag ('A', "has-all")
				.with_description ("require letters, digits and symbols");
		_flag.define_flag_0 ()
				.with_flag ((), "all-min")
				.with_placeholder ("count");
		let _flag = _flags.define_complex (&mut _has_letters);
		_flag.define_switch_0 (1)
				.with_flag ('l', "has-letters")
				.with_description ("require letters");
		_flag.define_flag_0 ()
				.with_flag ((), "letters-min")
				.with_placeholder ("count");
		let _flag = _flags.define_complex (&mut _has_letters_upper);
		_flag.define_switch_0 (1)
				.with_flag ('U', "has-letters-upper")
				.with_description ("require upper letters");
		_flag.define_flag_0 ()
				.with_flag ((), "letters-upper-min")
				.with_placeholder ("count");
		let _flag = _flags.define_complex (&mut _has_letters_lower);
		_flag.define_switch_0 (1)
				.with_flag ('L', "has-letters-lower")
				.with_description ("require lower letters");
		_flag.define_flag_0 ()
				.with_flag ((), "letters-lower-min")
				.with_placeholder ("count");
		let _flag = _flags.define_complex (&mut _has_digits);
		_flag.define_switch_0 (1)
				.with_flag ('D', "has-digits")
				.with_description ("require digits");
		_flag.define_flag_0 ()
				.with_flag ((), "digits-min")
				.with_placeholder ("count");
		let _flag = _flags.define_complex (&mut _has_symbols);
		_flag.define_switch_0 (1)
				.with_flag ('Y', "has-symbols")
				.with_description ("require symbols");
		_flag.define_flag_0 ()
				.with_flag ((), "symbols-min")
				.with_placeholder ("count");
		
		_flags.define_switch_0 (&mut _for_cryptography)
				.with_flag ((), "for-cryptography")
				.with_description ("filter if usable for cryptography");
		_flags.define_switch_0 (&mut _for_authentication)
				.with_flag ((), "for-authentication")
				.with_description ("filter if usable for authentication");
		_flags.define_switch_0 (&mut _for_archival_storage)
				.with_flag ((), "for-archival-storage")
				.with_description ("filter if usable for archival storage");
		_flags.define_switch_0 (&mut _for_long_term_storage)
				.with_flag ((), "for-long-term-storage")
				.with_description ("filter if usable for long term storage");
		_flags.define_switch_0 (&mut _for_short_term_storage)
				.with_flag ((), "for-short-term-storage")
				.with_description ("filter if usable for short term storage");
		
		_output_flags.flags (&mut _flags) .else_wrap (0x2dbc1e80) ?;
		_randomizer_flags.flags (&mut _flags) .else_wrap (0x7a560f7c) ?;
		
		if execute_flags (_flags, _arguments) .else_wrap (0xf1ae4cdd) ? {
			return Ok (ExitCode::SUCCESS);
		}
	}
	
	let _output_options = _output_flags.build () .else_wrap (0x3c0d75d6) ?;
	
	
	
	
	let _select_all = _select_all.unwrap_or (false) || _has_identifier.is_some ();
	let _select_shortest = _select_shortest.unwrap_or (false);
	let _identifiers_only = _identifiers_only.unwrap_or (false);
	
	let _skip_upper =
			(! _identifiers_only) && (! _select_all)
			&& _identifier_prefix.is_none ()
			&& _identifier_suffix.is_none ()
			&& _identifier_contains.is_none ()
			&& _has_all.is_none ()
			&& _has_letters.is_none ()
			&& _has_letters_upper.is_none ()
			&& _has_letters_lower.is_none ()
			&& _has_digits.is_none ()
			&& _has_symbols.is_none ();
	
	let _for_cryptography = _for_cryptography.unwrap_or (false);
	let _for_authentication = _for_authentication.unwrap_or (false);
	let _for_archival_storage = _for_archival_storage.unwrap_or (false);
	let _for_long_term_storage = _for_long_term_storage.unwrap_or (false);
	let _for_short_term_storage = _for_short_term_storage.unwrap_or (false);
	let _classify_usage = _for_cryptography || _for_authentication || _for_archival_storage || _for_long_term_storage || _for_short_term_storage;
	
	let _display_all = _display_all.unwrap_or (false);
	let _display_aliases = _display_aliases.unwrap_or (false) || _display_all;
	let _display_labels = _display_labels.unwrap_or (false) || _display_all;
	let _display_characters = _display_characters.unwrap_or (false) || _display_all;
	let _display_security = _display_security.unwrap_or (false) || _display_all;
	let _display_bruteforce = _display_bruteforce.unwrap_or (false) || _display_all;
	let _display_examples = _display_examples.unwrap_or (1);
	let _display_trim = _display_trim.unwrap_or (if _display_all { 0 } else { DEFAULT_DISPLAY_TRIM });
	let _display_cards = _display_aliases || _display_labels || _display_characters || _display_security || _display_bruteforce || (_display_examples >= 2);
	
	let _classify_chars =
			_length_minimum.is_some () ||
			_length_maximum.is_some () ||
			_has_all.is_some () ||
			_has_letters.is_some () ||
			_has_letters_upper.is_some () ||
			_has_letters_lower.is_some () ||
			_has_digits.is_some () ||
			_has_symbols.is_some () ||
			_display_characters;
	
	let (_length_maximum, _string_maximum) = if
				(! _identifiers_only) && (! _select_all) && (! _select_shortest)
				&& _identifier_prefix.is_none ()
				&& _identifier_suffix.is_none ()
				&& _identifier_contains.is_none ()
				&& _entropy_minimum.is_none ()
				&& _entropy_maximum.is_none ()
				&& _length_minimum.is_none ()
				&& _length_maximum.is_none ()
				&& (! _classify_usage)
		{
			(Some (DEFAULT_LENGTH_MAXIMUM), Some (DEFAULT_STRING_MAXIMUM))
		} else {
			(_length_maximum, None)
		};
	
	
	
	
	let mut _randomizer = _randomizer_flags.build () .else_wrap (0xa43471c4) ?;
	let _randomizer = _randomizer.deref_mut ();
	
	let mut _stream = BufWriter::with_capacity (IO_BUFFER_SIZE, stdout_locked ());
	
	let mut _selected_count = 0;
	let mut _selected_last : &[Rb<Text>] = &[];
	
	'_loop : for _pattern in all_token_patterns () .into_iter () .map (|_pair| _pair.1.as_ref ()) {
		
		let (_identifier, _aliases, _labels) = if let TokenPattern::Tagged (_, _tags) = _pattern {
				let _identifier = _tags.identifier.as_ref ();
				let _aliases = _tags.aliases.as_deref () .unwrap_or (&[]);
				let _labels = _tags.labels.as_deref () .unwrap_or (&[]);
				(_identifier, _aliases, _labels)
			} else {
				(None, &[][..], &[][..])
			};
		
		let _identifiers_strings =
				Iterator::chain (
						_identifier.into_iter (),
						_aliases.iter ()
					)
					.map (Rb::as_ref)
					.map (Text::to_string)
					.collect::<Vec<_>> ();
		
		{
			let mut _skip_any = false;
			let mut _matched_any = false;
			
			for _identifier in _identifiers_strings.iter () {
				
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
		
		if let Some (ref _has_identifier) = _has_identifier {
			let mut _matched_any = false;
			for _identifier in _identifiers_strings {
				if _identifier.eq (_has_identifier) {
					_matched_any = true;
					break;
				}
			}
			if !_matched_any {
				continue '_loop;
			}
		}
		if let Some (ref _has_label) = _has_label {
			let mut _matched_any = false;
			for _label in _labels {
				if _label.eq (_has_label) {
					_matched_any = true;
					break;
				}
			}
			if !_matched_any {
				continue '_loop;
			}
		}
		
		let _entropy = entropy_token (&_pattern) .else_wrap (0x6374858a) ?;
		let (_bits, _bits_exact) = _entropy.bits_exact ();
		
		if _bits < (_entropy_minimum.unwrap_or (0) as f64) {
			continue '_loop;
		}
		if _bits > (_entropy_maximum.unwrap_or (usize::MAX) as f64) {
			continue '_loop;
		}
		
		_randomizer.reset () .else_wrap (0xb2fb5275) ?;
		let _token = generate_token (&_pattern, _randomizer) .else_wrap (0xef0a3430) ?;
		_randomizer.advance () .else_wrap (0x1307c15b) ?;
		
		let _string = output_token_to_string (&_token, &_output_options) .else_wrap (0x36471fa6) ?;
		let _string_length = _string.len ();
		
		if _string_length > _string_maximum.unwrap_or (usize::MAX) {
			continue '_loop;
		}
		
		let _characters = if _classify_chars {
			let mut _string = Some (Cow::Borrowed (&_string));
			let mut _matched_any = false;
			let mut _characters = None;
			for _try in 0 ..= DEFAULT_CLASSIFY_TRIES {
				let _string = if let Some (_string) = _string.take () {
						_string
					} else {
						let _token = generate_token (&_pattern, _randomizer) .else_wrap (0xa4cd2699) ?;
						_randomizer.advance () .else_wrap (0xbd368c66) ?;
						let _string = output_token_to_string (&_token, &_output_options) .else_wrap (0xde6323af) ?;
						Cow::Owned (_string)
					};
				let _characters_0 = pattern_classify_chars (_string.as_str ());
				let (_letters, _letters_upper, _letters_lower, _digits, _symbols, _no_spaces) = _characters_0;
				let _matched = true
						&& _letters >= usize::max (_has_letters.unwrap_or (0), _has_all.unwrap_or (0))
						&& _letters_upper >= usize::max (_has_letters_upper.unwrap_or (0), _has_all.unwrap_or (0))
						&& _letters_lower >= usize::max (_has_letters_lower.unwrap_or (0), _has_all.unwrap_or (0))
						&& _digits >= usize::max (_has_digits.unwrap_or (0), _has_all.unwrap_or (0))
						&& _symbols >= usize::max (_has_symbols.unwrap_or (0), _has_all.unwrap_or (0))
						&& _no_spaces >= _length_minimum.unwrap_or (0)
						&& _no_spaces <= _length_maximum.unwrap_or (usize::MAX);
				_matched_any = _matched_any || _matched;
				if _matched_any {
					_characters = Some (_characters_0);
					break;
				}
			}
			if ! _matched_any {
				continue '_loop;
			}
			_characters
		} else {
			None
		};
		
		let _estimates = if _classify_usage || _display_security || _display_bruteforce {
			
			let _estimates = entropy_estimates (&_entropy) .else_wrap (0xdcca14bf) ?;
			
			let _not_usable =
					(_for_cryptography && !_estimates.for_cryptography) ||
					(_for_authentication && !_estimates.for_authentication) ||
					(_for_archival_storage && !_estimates.for_archival) ||
					(_for_long_term_storage && !_estimates.for_long_term) ||
					(_for_short_term_storage && !_estimates.for_short_term);
			
			if _not_usable {
				continue '_loop;
			}
			
			Some (_estimates)
			
		} else {
			None
		};
		
		if _select_shortest {
			if _selected_last.len () == _labels.len () {
				let mut _matches = true;
				for (_left, _right) in iter::zip (_selected_last, _labels) {
					if ! str::eq (& _left.to_string (), & _right.to_string ()) {
						_matches = false;
						break;
					}
				}
				if _matches {
					continue '_loop;
				}
			}
		}
		
		_selected_count += 1;
		_selected_last = _labels;
		
		if _identifiers_only {
			if let Some (_identifier) = _identifier {
				writeln! (&mut _stream, "{}", _identifier.as_ref ()) .else_wrap (0xfcdcb2ff) ?;
			}
			if _display_aliases && ! _aliases.is_empty () {
				for _alias in _aliases {
					writeln! (&mut _stream, "{}", _alias.as_ref ()) .else_wrap (0xffe94769) ?;
				}
			}
			continue '_loop;
		}
		
		if ! _display_cards {
			
			let _identifier = if let Some (_identifier) = _identifier {
				_identifier.as_ref () .to_string ()
			} else {
				Cow::Borrowed ("<unknown>")
			};
			
			let _length = _characters.map (|_characters| _characters.5) .unwrap_or (_string_length);
			
			if _bits_exact {
				write! (&mut _stream, "::  {:22}  : {:4.0}  =b : {:4} c ::", _identifier.as_ref (), _bits, _length) .else_wrap (0x737c2a4f) ?;
			} else {
				let _display_bits = (_bits * 10.0) .floor () / 10.0;
				write! (&mut _stream, "::  {:22}  : {:6.1} b : {:4} c ::", _identifier.as_ref (), _display_bits, _length) .else_wrap (0xd141c5ef) ?;
			}
			
			if _display_examples > 0 {
				let _display_string = if (_display_trim == 0) || (_string_length <= _display_trim) {
						Cow::Borrowed (&_string)
					} else {
						let mut _buffer = String::with_capacity (_display_trim + 10);
						_buffer.push_str (&_string[0 .. _display_trim]);
						_buffer.push_str (" [...]");
						Cow::Owned (_buffer)
					};
				write! (&mut _stream, "    {}", _display_string) .else_wrap (0x71418c89) ?;
			}
			
			writeln! (&mut _stream) .else_wrap (0x3da13144) ?;
			
		} else {
			
			writeln! (&mut _stream) .else_wrap (0x28af8876) ?;
			
			pattern_describe_display (
					_pattern, _identifier, _aliases, _labels,
					_bits, _bits_exact, _string_length,
					_characters,
					_estimates.as_ref (),
					_display_aliases, _display_labels, _display_characters, _display_security, _display_bruteforce,
					&mut _stream,
				) ?;
			
			if _display_examples == 1 {
				writeln! (&mut _stream, "\\_  example:  {}", _string) .else_wrap (0x6ada645e) ?;
			} else if _display_examples > 1 {
				_randomizer.reset () .else_wrap (0x3a3ebd10) ?;
				writeln! (&mut _stream, "\\_  examples:") .else_wrap (0x69c29df9) ?;
				for _ in 0 .. _display_examples {
					let _token = generate_token (&_pattern, _randomizer) .else_wrap (0x00930317) ?;
					_randomizer.advance () .else_wrap (0xdd4399fd) ?;
					let _string = output_token_to_string (&_token, &_output_options) .else_wrap (0x6a999483) ?;
					writeln! (&mut _stream, "    \\_        {}", _string) .else_wrap (0xdd133491) ?;
				}
			}
			
			writeln! (&mut _stream) .else_wrap (0xcea13ddc) ?;
		}
	}
	
	drop (_stream.into_inner () .else_replace (0xb10d6da8) ?);
	
	if _selected_count == 0 {
		::std::eprintln! ("[ee] [f92050d5]  no patterns selected!");
		return Ok (ExitCode::FAILURE);
	}
	
	Ok (ExitCode::SUCCESS)
}








fn pattern_describe_display (
		_pattern : &TokenPattern,
		_identifier : Option<&Rb<Text>>,
		_aliases : &[Rb<Text>],
		_labels : &[Rb<Text>],
		_bits : f64,
		_bits_exact : bool,
		_string_length : usize,
		_characters : Option<(usize, usize, usize, usize, usize, usize)>,
		_estimates : Option<&EntropyEstimates>,
		_display_aliases : bool,
		_display_labels : bool,
		_display_characters : bool,
		_display_security : bool,
		_display_bruteforce : bool,
		mut _stream : impl Write,
	) -> MainResult
{
	if let Some (_identifier) = _identifier {
		writeln! (&mut _stream, "**  ~~~~~~~~  {}", _identifier.as_ref ()) .else_wrap (0xc6bd1c82) ?;
	}
	
	if _display_aliases && ! _aliases.is_empty () {
		write! (&mut _stream, "\\_  aliases: ") .else_wrap (0x4b3973c7) ?;
		for _alias in _aliases {
			write! (&mut _stream, " {}", _alias.as_ref ()) .else_wrap (0x7275b085) ?;
		}
		writeln! (&mut _stream) .else_wrap (0x8dfe1e4d) ?;
	}
	if _display_labels && ! _labels.is_empty () {
		write! (&mut _stream, "\\_  labels:  ") .else_wrap (0x4a4b1151) ?;
		for _label in _labels {
			write! (&mut _stream, " {}", _label.as_ref ()) .else_wrap (0xc3ff5175) ?;
		}
		writeln! (&mut _stream) .else_wrap (0x2314c8ec) ?;
	}
	
	if _bits_exact {
		writeln! (&mut _stream, "\\_  bits:     {}  (exact)", _bits) .else_wrap (0x36fc1a4b) ?;
	} else {
		let _display_bits = (_bits * 10000.0) .floor () / 10000.0;
		writeln! (&mut _stream, "\\_  bits:     {:.4}", _display_bits) .else_wrap (0xf2b57c8b) ?;
	}
	
	writeln! (&mut _stream, "\\_  length:   {}  (with spaces)", _string_length) .else_wrap (0x000c5aba) ?;
	if let Some (_no_spaces) = _characters.map (|_characters| _characters.5) {
		writeln! (&mut _stream, "\\_  length:   {}  (without spaces)", _no_spaces) .else_wrap (0x6fec0066) ?;
	}
	
	if _display_characters {
		
		let _characters = _characters.infallible (0x2b128d65);
		let (_letters, _letters_upper, _letters_lower, _digits, _symbols, _no_spaces) = _characters;
		
		writeln! (&mut _stream, "\\_  characters:") .else_wrap (0x288d7222) ?;
		writeln! (&mut _stream, "    \\_  letters:  {}", _letters) .else_wrap (0x67382f89) ?;
		writeln! (&mut _stream, "    \\_  l. upper: {}", _letters_upper) .else_wrap (0x8eafe218) ?;
		writeln! (&mut _stream, "    \\_  l. lower: {}", _letters_lower) .else_wrap (0xba9c7438) ?;
		writeln! (&mut _stream, "    \\_  digits:   {}", _digits) .else_wrap (0xe2d03da9) ?;
		writeln! (&mut _stream, "    \\_  symbols:  {}", _symbols) .else_wrap (0xe2cc87c1) ?;
		writeln! (&mut _stream, "    \\_  no space: {}", _no_spaces) .else_wrap (0x7bc302e4) ?;
	}
	
	if _display_security || _display_bruteforce {
		
		let _estimates = _estimates.infallible (0xcda96bfe);
		
		if _display_security {
			writeln! (&mut _stream, "\\_  usable for:") .else_wrap (0xb523c114) ?;
			writeln! (&mut _stream, "    \\_  cryptography         {}      with  {:+8.2}  bits of margin", if _estimates.for_cryptography { "   OK   " } else { "!! NO !!" }, _estimates.for_cryptography_margin_bits) .else_wrap (0x83e2fa56) ?;
			writeln! (&mut _stream, "    \\_  authentication       {}      with  {:+8.2}  bits of margin", if _estimates.for_authentication { "   OK   " } else { "!! NO !!" }, _estimates.for_authentication_margin_bits) .else_wrap (0x31c4f00d) ?;
			writeln! (&mut _stream, "    \\_  archival storage     {}      with  {:+8.2}  bits of margin", if _estimates.for_archival { "   OK   " } else { "!! NO !!" }, _estimates.for_archival_margin_bits) .else_wrap (0x0332488c) ?;
			writeln! (&mut _stream, "    \\_  long term storage    {}      with  {:+8.2}  bits of margin", if _estimates.for_long_term { "   OK   " } else { "!! NO !!" }, _estimates.for_long_term_margin_bits) .else_wrap (0xbebfa304) ?;
			writeln! (&mut _stream, "    \\_  short term storage   {}      with  {:+8.2}  bits of margin", if _estimates.for_short_term { "   OK   " } else { "!! NO !!" }, _estimates.for_short_term_margin_bits) .else_wrap (0x19d58942) ?;
		}
		
		if _display_bruteforce {
			writeln! (&mut _stream, "\\_  bruteforce time:") .else_wrap (0xa2206100) ?;
			for (_algorithm, _hours) in _estimates.bruteforce_hours.iter () {
				let _hours = *_hours;
				if let Some (_hours) = _hours {
					let (_time_value, _time_unit) = if _hours < (1.0 / 3600.0 / 10.0) {
						(-1.0, "[0127b098]")
					} else if _hours < (1.0 / 60.0) {
						(_hours * 3600.0, "seconds")
					} else if _hours < 1.0 {
						(_hours * 60.0, "minutes")
					} else if _hours < 24.0 {
						(_hours, "hours")
					} else if _hours < (24.0 * 30.5) {
						(_hours / 24.0, "days")
					} else if _hours < (24.0 * 365.25) {
						(_hours / 24.0 / 30.5, "months")
					} else if _hours < (24.0 * 365.25 * 10.0) {
						(_hours / 24.0 / 365.25, "years")
					} else if _hours < (24.0 * 365.25 * 100.0) {
						(_hours / 24.0 / 365.25 / 10.0, "decades")
					} else if _hours < (24.0 * 365.25 * 1000.0) {
						(_hours / 24.0 / 365.25 / 100.0, "centuries")
					} else if _hours < (24.0 * 365.25 * 1000.0 * 1000.0) {
						(_hours / 24.0 / 365.25 / 1000.0, "millennia")
					} else if _hours < (24.0 * 365.25 * 1_000_000_000.0) {
						(_hours / 24.0 / 365.25 / 1_000_000.0, "millions of years")
					} else if _hours < (24.0 * 365.25 * 1_000_000_000_000.0) {
						(_hours / 24.0 / 365.25 / 1_000_000_000.0, "billions of years")
					} else if _hours < (24.0 * 365.25 * 1_000_000_000_000_000.0) {
						(_hours / 24.0 / 365.25 / 1_000_000_000_000.0, "trillions of years")
					} else {
						(-2.0, "[3f7db730]")
					};
					if _time_value >= 0.0 {
						writeln! (&mut _stream, "    \\_  {:20}    --  {:5.1}  {}", _algorithm, _time_value, _time_unit) .else_wrap (0x78e7778c) ?;
					} else if _time_value == -1.0 {
						writeln! (&mut _stream, "    \\_  {:20}    --         now", _algorithm) .else_wrap (0x7cef82a9) ?;
					} else if _time_value == -2.0 {
						writeln! (&mut _stream, "    \\_  {:20}    --         (more than trillions of years)", _algorithm) .else_wrap (0xbe09b64a) ?;
					} else {
						panic! (unreachable, 0x218ff386);
					}
				} else {
					writeln! (&mut _stream, "    \\_  {:20}    --         (such a large number we can't even compute)", _algorithm) .else_wrap (0x0a4d0a2c) ?;
				};
			}
		}
	}
	
	Ok (())
}




pub(crate) fn pattern_describe (_pattern : &TokenPattern, _token : &Token, _output_options : &OutputOptions, mut _stream : impl Write) -> MainResult {
	
	let (_identifier, _aliases, _labels) = if let TokenPattern::Tagged (_pattern, _tags) = _pattern {
			let _identifier = _tags.identifier.as_ref ();
			let _aliases = _tags.aliases.as_deref () .unwrap_or (&[]);
			let _labels = _tags.labels.as_deref () .unwrap_or (&[]);
			(_identifier, _aliases, _labels)
		} else {
			(None, &[][..], &[][..])
		};
	
	let _entropy = entropy_token (&_pattern) .else_wrap (0x35aa7130) ?;
	let (_bits, _bits_exact) = _entropy.bits_exact ();
	
	let _string = output_token_to_string (&_token, &_output_options) .else_wrap (0x2287d957) ?;
	let _string_length = _string.len ();
	
	let _characters = pattern_classify_chars (_string.as_str ());
	let _characters = Some (_characters);
	
	let _estimates = entropy_estimates (&_entropy) .else_wrap (0xa6cefbae) ?;
	let _estimates = Some (_estimates);
	
	pattern_describe_display (
			_pattern, _identifier, _aliases, _labels,
			_bits, _bits_exact, _string_length,
			_characters,
			_estimates.as_ref (),
			true, true, true, true, true,
			&mut _stream,
		)
}




fn pattern_classify_chars (_string : &str) -> (usize, usize, usize, usize, usize, usize) {
	
	let mut _letters = 0;
	let mut _letters_upper = 0;
	let mut _letters_lower = 0;
	let mut _digits = 0;
	let mut _symbols = 0;
	let mut _no_spaces = 0;
	
	for _char in _string.chars () {
		match _char {
			'a' ..= 'z' => {
					_letters += 1;
					_letters_lower += 1;
					_no_spaces += 1;
				}
			'A' ..= 'Z' => {
					_letters += 1;
					_letters_upper += 1;
					_no_spaces += 1;
				}
			'0' ..= '9' => {
					_digits += 1;
					_no_spaces += 1;
				}
			'!' ..= '~' => {
					_symbols += 1;
					_no_spaces += 1;
				}
			_ => {
					if ! _char.is_whitespace () && ! _char.is_control () {
						_no_spaces += 1;
					}
				}
		}
	}
	
	(_letters, _letters_upper, _letters_lower, _digits, _symbols, _no_spaces)
}


