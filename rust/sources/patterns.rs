

use crate::prelude::*;




pub mod glyphs {
	
	use super::*;
	
	macro_rules! define_set {
		( $_pattern : ident, [ $( $_char : expr, )* ] ) => {
			::paste::paste! {
				$(
					static [< _ $_pattern __ $_char __TEXT >] : &Text = & Text::Char ($_char);
					static [< _ $_pattern __ $_char __GLYPH >] : &Glyph = & Glyph::Text (Rb::new_static ( [< _ $_pattern __ $_char __TEXT >] ));
				)*
				
				pub static [< $_pattern _ SET >] : &[Rb<Glyph>] = &[ $(
						Rb::new_static ( [< _ $_pattern __ $_char __GLYPH >] ),
					)* ];
				
				pub static [< $_pattern _ GLYPH >] : &GlyphPattern = & GlyphPattern::Set (RbList::from_static ( [< $_pattern _ SET >] ));
				
				pub static [< $_pattern _ ATOM >] : &AtomPattern = & AtomPattern::Glyph (Rb::new_static ( [< $_pattern _ GLYPH >] ));
				pub static [< $_pattern _ TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_pattern _ ATOM >] ));
			}
		}
	}
	
	define_set! (DIGIT, [ '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ]);
	
	define_set! (VOWEL_LOWER, [ 'a', 'e', 'i', 'o', 'u', ]);
	
	define_set! (CONSONANT_LOWER, [ 'b', 'c', 'd', 'f', 'g', 'h', 'j', 'k', 'l', 'm', 'n', 'p', 'q', 'r', 's', 't', 'v', 'w', 'x', 'y', 'z', ]);
}




pub mod atoms {
	
	use super::*;
}




pub mod tokens {
	
	use super::*;
	
	pub static EMPTY : &TokenPattern = & TokenPattern::Empty;
	
	static CONSONANT_VOWEL_LOWER_SYLLABLE_SEQUENCE : &[Rb<TokenPattern>] = &[
			Rb::new_static (glyphs::CONSONANT_LOWER_TOKEN),
			Rb::new_static (glyphs::VOWEL_LOWER_TOKEN),
		];
	pub static CONSONANT_VOWEL_LOWER_SYLLABLE : &TokenPattern = & TokenPattern::Sequence (RbList::from_static (CONSONANT_VOWEL_LOWER_SYLLABLE_SEQUENCE));
	pub static CONSONANT_VOWEL_LOWER_2 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (CONSONANT_VOWEL_LOWER_SYLLABLE), 2);
	pub static CONSONANT_VOWEL_LOWER_4 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (CONSONANT_VOWEL_LOWER_SYLLABLE), 4);
	pub static CONSONANT_VOWEL_LOWER_8 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (CONSONANT_VOWEL_LOWER_SYLLABLE), 8);
	
	pub static DIGITS_2 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 2);
	pub static DIGITS_4 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 4);
	pub static DIGITS_6 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 6);
	pub static DIGITS_8 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 8);
	pub static DIGITS_10 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 10);
	pub static DIGITS_12 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 12);
	pub static DIGITS_14 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 14);
	pub static DIGITS_16 : &TokenPattern = & TokenPattern::Repeat (Rb::new_static (glyphs::DIGIT_TOKEN), 16);
}

