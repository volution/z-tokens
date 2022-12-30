

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
				
				#[ doc = concat! ( "Glyph character set for `", $( stringify! ($_char) ),*, "`." ) ]
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
	define_set! (pub DIGIT_BASE16, [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', ]);
	
	define_set! (pub DIGIT_BASE32_HEX, [
			'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f',
			'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v',
		]);
	define_set! (pub DIGIT_BASE32_RFC4648, [
			'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h',
			'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p',
			'q', 'r', 's', 't', 'u', 'v', 'w', 'x',
			'y', 'z', '2', '3', '4', '5', '6', '7',
		]);
	
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
	
	
	
	
	define_set! (pub VOWEL_LOWER, [ 'a', 'e', 'i', 'o', 'u', ]);
	define_set! (pub VOWEL_UPPER, [ 'A', 'B', 'I', 'O', 'U', ]);
	
	define_set! (pub CONSONANT_LOWER, [ 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z', ]);
	define_set! (pub CONSONANT_UPPER, [ 'B', 'C', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'Q', 'R', 'S', 'T', 'V', 'W', 'X', 'Y', 'Z', ]);
}




pub mod tokens {
	
	use super::*;
	
	macro_rules! define_sequence {
		( $_visibility : vis $_pattern : ident, $_identifier : expr, [ $( $_element : expr, )* ], $_separator : expr ) => {
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
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, 4 ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, 8 ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, 5, 6, 7, 8, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, 16 ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, 32 ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, 64 ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, 128 ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, 256 ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97, 98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116, 117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255, 256, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, [ $( $_count : expr, )* ] ) => {
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
	
	
	
	
	define_repeat! (pub DIGITS_BASE2, "digits-b2", glyphs::DIGIT_BASE2_TOKEN, None, 256);
	define_repeat! (pub DIGITS_BASE8, "digits-b8", glyphs::DIGIT_BASE8_TOKEN, None, 128);
	define_repeat! (pub DIGITS_BASE10, "digits-b10", glyphs::DIGIT_BASE10_TOKEN, None, 64);
	define_repeat! (pub DIGITS_BASE16, "digits-b16", glyphs::DIGIT_BASE16_TOKEN, None, 64);
	define_repeat! (pub DIGITS_BASE32_HEX, "digits-b32hex", glyphs::DIGIT_BASE32_HEX_TOKEN, None, 64);
	define_repeat! (pub DIGITS_BASE32_RFC4648, "digits-b32rfc4648", glyphs::DIGIT_BASE32_RFC4648_TOKEN, None, 64);
	define_repeat! (pub DIGITS_Z85, "digits-Z85", glyphs::DIGIT_Z85_TOKEN, None, 64);
	
	
	
	
	define_sequence! (pub CONSONANT_VOWEL_LOWER_SYLLABLE, "cvl-syllable", [
			glyphs::CONSONANT_LOWER_TOKEN,
			glyphs::VOWEL_LOWER_TOKEN,
			glyphs::CONSONANT_LOWER_TOKEN,
			glyphs::VOWEL_LOWER_TOKEN,
		], Some (Rb::new_static (separators::SPACE_OPTIONAL)));
	
	define_sequence! (pub CONSONANT_VOWEL_UPPER_SYLLABLE, "cvu-syllable", [
			glyphs::CONSONANT_UPPER_TOKEN,
			glyphs::VOWEL_UPPER_TOKEN,
			glyphs::CONSONANT_UPPER_TOKEN,
			glyphs::VOWEL_UPPER_TOKEN,
		], Some (Rb::new_static (separators::SPACE_OPTIONAL)));
	
	define_repeat! (pub CONSONANT_VOWEL_LOWER, "cvl-token", CONSONANT_VOWEL_LOWER_SYLLABLE, Some (Rb::new_static (separators::SPACE_MANDATORY)), 16);
	define_repeat! (pub CONSONANT_VOWEL_UPPER, "cvu-token", CONSONANT_VOWEL_UPPER_SYLLABLE, Some (Rb::new_static (separators::SPACE_MANDATORY)), 16);
	
	
	
	
	pub static ALL : &[&[Rb<TokenPattern>]] = &[
			
			CONSONANT_VOWEL_LOWER_ALL,
			CONSONANT_VOWEL_UPPER_ALL,
			
			DIGITS_BASE2_ALL,
			DIGITS_BASE8_ALL,
			DIGITS_BASE10_ALL,
			DIGITS_BASE16_ALL,
			DIGITS_BASE32_HEX_ALL,
			DIGITS_BASE32_RFC4648_ALL,
			DIGITS_Z85_ALL,
		];
}




pub mod separators {
	
	use super::*;
	
	macro_rules! define_separator {
		( $_visibility : vis $_pattern : ident, $_text : expr ) => {
			::paste::paste! {
				
				static [< _ $_pattern _TEXT >] : &Text = & Text::Static ($_text);
				
				$_visibility static [< $_pattern _MANDATORY >] : &Separator = & Separator::Mandatory (Rb::new_static ( [< _ $_pattern _TEXT >] ));
				$_visibility static [< $_pattern _OPTIONAL >] : &Separator = & Separator::Optional (Rb::new_static ( [< _ $_pattern _TEXT >] ));
			}
		};
	}
	
	
	
	
	define_separator! (pub SPACE, " ");
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

