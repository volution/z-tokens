

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
				
				#[ doc = ::std::concat! ( "Glyph character set for `", $( $_char ),*, "`." ) ]
				$_visibility static [< $_pattern _ SET >] : &[Rb<Glyph>] = &[ $(
						Rb::new_static ( [< _ $_pattern __ $_char __GLYPH >] ),
					)* ];
				
				$_visibility static [< $_pattern _ GLYPH >] : &GlyphPattern = & GlyphPattern::Set (RbList::from_static ( [< $_pattern _ SET >] ));
				
				$_visibility static [< $_pattern _ ATOM >] : &AtomPattern = & AtomPattern::Glyph (Rb::new_static ( [< $_pattern _ GLYPH >] ));
				
				$_visibility static [< $_pattern _ TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_pattern _ ATOM >] ));
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
		( $_visibility : vis $_pattern : ident, $_identifier : expr, [ $( $_element : expr, )* ] ) => {
			::paste::paste! {
				static [< _ $_pattern __SEQUENCE >] : &[Rb<TokenPattern>] = &[ $(
						Rb::new_static ($_element),
					)* ];
				static [< _ $_pattern __NO_NAME >] : &TokenPattern = & TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern __SEQUENCE >] ));
				$_visibility static [< $_pattern >] : &TokenPattern = & TokenPattern::Named ($_identifier, Rb::new_static ( [< _ $_pattern __NO_NAME >] ));
			}
		};
	}
	
	macro_rules! define_repeat {
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, default ) => {
			define_repeat! ($_visibility $_pattern, $_identifier, $_element, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, ]);
		};
		( $_visibility : vis $_pattern : ident, $_identifier : expr, $_element : expr, [ $( $_count : expr, )* ] ) => {
			::paste::paste! {
				$(
					static [< _ $_pattern _ $_count __NO_NAME >] : &TokenPattern = & TokenPattern::Repeat (Rb::new_static ($_element), $_count);
					$_visibility static [< $_pattern _ $_count >] : &TokenPattern = & TokenPattern::Named (::std::concat! ($_identifier, "-", $_count), Rb::new_static ( [< _ $_pattern _ $_count __NO_NAME >] ));
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
		]);
	define_sequence! (pub CONSONANT_VOWEL_UPPER_SYLLABLE, "cvu-syllable", [
			glyphs::CONSONANT_UPPER_TOKEN,
			glyphs::VOWEL_UPPER_TOKEN,
		]);
	
	define_repeat! (pub CONSONANT_VOWEL_LOWER, "cvl-token", CONSONANT_VOWEL_LOWER_SYLLABLE, default);
	define_repeat! (pub CONSONANT_VOWEL_UPPER, "cvu-token", CONSONANT_VOWEL_UPPER_SYLLABLE, default);
	
	define_repeat! (pub DIGITS, "digits", glyphs::DIGIT_TOKEN, default);
	
	
	
	
	pub static ALL : &[&[Rb<TokenPattern>]] = &[
			CONSONANT_VOWEL_LOWER_ALL,
			CONSONANT_VOWEL_UPPER_ALL,
			DIGITS_ALL,
		];
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

