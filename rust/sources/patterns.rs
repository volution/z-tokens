

use crate::prelude::*;




pub mod glyphs {
	
	use super::*;
	use super::ascii::*;
	
	
	
	
	macro_rules! define_set {
		( $_visibility : vis $_pattern : ident, [ $( $_char : expr, )* ] ) => {
			::paste::paste! {
				
				$(
					static [< _ $_pattern __ $_char __TEXT >] : &Text = & Text::Char ($_char);
					static [< _ $_pattern __ $_char __GLYPH >] : &Glyph = & Glyph::Text (Rb::new_static ( [< _ $_pattern __ $_char __TEXT >] ));
				)*
				
				#[ doc = concat! ( "Glyph character set for ", $( "`", stringify! ($_char), "` " ),*, "." ) ]
				$_visibility static [< $_pattern _SET >] : &[Rb<Glyph>] = &[ $(
						Rb::new_static ( [< _ $_pattern __ $_char __GLYPH >] ),
					)* ];
				
				$_visibility static [< $_pattern _GLYPH >] : &GlyphPattern = & GlyphPattern::Set (RbList::from_static ( [< $_pattern _SET >] ));
				$_visibility static [< $_pattern _ATOM >] : &AtomPattern = & AtomPattern::Glyph (Rb::new_static ( [< $_pattern _GLYPH >] ));
				$_visibility static [< $_pattern _TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_pattern _ATOM >] ));
			}
		};
	}
	
	
	
	
	define_set! (pub DIGIT_BASE2, [ '0', '1', ]);
	define_set! (pub DIGIT_BASE8, [ '0', '1', '2', '3', '4', '5', '6', '7', ]);
	define_set! (pub DIGIT_BASE10, [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ]);
	
	define_set! (pub DIGIT_BASE16, [
			'0', '1', '2', '3', '4', '5', '6', '7',
			'8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
		]);
	
	define_set! (pub DIGIT_BASE32_HEX, [
			'0', '1', '2', '3', '4', '5', '6', '7',
			'8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
			'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
			'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
		]);
	
	
	
	
	// NOTE:  => https://www.ietf.org/rfc/rfc4648.html
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" abcdefgh ijklmnop qrstuvw xyz234567 """ if c != " " ]))'
	define_set! (pub DIGIT_BASE32_RFC, [
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
			'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
			'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
			'y', 'z', '2', '3', '4', '5', '6', '7',
		]);
	
	// NOTE:   => https://www.ietf.org/rfc/rfc4648.html
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789 +/ """ if c != " " ]))'
	define_set! (pub DIGIT_BASE64_RFC, [
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
			'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
			'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
			'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
			'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
			'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
			'w', 'x', 'y', 'z', '0', '1', '2', '3',
			'4', '5', '6', '7', '8', '9', C2B, C2F,
		]);
	
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" ABCDEFGHIJKLMNOPQRSTUVWXYZ abcdefghijklmnopqrstuvwxyz 0123456789 -_ """ if c != " " ]))'
	define_set! (pub DIGIT_BASE64_URL, [
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
			'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P',
			'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
			'Y', 'Z', 'a', 'b', 'c', 'd', 'e', 'f',
			'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n',
			'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
			'w', 'x', 'y', 'z', '0', '1', '2', '3',
			'4', '5', '6', '7', '8', '9', C2D, C5F,
		]);
	
	
	
	
	// NOTE:  =>  https://en.bitcoinwiki.org/wiki/Base58
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" 123456789 ABCDEFGH JKLMN PQRSTUVWXYZ abcdefghijk mnopqrstuvwxyz """ if c != " " ]))'
	define_set! (pub DIGIT_BASE58, [
			'1', '2', '3', '4', '5', '6', '7', '8', '9',
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H',
			'J', 'K', 'L', 'M', 'N',
			'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k',
			'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
		]);
	
	// NOTE:  =>  https://en.bitcoin.it/wiki/BIP_0173
	// NOTE:  #>  python -c 'print (", ".join ([ repr(c) for c in r""" qpzry9x8 gf2tvdw0 s3jn54kh ce6mua7l """ if c != " " ]))'
	define_set! (pub DIGIT_BECH32, [
			'q', 'p', 'z', 'r', 'y', '9', 'x', '8',
			'g', 'f', '2', 't', 'v', 'd', 'w', '0',
			's', '3', 'j', 'n', '5', '4', 'k', 'h',
			'c', 'e', '6', 'm', 'u', 'a', '7', 'l',
		]);
	
	// NOTE:  => https://rfc.zeromq.org/spec/32/
	define_set! (pub DIGIT_Z85, [
			'0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j',
			'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't',
			'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D',
			'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N',
			'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X',
			'Y', 'Z', C2E, C2D, C3A, C2B, C3D, C5E, C21, C2F,
			C2A, C3F, C26, C3C, C3E, C28, C29, C5B, C5D, C7B,
			C7D, C40, C25, C24, C23,
		]);
	
	
	
	
	define_set! (pub VOWEL_LOWER, [
			'a', 'e', 'i', 'o', 'u',
		]);
	define_set! (pub VOWEL_UPPER, [
			'A', 'B', 'I', 'O', 'U',
		]);
	define_set! (pub VOWEL_MIXED, [
			'a', 'e', 'i', 'o', 'u',
			'A', 'B', 'I', 'O', 'U',
		]);
	
	
	define_set! (pub CONSONANT_LOWER, [
			'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
		]);
	define_set! (pub CONSONANT_UPPER, [
			'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
		]);
	define_set! (pub CONSONANT_MIXED, [
			'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z',
			'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z',
		]);
	
	
	define_set! (pub LETTER_LOWER, [
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
		]);
	define_set! (pub LETTER_UPPER, [
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
		]);
	define_set! (pub LETTER_MIXED, [
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
			'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
		]);
	
	
	
	
	// NOTE:  #>  python -c 'print (", ".join ([ "C%0X" % ord (c) for c in r"""!"#$%&'\''()*+,-./:;<=>?@[\]^_`{|}~""" ]))'
	define_set! (pub ASCII_SPECIAL, [
			C21, C22, C23, C24, C25, C26, C27, C28, C29, C2A, C2B, C2C, C2D, C2E, C2F, C3A, C3B, C3C, C3D, C3E, C3F, C40, C5B, C5C, C5D, C5E, C5F, C60, C7B, C7C, C7D, C7E,
		]);
	
	// NOTE:  #>  python -c 'print ("".join ([ chr(c) for c in range (33, 127) ]))'
	// NOTE:  #>  python -c 'print (", ".join ([ "C%0X" % ord (c) for c in r"""!"#$%&'\''()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\]^_`abcdefghijklmnopqrstuvwxyz{|}~""" ]))'
	define_set! (pub ASCII_PRINTABLE, [
			     C21, C22, C23, C24, C25, C26, C27, C28, C29, C2A, C2B, C2C, C2D, C2E, C2F,
			C30, C31, C32, C33, C34, C35, C36, C37, C38, C39, C3A, C3B, C3C, C3D, C3E, C3F,
			C40, C41, C42, C43, C44, C45, C46, C47, C48, C49, C4A, C4B, C4C, C4D, C4E, C4F,
			C50, C51, C52, C53, C54, C55, C56, C57, C58, C59, C5A, C5B, C5C, C5D, C5E, C5F,
			C60, C61, C62, C63, C64, C65, C66, C67, C68, C69, C6A, C6B, C6C, C6D, C6E, C6F,
			C70, C71, C72, C73, C74, C75, C76, C77, C78, C79, C7A, C7B, C7C, C7D, C7E,
		]);
	
}




pub mod tokens {
	
	use super::*;
	
	
	
	
	macro_rules! define_sequence {
		( $_visibility : vis $_pattern : ident, $_identifier : literal, [ $( $_element : expr, )* ], $_separator : expr ) => {
			::paste::paste! {
				
				static [< _ $_pattern __SEQUENCE >] : &[Rb<TokenPattern>] = &[ $(
						Rb::new_static ($_element),
					)* ];
				
				static [< _ $_pattern __NO_NAME >] : &TokenPattern = & TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern __SEQUENCE >] ), $_separator);
				$_visibility static [< $_pattern >] : &TokenPattern = & TokenPattern::Named ($_identifier, Rb::new_static ( [< _ $_pattern __NO_NAME >] ));
			}
		};
	}
	
	
	macro_rules! define_repeat {
		
		( $_visibility : vis $_pattern : ident, $_identifier : literal, $_element : expr, $_separator : expr, ( $_length : tt : $_each : tt ) ) => {
			macros::__count_call_with! ( [ $_length : $_each ] => define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, ));
		};
		
		( $_visibility : vis $_pattern : ident, $_identifier : literal, $_element : expr, $_separator : expr, [ $( $_count : literal, )* ] ) => {
			::paste::paste! {
				
				$(
					static [< _ $_pattern _ $_count __NO_NAME >] : &TokenPattern = & TokenPattern::Repeat (Rb::new_static ($_element), $_separator, $_count);
					$_visibility static [< $_pattern _ $_count >] : &TokenPattern = & TokenPattern::Named (concat! ($_identifier, "-", $_count), Rb::new_static ( [< _ $_pattern _ $_count __NO_NAME >] ));
				)*
				
				$_visibility static [< $_pattern _ALL >] : &[Rb<TokenPattern>] = &[ $(
						Rb::new_static ( [< $_pattern _ $_count >] ),
					)* ];
			}
		};
	}
	
	
	
	
	define_repeat! (pub DIGITS_BASE10, "digits-base10", glyphs::DIGIT_BASE10_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN), (64 : 4));
	
	define_repeat! (pub DIGITS_BASE2, "digits-base2", glyphs::DIGIT_BASE2_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (64 : 8));
	define_repeat! (pub DIGITS_BASE8, "digits-base8", glyphs::DIGIT_BASE8_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (64 : 8));
	
	define_repeat! (pub DIGITS_BASE16, "digits-base16", glyphs::DIGIT_BASE16_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_4_PATTERN), (64 : 4));
	
	define_repeat! (pub DIGITS_BASE32_HEX, "digits-base32-hex", glyphs::DIGIT_BASE32_HEX_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (64 : 8));
	define_repeat! (pub DIGITS_BASE32_RFC, "digits-base32-rfc", glyphs::DIGIT_BASE32_RFC_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (64 : 8));
	
	define_repeat! (pub DIGITS_BASE64_URL, "digits-base64-url", glyphs::DIGIT_BASE64_URL_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (64 : 8));
	define_repeat! (pub DIGITS_BASE64_RFC, "digits-base64-rfc", glyphs::DIGIT_BASE64_RFC_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (64 : 8));
	
	define_repeat! (pub DIGITS_BASE58, "digits-base58", glyphs::DIGIT_BASE58_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_8_PATTERN), (64 : 8));
	
	define_repeat! (pub DIGITS_BECH32, "digits-bech32", glyphs::DIGIT_BECH32_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_5_PATTERN), (65 : 5));
	
	define_repeat! (pub DIGITS_Z85, "digits-z85", glyphs::DIGIT_Z85_TOKEN, Rb::new_static (separators::SPACE_OPTIONAL_INFIX_EACH_5_PATTERN), (65 : 5));
	
	
	
	
	define_sequence! (pub CONSONANT_VOWEL_LOWER_SYLLABLE, "cv-lower-syllable", [
			glyphs::CONSONANT_LOWER_TOKEN,
			glyphs::VOWEL_LOWER_TOKEN,
			glyphs::CONSONANT_LOWER_TOKEN,
			glyphs::VOWEL_LOWER_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_sequence! (pub CONSONANT_VOWEL_UPPER_SYLLABLE, "cv-upper-syllable", [
			glyphs::CONSONANT_UPPER_TOKEN,
			glyphs::VOWEL_UPPER_TOKEN,
			glyphs::CONSONANT_UPPER_TOKEN,
			glyphs::VOWEL_UPPER_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_sequence! (pub CONSONANT_VOWEL_MIXED_SYLLABLE, "cv-mixed-syllable", [
			glyphs::CONSONANT_MIXED_TOKEN,
			glyphs::VOWEL_MIXED_TOKEN,
			glyphs::CONSONANT_MIXED_TOKEN,
			glyphs::VOWEL_MIXED_TOKEN,
		], Rb::new_static (separators::NONE_PATTERN));
	
	define_repeat! (pub CONSONANT_VOWEL_LOWER, "cv-lower", CONSONANT_VOWEL_LOWER_SYLLABLE, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (8 : 1));
	define_repeat! (pub CONSONANT_VOWEL_UPPER, "cv-upper", CONSONANT_VOWEL_UPPER_SYLLABLE, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (8 : 1));
	define_repeat! (pub CONSONANT_VOWEL_MIXED, "cv-mixed", CONSONANT_VOWEL_MIXED_SYLLABLE, Rb::new_static (separators::SPACE_MANDATORY_INFIX_PATTERN), (8 : 1));
	
	
	
	
	define_repeat! (pub LETTER_LOWER, "ascii-lower", glyphs::LETTER_LOWER_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (64 : 4));
	define_repeat! (pub LETTER_UPPER, "ascii-upper", glyphs::LETTER_UPPER_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (64 : 4));
	define_repeat! (pub LETTER_MIXED, "ascii-mixed", glyphs::LETTER_MIXED_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (64 : 4));
	
	
	
	
	define_repeat! (pub ASCII_PRINTABLE, "ascii-any", glyphs::ASCII_PRINTABLE_TOKEN, Rb::new_static (separators::SPACE_MANDATORY_INFIX_EACH_4_PATTERN), (64 : 4));
	
	
	
	
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
			DIGITS_BECH32_ALL,
			DIGITS_Z85_ALL,
			
			CONSONANT_VOWEL_LOWER_ALL,
			CONSONANT_VOWEL_UPPER_ALL,
			CONSONANT_VOWEL_MIXED_ALL,
			
			LETTER_LOWER_ALL,
			LETTER_UPPER_ALL,
			LETTER_MIXED_ALL,
			
			ASCII_PRINTABLE_ALL,
		];
}




pub mod separators {
	
	use super::*;
	
	
	
	
	macro_rules! define_separator {
		( $_visibility : vis $_pattern : ident, $_text : expr, infix, ( $_length : tt : $_each : tt ) ) => {
			macros::__count_call_with! ( [ $_length : $_each ] => define_separator! ($_visibility $_pattern, $_text, infix, ));
		};
		( $_visibility : vis $_pattern : ident, $_text : expr, infix, [ $( $_infix_each : literal, )* ] ) => {
			::paste::paste! {
				
				static [< _ $_pattern _TEXT >] : &Text = & Text::Static ($_text);
				
				$_visibility static [< $_pattern _MANDATORY_SEPARATOR >] : &Separator = & Separator::Mandatory (Rb::new_static ( [< _ $_pattern _TEXT >] ));
				$_visibility static [< $_pattern _OPTIONAL_SEPARATOR >] : &Separator = & Separator::Optional (Rb::new_static ( [< _ $_pattern _TEXT >] ));
				
				$_visibility static [< $_pattern _MANDATORY_INFIX_PATTERN >] : &SeparatorPattern = & SeparatorPattern::Infix (Rb::new_static ( [< $_pattern _MANDATORY_SEPARATOR >] ));
				$_visibility static [< $_pattern _OPTIONAL_INFIX_PATTERN >] : &SeparatorPattern = & SeparatorPattern::Infix (Rb::new_static ( [< $_pattern _OPTIONAL_SEPARATOR >] ));
				
				$(
					$_visibility static [< $_pattern _MANDATORY_INFIX_EACH_ $_infix_each _PATTERN >] : &SeparatorPattern = & SeparatorPattern::InfixEach (Rb::new_static ( [< $_pattern _MANDATORY_SEPARATOR >] ), $_infix_each);
					$_visibility static [< $_pattern _OPTIONAL_INFIX_EACH_ $_infix_each _PATTERN >] : &SeparatorPattern = & SeparatorPattern::InfixEach (Rb::new_static ( [< $_pattern _OPTIONAL_SEPARATOR >] ), $_infix_each);
				)*
			}
		};
	}
	
	
	
	
	pub static NONE_PATTERN : &SeparatorPattern = & SeparatorPattern::None;
	
	
	define_separator! (pub SPACE, " ", infix, ( 16 : 1 ));
	define_separator! (pub DOT, ".", infix, ( 16 : 1 ));
	define_separator! (pub HYPHEN, "-", infix, ( 16 : 1 ));
}




pub fn all_token_patterns () -> RbList<(String, Rb<TokenPattern>)> {
	
	let mut _collector = Vec::with_capacity (1024);
	
	for _patterns in tokens::ALL.iter () {
		for _pattern in _patterns.iter () {
			match _pattern.as_ref () {
				TokenPattern::Named (_identifier, _) =>
					_collector.push ((String::from (*_identifier), _pattern.clone ())),
				_ =>
					panic! (0xcb0098dd),
			}
		}
	}
	
	RbList::from_vec (_collector)
}




pub mod ascii {
	
	// NOTE:  python -c 'for c in range (32, 127) : print ("pub const C%02X : char = %r;" % (c, chr(c)))'
	
	pub const C20 : char = ' ';
	pub const C21 : char = '!';
	pub const C22 : char = '"';
	pub const C23 : char = '#';
	pub const C24 : char = '$';
	pub const C25 : char = '%';
	pub const C26 : char = '&';
	pub const C27 : char = '\'';
	pub const C28 : char = '(';
	pub const C29 : char = ')';
	pub const C2A : char = '*';
	pub const C2B : char = '+';
	pub const C2C : char = ',';
	pub const C2D : char = '-';
	pub const C2E : char = '.';
	pub const C2F : char = '/';
	pub const C30 : char = '0';
	pub const C31 : char = '1';
	pub const C32 : char = '2';
	pub const C33 : char = '3';
	pub const C34 : char = '4';
	pub const C35 : char = '5';
	pub const C36 : char = '6';
	pub const C37 : char = '7';
	pub const C38 : char = '8';
	pub const C39 : char = '9';
	pub const C3A : char = ':';
	pub const C3B : char = ';';
	pub const C3C : char = '<';
	pub const C3D : char = '=';
	pub const C3E : char = '>';
	pub const C3F : char = '?';
	pub const C40 : char = '@';
	pub const C41 : char = 'A';
	pub const C42 : char = 'B';
	pub const C43 : char = 'C';
	pub const C44 : char = 'D';
	pub const C45 : char = 'E';
	pub const C46 : char = 'F';
	pub const C47 : char = 'G';
	pub const C48 : char = 'H';
	pub const C49 : char = 'I';
	pub const C4A : char = 'J';
	pub const C4B : char = 'K';
	pub const C4C : char = 'L';
	pub const C4D : char = 'M';
	pub const C4E : char = 'N';
	pub const C4F : char = 'O';
	pub const C50 : char = 'P';
	pub const C51 : char = 'Q';
	pub const C52 : char = 'R';
	pub const C53 : char = 'S';
	pub const C54 : char = 'T';
	pub const C55 : char = 'U';
	pub const C56 : char = 'V';
	pub const C57 : char = 'W';
	pub const C58 : char = 'X';
	pub const C59 : char = 'Y';
	pub const C5A : char = 'Z';
	pub const C5B : char = '[';
	pub const C5C : char = '\\';
	pub const C5D : char = ']';
	pub const C5E : char = '^';
	pub const C5F : char = '_';
	pub const C60 : char = '`';
	pub const C61 : char = 'a';
	pub const C62 : char = 'b';
	pub const C63 : char = 'c';
	pub const C64 : char = 'd';
	pub const C65 : char = 'e';
	pub const C66 : char = 'f';
	pub const C67 : char = 'g';
	pub const C68 : char = 'h';
	pub const C69 : char = 'i';
	pub const C6A : char = 'j';
	pub const C6B : char = 'k';
	pub const C6C : char = 'l';
	pub const C6D : char = 'm';
	pub const C6E : char = 'n';
	pub const C6F : char = 'o';
	pub const C70 : char = 'p';
	pub const C71 : char = 'q';
	pub const C72 : char = 'r';
	pub const C73 : char = 's';
	pub const C74 : char = 't';
	pub const C75 : char = 'u';
	pub const C76 : char = 'v';
	pub const C77 : char = 'w';
	pub const C78 : char = 'x';
	pub const C79 : char = 'y';
	pub const C7A : char = 'z';
	pub const C7B : char = '{';
	pub const C7C : char = '|';
	pub const C7D : char = '}';
	pub const C7E : char = '~';
}




pub(crate) mod macros {
	
	// NOTE:  #>  python -c $'print ("macro_rules! __count_list {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( %d, %d )" % (n, e) + "=> { [ " + ", ".join (["%d" % c for c in range (0, n + 1, e) if c != 0]) + ", ] };")\nprint ("}")' >| ./sources/patterns_count_list.in
	include! ("./patterns_count_list.in");
	
	// NOTE:  #>  python -c $'print ("macro_rules! __count_call_each {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( [ %d : %d ] => $c:ident! ( $($p:tt)* ) )" % (n, e) + "=> {\\n" + "\\n".join (["\t$c! ( $($p)* %d );" % c for c in range (0, n + 1, e) if c != 0]) + "\\n};")\nprint ("}")' >| ./sources/patterns_count_call_each.in
	include! ("./patterns_count_call_each.in");
	
	// NOTE:  #>  python -c $'print ("macro_rules! __count_call_with {")\nfor n in range (1, 512 + 1) :\n  for e in range (1, 16 + 1) :\n    if e <= n : print ("( [ %d : %d ] => $c:ident! ( $($p:tt)* ) )" % (n, e) + "=> { $c! ( $($p)* [ " + ", ".join (["%d" % c for c in range (0, n + 1, e) if c != 0]) + ", ] ); };")\nprint ("}")' >| ./sources/patterns_count_call_with.in
	include! ("./patterns_count_call_with.in");
	
	pub(crate) use __count_list;
	pub(crate) use __count_call_each;
	pub(crate) use __count_call_with;
}

