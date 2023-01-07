

use crate::prelude::*;


use crate::patterns_glyphs as glyphs;
use crate::patterns_separators as separators;
use crate::patterns_macros as macros;




macro_rules! define_sequence {
	
	( $_visibility : vis $_pattern : ident, $_identifier : tt, [ $( $_element : expr, )* ], $_separator : expr ) => {
		::paste::paste! {
			
			static [< _ $_pattern __SEQUENCE_0 >] : &[Rb<TokenPattern>] = &[ $(
					Rb::new_static ($_element),
				)* ];
			
			define_named! ($_visibility $_pattern, $_identifier,
					& TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern __SEQUENCE_0 >] ), Rb::new_static ($_separator))
				);
		}
	};
}




macro_rules! define_repeat {
	
	( $_visibility : vis $_pattern : ident, $_identifier : tt, $_elements : tt, ( $_length : tt : $_each : tt ) ) => {
		macros::__count_call_with! ( [ $_length : $_each ] => define_repeat! ($_visibility $_pattern, $_identifier, $_elements, ));
	};
	
	( $_visibility : vis $_pattern : ident, $_identifier : tt, { $_element : expr => $_separator : expr }, [ $( $_count : literal, )* ] ) => {
		::paste::paste! {
			
			$(
				define_named! ($_visibility [< $_pattern _ $_count >], { concat => $_identifier, "-", $_count },
						& TokenPattern::Repeat (Rb::new_static ($_element), Rb::new_static ($_separator), $_count)
					);
			)*
			
			define_all! ($_visibility [< $_pattern _ALL >], [ $( [< $_pattern _ $_count >], )* ]);
		}
	};
	
	( $_visibility : vis $_pattern : ident, $_identifier : tt, { ( $( $_prefix : expr, )* ), ( $_element : expr => $_separator : expr ), ( $( $_suffix : expr, )* ) }, [ $( $_count : literal, )* ] ) => {
		::paste::paste! {
			
			static [< _ $_pattern __PREFIX_0 >] : &[Rb<TokenPattern>] = &[
					$( Rb::new_static ($_prefix), )*
				];
			static [< _ $_pattern __SUFFIX_0 >] : &[Rb<TokenPattern>] = &[
					$( Rb::new_static ($_suffix), )*
				];
			
			static [< _ $_pattern __PREFIX >] : &TokenPattern =
				& TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern __PREFIX_0 >] ), Rb::new_static (separators::NONE_PATTERN));
			static [< _ $_pattern __SUFFIX >] : &TokenPattern =
				& TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern __SUFFIX_0 >] ), Rb::new_static (separators::NONE_PATTERN));
			
			$(
				static [< _ $_pattern _ $_count __REPEAT >] : &TokenPattern =
						& TokenPattern::Repeat (Rb::new_static ($_element), Rb::new_static ($_separator), $_count);
				
				static [< _ $_pattern _ $_count __SEQUENCE_0 >] : &[Rb<TokenPattern>] = &[
						Rb::new_static ([< _ $_pattern __PREFIX >]),
						Rb::new_static ([< _ $_pattern _ $_count __REPEAT >]),
						Rb::new_static ([< _ $_pattern __SUFFIX >]),
					];
				
				static [< _ $_pattern _ $_count __SEQUENCE >] : &TokenPattern =
					& TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern _ $_count __SEQUENCE_0 >] ), Rb::new_static (separators::NONE_PATTERN));
				
				define_named! ($_visibility [< $_pattern _ $_count >], { concat => $_identifier, "-", $_count },
						[< _ $_pattern _ $_count __SEQUENCE >]
					);
			)*
			
			define_all! ($_visibility [< $_pattern _ALL >], [ $( [< $_pattern _ $_count >], )* ]);
		}
	};
}




macro_rules! define_bytes {
	
	( $_visibility : vis $_pattern : ident, $_identifier : tt, $_glyph : ident, ( $_length : tt : $_each : tt ) ) => {
		macros::__count_call_with! ( [ $_length : $_each ] => define_bytes! ($_visibility $_pattern, $_identifier, $_glyph, ));
	};
	
	( $_visibility : vis $_pattern : ident, $_identifier : tt, $_glyph : ident, [ $( $_count : literal, )* ] ) => {
		::paste::paste! {
			
			$(
				define_named! ($_visibility [< $_pattern _ $_count >], { concat => $_identifier, "-", $_count },
						glyphs::[< $_glyph _ $_count _TOKEN >]
					);
			)*
			
			define_all! ($_visibility [< $_pattern _ALL >], [ $( [< $_pattern _ $_count >], )* ]);
		}
	};
}




macro_rules! define_named {
	
	( $_visibility : vis $_pattern : ident, (), $_wrapped : expr ) => {
		$_visibility static $_pattern : &TokenPattern = $_wrapped;
	};
	
	( $_visibility : vis $_pattern : ident, ( $_identifier : literal ), $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern, { expr => $_identifier }, $_wrapped);
	};
	( $_visibility : vis $_pattern : ident, ( $_identifier : literal, $_alias_1 : literal ), $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern, { expr => $_identifier, $_alias_1 }, $_wrapped);
	};
	( $_visibility : vis $_pattern : ident, ( $_identifier : literal, $_alias_1 : literal, $_alias_2 : literal ), $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern, { expr => $_identifier, $_alias_1, $_alias_2 }, $_wrapped);
	};
	
	( $_visibility : vis $_pattern : ident, { concat => (), $( $_suffix : literal ),* }, $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern, (), $_wrapped);
	};
	( $_visibility : vis $_pattern : ident, { concat => ( $_identifier : literal ), $( $_suffix : literal ),* }, $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern,
			{ expr =>
				concat! ( $_identifier, $( $_suffix ),* )
			}, $_wrapped);
	};
	( $_visibility : vis $_pattern : ident, { concat => ( $_identifier : literal, $_alias_1 : literal ), $( $_suffix : literal ),* }, $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern,
			{ expr =>
				concat! ( $_identifier, $( $_suffix ),* ),
				concat! ( $_alias_1, $( $_suffix ),* )
			}, $_wrapped);
	};
	( $_visibility : vis $_pattern : ident, { concat => ( $_identifier : literal, $_alias_1 : literal, $_alias_2 : literal ), $( $_suffix : literal ),* }, $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern,
			{ expr =>
				concat! ( $_identifier, $( $_suffix ),* ),
				concat! ( $_alias_1, $( $_suffix ),* ),
				concat! ( $_alias_2, $( $_suffix ),* )
			}, $_wrapped);
	};
	
	( $_visibility : vis $_pattern : ident, { expr => () }, $_wrapped : expr ) => {
		define_named! ($_visibility $_pattern, (), $_wrapped);
	};
	( $_visibility : vis $_pattern : ident, { expr => $_identifier : expr $( , $_alias : expr )* }, $_wrapped : expr ) => {
		::paste::paste! {
			
			static [< _ $_pattern __NO_NAME >] : &TokenPattern = $_wrapped;
			$_visibility static $_pattern : &TokenPattern = & TokenPattern::Named ($_identifier, &[ $( $_alias ),* ], Rb::new_static ( [< _ $_pattern __NO_NAME >] ));
		}
	};
}




macro_rules! define_all {
	
	( $_visibility : vis $_all : ident, [ $( $_pattern : expr, )* ] ) => {
		::paste::paste! {
			
			$_visibility static $_all : &[Rb<TokenPattern>] = &[
					$(
						Rb::new_static ($_pattern),
					)*
				];
		}
	};
}




macro_rules! define_constant {
	
	( $_visibility : vis $_constant : ident, $_variant : ident, $_text : expr ) => {
		::paste::paste! {
			
			static [< _ $_constant _TEXT >] : &Text = & Text::$_variant ($_text);
			
			$_visibility static [< $_constant _ATOM >] : &AtomPattern = & AtomPattern::Constant (Rb::new_static ( [< _ $_constant _TEXT >] ));
			$_visibility static [< $_constant _TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_constant _ATOM >] ));
		}
	};
}








define_repeat! (pub DIGITS_BASE10, ("digits-base10", "d"), { glyphs::DIGIT_BASE10_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));

define_repeat! (pub DIGITS_BASE2, ("digits-base2"), { glyphs::DIGIT_BASE2_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN }, (256 : 8));
define_repeat! (pub DIGITS_BASE8, ("digits-base8"), { glyphs::DIGIT_BASE8_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));

define_repeat! (pub DIGITS_BASE16, ("digits-base16", "x"), { glyphs::DIGIT_BASE16_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));

define_repeat! (pub DIGITS_BASE32_HEX, ("digits-base32-hex"), { glyphs::DIGIT_BASE32_HEX_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));
define_repeat! (pub DIGITS_BASE32_RFC, ("digits-base32-rfc"), { glyphs::DIGIT_BASE32_RFC_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN}, (256 : 4));

define_repeat! (pub DIGITS_BASE64_URL, ("digits-base64-url"), { glyphs::DIGIT_BASE64_URL_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN}, (256 : 4));
define_repeat! (pub DIGITS_BASE64_RFC, ("digits-base64-rfc"), { glyphs::DIGIT_BASE64_RFC_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN}, (256 : 4));

define_repeat! (pub DIGITS_BASE58, ("digits-base58"), { glyphs::DIGIT_BASE58_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));
define_repeat! (pub DIGITS_BASE62, ("digits-base62"), { glyphs::DIGIT_BASE62_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));

define_repeat! (pub DIGITS_BECH32, ("digits-bech32"), { glyphs::DIGIT_BECH32_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));

define_repeat! (pub DIGITS_Z85, ("digits-z85"), { glyphs::DIGIT_Z85_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_5_PATTERN }, (260 : 5));




define_repeat! (pub ASCII_LETTER_LOWER, ("ascii-lower"), { glyphs::ASCII_LETTER_LOWER_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));
define_repeat! (pub ASCII_LETTER_UPPER, ("ascii-upper"), { glyphs::ASCII_LETTER_UPPER_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));
define_repeat! (pub ASCII_LETTER_MIXED, ("ascii-mixed"), { glyphs::ASCII_LETTER_MIXED_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));

define_repeat! (pub ASCII_SYMBOLS, ("ascii-symbols"), { glyphs::ASCII_SYMBOL_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));
define_repeat! (pub ASCII_PRINTABLE, ("ascii-any", "r"), { glyphs::ASCII_PRINTABLE_TOKEN => separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN }, (256 : 4));




define_sequence! (pub ASCII_CONSONANT_VOWEL_LOWER_WORD, (), [
		glyphs::ASCII_CONSONANT_LOWER_TOKEN,
		glyphs::ASCII_VOWEL_LOWER_TOKEN,
		glyphs::ASCII_CONSONANT_LOWER_TOKEN,
		glyphs::ASCII_VOWEL_LOWER_TOKEN,
	], separators::NONE_PATTERN);

define_sequence! (pub ASCII_CONSONANT_VOWEL_UPPER_WORD, (), [
		glyphs::ASCII_CONSONANT_UPPER_TOKEN,
		glyphs::ASCII_VOWEL_UPPER_TOKEN,
		glyphs::ASCII_CONSONANT_UPPER_TOKEN,
		glyphs::ASCII_VOWEL_UPPER_TOKEN,
	], separators::NONE_PATTERN);

define_sequence! (pub ASCII_CONSONANT_VOWEL_MIXED_WORD, (), [
		glyphs::ASCII_CONSONANT_MIXED_TOKEN,
		glyphs::ASCII_VOWEL_MIXED_TOKEN,
		glyphs::ASCII_CONSONANT_MIXED_TOKEN,
		glyphs::ASCII_VOWEL_MIXED_TOKEN,
	], separators::NONE_PATTERN);

define_repeat! (pub ASCII_CONSONANT_VOWEL_LOWER, ("cv-lower", "cv"), { ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN }, (64 : 1));
define_repeat! (pub ASCII_CONSONANT_VOWEL_UPPER, ("cv-upper"), { ASCII_CONSONANT_VOWEL_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN }, (64 : 1));
define_repeat! (pub ASCII_CONSONANT_VOWEL_MIXED, ("cv-mixed"), { ASCII_CONSONANT_VOWEL_MIXED_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN }, (64 : 1));




define_sequence! (pub ASCII_CONSONANT_VOWEL_PLUS_A_WORD, (), [
		glyphs::DIGIT_BASE10_TOKEN,
		glyphs::DIGIT_BASE10_TOKEN,
		glyphs::DIGIT_BASE10_TOKEN,
		glyphs::DIGIT_BASE10_TOKEN,
	], separators::NONE_PATTERN);

define_sequence! (pub ASCII_CONSONANT_VOWEL_PLUS_B_WORD, (), [
		glyphs::ASCII_CONSONANT_UPPER_TOKEN,
		glyphs::ASCII_VOWEL_UPPER_TOKEN,
		glyphs::DIGIT_BASE10_TOKEN,
		glyphs::DIGIT_BASE10_TOKEN,
	], separators::NONE_PATTERN);

define_sequence! (pub ASCII_CONSONANT_VOWEL_PLUS_C_WORD, (), [
		glyphs::ASCII_CONSONANT_UPPER_TOKEN,
		glyphs::ASCII_VOWEL_UPPER_TOKEN,
		glyphs::DIGIT_BASE10_TOKEN,
		glyphs::ASCII_SYMBOL_TOKEN,
	], separators::NONE_PATTERN);


define_repeat! (pub ASCII_CONSONANT_VOWEL_PLUS_A, ("cv-plus-a", "cva"), {
		(),
		( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
		( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_A_WORD, )
	}, ( 16 : 1 ));

define_repeat! (pub ASCII_CONSONANT_VOWEL_PLUS_B, ("cv-plus-b", "cvb"), {
		(),
		( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
		( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_B_WORD, )
	}, ( 16 : 1 ));

define_repeat! (pub ASCII_CONSONANT_VOWEL_PLUS_C, ("cv-plus-c", "cvc"), {
		(),
		( ASCII_CONSONANT_VOWEL_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN ),
		( separators::SPACE_OPTIONAL_TOKEN, ASCII_CONSONANT_VOWEL_PLUS_C_WORD, )
	}, ( 16 : 1 ));




define_sequence! (pub PROQUINT_LOWER_WORD, (), [
		glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
		glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
		glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
		glyphs::PROQUINT_VOWEL_LOWER_TOKEN,
		glyphs::PROQUINT_CONSONANT_LOWER_TOKEN,
	], separators::NONE_PATTERN);

define_sequence! (pub PROQUINT_UPPER_WORD, (), [
		glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
		glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
		glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
		glyphs::PROQUINT_VOWEL_UPPER_TOKEN,
		glyphs::PROQUINT_CONSONANT_UPPER_TOKEN,
	], separators::NONE_PATTERN);

define_repeat! (pub PROQUINT_LOWER, ("proquint-lower", "proquint"), { PROQUINT_LOWER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN }, (64 : 1));
define_repeat! (pub PROQUINT_UPPER, ("proquint-upper"), { PROQUINT_UPPER_WORD => separators::SPACE_OPTIONAL_INFIX_PATTERN }, (64 : 1));




define_sequence! (pub KOREMUTAKE_WORD_A, (), [
		glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
		glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
	], separators::NONE_PATTERN);

define_sequence! (pub KOREMUTAKE_WORD_B, (), [
		glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
		glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
		glyphs::KOREMUTAKE_SYLLABLE_TOKEN,
	], separators::NONE_PATTERN);

define_repeat! (pub KOREMUTAKE_A, ("koremutake-a"), { KOREMUTAKE_WORD_A => separators::SPACE_OPTIONAL_INFIX_PATTERN }, (64 : 1));
define_repeat! (pub KOREMUTAKE_B, ("koremutake-b"), { KOREMUTAKE_WORD_B => separators::SPACE_OPTIONAL_INFIX_PATTERN }, (64 : 1));




define_sequence! (pub MNEMONIC_TUPLE, (), [
		glyphs::MNEMONIC_WORD_TOKEN,
		glyphs::MNEMONIC_WORD_TOKEN,
		glyphs::MNEMONIC_WORD_TOKEN,
	], separators::SPACE_MANDATORY_INFIX_PATTERN);

define_repeat! (pub MNEMONIC, ("mnemonic"), { MNEMONIC_TUPLE => separators::SPACE_HYPHEN_SPACE_MANDATORY_INFIX_PATTERN }, (66 : 1));




define_sequence! (pub BIP0039_TUPLE, (), [
		glyphs::BIP0039_WORD_TOKEN,
		glyphs::BIP0039_WORD_TOKEN,
		glyphs::BIP0039_WORD_TOKEN,
	], separators::SPACE_MANDATORY_INFIX_PATTERN);

define_repeat! (pub BIP0039, ("bip0039"), { BIP0039_TUPLE => separators::SPACE_HYPHEN_SPACE_MANDATORY_INFIX_PATTERN }, (66 : 1));




define_repeat! (pub NATO, ("nato"), { glyphs::NATO_WORD_TOKEN => separators::SPACE_MANDATORY_INFIX_PATTERN }, (64 : 1));




define_sequence! (pub UUID_V4, ("uuid-v4"), [
		glyphs::UUID_ANY_FIELD_1_TOKEN,
		glyphs::UUID_ANY_FIELD_2_TOKEN,
		glyphs::UUID_V4_FIELD_3_TOKEN,
		glyphs::UUID_V4_FIELD_4_TOKEN,
		glyphs::UUID_ANY_FIELD_5_TOKEN,
	], separators::HYPHEN_MANDATORY_INFIX_PATTERN);

define_all! (pub UUID_ALL, [
		UUID_V4,
	]);




define_constant! (IP_127_PREFIX, Str, "127");
define_constant! (IP_10_PREFIX, Str, "10");
define_constant! (IP_172_PREFIX, Str, "172");
define_constant! (IP_192_A_PREFIX, Str, "192");
define_constant! (IP_192_B_PREFIX, Str, "168");
define_constant! (IP_MAC_PREFIX, Str, "02");

define_sequence! (pub IP_127, ("ip-127"), [
		IP_127_PREFIX_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
	], separators::DOT_MANDATORY_INFIX_PATTERN);

define_sequence! (pub IP_10, ("ip-10"), [
		IP_10_PREFIX_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
	], separators::DOT_MANDATORY_INFIX_PATTERN);

define_sequence! (pub IP_172, ("ip-172"), [
		IP_172_PREFIX_TOKEN,
		glyphs::INTEGER_1_30_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
	], separators::DOT_MANDATORY_INFIX_PATTERN);

define_sequence! (pub IP_192, ("ip-192"), [
		IP_192_A_PREFIX_TOKEN,
		IP_192_B_PREFIX_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
		glyphs::INTEGER_2_253_TOKEN,
	], separators::DOT_MANDATORY_INFIX_PATTERN);

define_sequence! (pub IP_MAC, ("ip-mac"), [
		IP_MAC_PREFIX_TOKEN,
		glyphs::INTEGER_8B_HEX_TOKEN,
		glyphs::INTEGER_8B_HEX_TOKEN,
		glyphs::INTEGER_8B_HEX_TOKEN,
		glyphs::INTEGER_8B_HEX_TOKEN,
		glyphs::INTEGER_8B_HEX_TOKEN,
	], separators::COLON_MANDATORY_INFIX_PATTERN);

define_all! (pub IP_ALL, [
		IP_127,
		IP_10,
		IP_172,
		IP_192,
		IP_MAC,
	]);




define_bytes! (pub BYTES_HEX, ("bytes-hex", "b"), BYTES_HEX, ( 512 : 4 ));




define_named! (pub TIMESTAMP_STRING_DATETIME, ("timestamp-date-time", "date-time"), glyphs::TIMESTAMP_STRING_DATETIME_TOKEN);
define_named! (pub TIMESTAMP_STRING_DATE, ("timestamp-date", "date"), glyphs::TIMESTAMP_STRING_DATE_TOKEN);
define_named! (pub TIMESTAMP_STRING_TIME, ("timestamp-time", "time"), glyphs::TIMESTAMP_STRING_TIME_TOKEN);
define_named! (pub TIMESTAMP_SECONDS_DEC, ("timestamp-sec", "timestamp"), glyphs::TIMESTAMP_SECONDS_DEC_TOKEN);
define_named! (pub TIMESTAMP_SECONDS_HEX, ("timestamp-sec-hex"), glyphs::TIMESTAMP_SECONDS_HEX_TOKEN);
define_named! (pub TIMESTAMP_NANOSECONDS_DEC, ("timestamp-nano"), glyphs::TIMESTAMP_NANOSECONDS_DEC_TOKEN);
define_named! (pub TIMESTAMP_NANOSECONDS_HEX, ("timestamp-nano-hex"), glyphs::TIMESTAMP_NANOSECONDS_HEX_TOKEN);
define_named! (pub TIMESTAMP_FLAKE_SECONDS_DEC, ("timestamp-flake"), glyphs::TIMESTAMP_FLAKE_SECONDS_DEC_TOKEN);
define_named! (pub TIMESTAMP_FLAKE_SECONDS_HEX, ("timestamp-flake-hex"), glyphs::TIMESTAMP_FLAKE_SECONDS_HEX_TOKEN);

define_all! (pub TIMESTAMP_ALL, [
		TIMESTAMP_STRING_DATETIME,
		TIMESTAMP_STRING_DATE,
		TIMESTAMP_STRING_TIME,
		TIMESTAMP_SECONDS_DEC,
		TIMESTAMP_SECONDS_HEX,
		TIMESTAMP_NANOSECONDS_DEC,
		TIMESTAMP_NANOSECONDS_HEX,
		TIMESTAMP_FLAKE_SECONDS_DEC,
		TIMESTAMP_FLAKE_SECONDS_HEX,
	]);


define_repeat! (pub FLAKE_SECONDS, ("flake"), {
		( glyphs::TIMESTAMP_FLAKE_SECONDS_HEX_TOKEN, separators::HYPHEN_OPTIONAL_TOKEN, ),
		( glyphs::BYTES_HEX_4_TOKEN => separators::HYPHEN_OPTIONAL_INFIX_PATTERN ),
		()
	}, ( 16 : 1 ));




pub static ALL : &[&[Rb<TokenPattern>]] = &[
		
		DIGITS_BASE2_ALL,
		DIGITS_BASE8_ALL,
		DIGITS_BASE10_ALL,
		DIGITS_BASE16_ALL,
		DIGITS_BASE32_HEX_ALL,
		DIGITS_BASE32_RFC_ALL,
		DIGITS_BASE64_URL_ALL,
		DIGITS_BASE64_RFC_ALL,
		DIGITS_BASE58_ALL,
		DIGITS_BASE62_ALL,
		DIGITS_BECH32_ALL,
		DIGITS_Z85_ALL,
		
		ASCII_LETTER_LOWER_ALL,
		ASCII_LETTER_UPPER_ALL,
		ASCII_LETTER_MIXED_ALL,
		
		ASCII_SYMBOLS_ALL,
		ASCII_PRINTABLE_ALL,
		
		ASCII_CONSONANT_VOWEL_LOWER_ALL,
		ASCII_CONSONANT_VOWEL_UPPER_ALL,
		ASCII_CONSONANT_VOWEL_MIXED_ALL,
		
		ASCII_CONSONANT_VOWEL_PLUS_A_ALL,
		ASCII_CONSONANT_VOWEL_PLUS_B_ALL,
		ASCII_CONSONANT_VOWEL_PLUS_C_ALL,
		
		PROQUINT_LOWER_ALL,
		PROQUINT_UPPER_ALL,
		
		KOREMUTAKE_A_ALL,
		KOREMUTAKE_B_ALL,
		
		MNEMONIC_ALL,
		BIP0039_ALL,
		NATO_ALL,
		
		UUID_ALL,
		
		IP_ALL,
		
		BYTES_HEX_ALL,
		
		TIMESTAMP_ALL,
		FLAKE_SECONDS_ALL,
		
	];


