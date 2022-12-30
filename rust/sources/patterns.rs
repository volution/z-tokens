

use crate::prelude::*;




pub mod glyphs {
	
	use super::*;
	
	macro_rules! define_set {
		( $_visibility : vis $_pattern : ident, [ $( $_char : expr, )* ] ) => {
			::paste::paste! {
				$(
					static [< _ $_pattern __ $_char __TEXT >] : &Text = & Text::Char ($_char);
					static [< _ $_pattern __ $_char __GLYPH >] : &Glyph = & Glyph::Text (Rb::new_static ( [< _ $_pattern __ $_char __TEXT >] ));
				)*
				
				#[ doc = concat! ( "Glyph character set for `", $( $_char ),*, "`." ) ]
				$_visibility static [< $_pattern _SET >] : &[Rb<Glyph>] = &[ $(
						Rb::new_static ( [< _ $_pattern __ $_char __GLYPH >] ),
					)* ];
				
				$_visibility static [< $_pattern _GLYPH >] : &GlyphPattern = & GlyphPattern::Set (RbList::from_static ( [< $_pattern _SET >] ));
				
				$_visibility static [< $_pattern _ATOM >] : &AtomPattern = & AtomPattern::Glyph (Rb::new_static ( [< $_pattern _GLYPH >] ));
				
				$_visibility static [< $_pattern _TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_pattern _ATOM >] ));
			}
		};
	}
	
	define_set! (pub DIGIT, [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ]);
	
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
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, $_separator : expr, default ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, $_separator, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, ]);
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
	
	define_repeat! (pub CONSONANT_VOWEL_LOWER, "cvl-token", CONSONANT_VOWEL_LOWER_SYLLABLE, Some (Rb::new_static (separators::SPACE_MANDATORY)), default);
	define_repeat! (pub CONSONANT_VOWEL_UPPER, "cvu-token", CONSONANT_VOWEL_UPPER_SYLLABLE, Some (Rb::new_static (separators::SPACE_MANDATORY)), default);
	
	define_repeat! (pub DIGITS, "digits", glyphs::DIGIT_TOKEN, None, default);
	
	
	
	
	pub static ALL : &[&[Rb<TokenPattern>]] = &[
			CONSONANT_VOWEL_LOWER_ALL,
			CONSONANT_VOWEL_UPPER_ALL,
			DIGITS_ALL,
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

