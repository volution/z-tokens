

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
		( $_visibility : vis $_pattern : ident, [ $( $_element : expr, )* ] ) => {
			::paste::paste! {
				static [< _ $_pattern __SEQUENCE >] : &[Rb<TokenPattern>] = &[ $(
						Rb::new_static ($_element),
					)* ];
				$_visibility static [< $_pattern >] : &TokenPattern = &TokenPattern::Sequence (RbList::from_static ( [< _ $_pattern __SEQUENCE >] ));
			}
		};
	}
	
	macro_rules! define_repeat {
		( $_visibility : vis $_pattern : ident, $_element : expr, default ) => {
			define_repeat! ($_visibility $_pattern, $_element, [ 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, ]);
		};
		( $_visibility : vis $_pattern : ident, $_element : expr, [ $( $_count : expr, )* ] ) => {
			::paste::paste! {
				$(
					$_visibility static [< $_pattern _ $_count >] : &TokenPattern = & TokenPattern::Repeat (Rb::new_static ($_element), $_count);
				)*
			}
		};
	}
	
	pub static EMPTY : &TokenPattern = & TokenPattern::Empty;
	
	define_sequence! (pub CONSONANT_VOWEL_LOWER_SYLLABLE, [
			glyphs::CONSONANT_LOWER_TOKEN,
			glyphs::VOWEL_LOWER_TOKEN,
		]);
	define_sequence! (pub CONSONANT_VOWEL_UPPER_SYLLABLE, [
			glyphs::CONSONANT_UPPER_TOKEN,
			glyphs::VOWEL_UPPER_TOKEN,
		]);
	
	define_repeat! (pub CONSONANT_VOWEL_LOWER, CONSONANT_VOWEL_LOWER_SYLLABLE, default);
	define_repeat! (pub CONSONANT_VOWEL_UPPER, CONSONANT_VOWEL_UPPER_SYLLABLE, default);
	
	define_repeat! (pub DIGITS, glyphs::DIGIT_TOKEN, default);
}

