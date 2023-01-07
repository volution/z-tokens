

use crate::prelude::*;


use crate::patterns_macros as macros;




macro_rules! define_separator {
	
	( $_visibility : vis $_pattern : ident, separator, $_variant : ident, $_text : expr ) => {
		::paste::paste! {
			
			static [< _ $_pattern _TEXT >] : &Text = & Text::$_variant ($_text);
			
			$_visibility static [< $_pattern _MANDATORY_SEPARATOR >] : &Separator = & Separator::Mandatory (Rb::new_static ( [< _ $_pattern _TEXT >] ));
			$_visibility static [< $_pattern _OPTIONAL_SEPARATOR >] : &Separator = & Separator::Optional (Rb::new_static ( [< _ $_pattern _TEXT >] ));
		}
	};
	
	( $_visibility : vis $_pattern : ident, atom ) => {
		::paste::paste! {
			
			$_visibility static [< $_pattern _MANDATORY_ATOM >] : &AtomPattern = & AtomPattern::Separator (Rb::new_static ( [< $_pattern _MANDATORY_SEPARATOR >] ));
			$_visibility static [< $_pattern _OPTIONAL_ATOM >] : &AtomPattern = & AtomPattern::Separator (Rb::new_static ( [< $_pattern _OPTIONAL_SEPARATOR >] ));
			
			$_visibility static [< $_pattern _MANDATORY_TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_pattern _MANDATORY_ATOM >] ));
			$_visibility static [< $_pattern _OPTIONAL_TOKEN >] : &TokenPattern = & TokenPattern::Atom (Rb::new_static ( [< $_pattern _OPTIONAL_ATOM >] ));
		}
	};
	
	( $_visibility : vis $_pattern : ident, infix, ( $_length : tt : $_each : tt ) ) => {
		macros::__count_call_with! ( [ $_length : $_each ] => define_separator! ($_visibility $_pattern, infix, ));
	};
	
	( $_visibility : vis $_pattern : ident, infix, [ $( $_infix_each : literal, )* ] ) => {
		::paste::paste! {
			
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




define_separator! (pub SPACE, separator, Char, ' ');
define_separator! (pub DOT, separator, Char, '.');
define_separator! (pub HYPHEN, separator, Char, '-');
define_separator! (pub COLON, separator, Char, ':');

define_separator! (pub SPACE_HYPHEN_SPACE, separator, Str, " - ");


define_separator! (pub SPACE, atom);
define_separator! (pub DOT, atom);
define_separator! (pub HYPHEN, atom);
define_separator! (pub COLON, atom);

define_separator! (pub SPACE_HYPHEN_SPACE, atom);


define_separator! (pub SPACE, infix, ( 16 : 1 ));
define_separator! (pub DOT, infix, ( 16 : 1 ));
define_separator! (pub HYPHEN, infix, ( 16 : 1 ));
define_separator! (pub COLON, infix, ( 16 : 1 ));

define_separator! (pub SPACE_HYPHEN_SPACE, infix, ( 16 : 1 ));


