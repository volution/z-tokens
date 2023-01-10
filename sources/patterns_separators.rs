

use crate::prelude::*;


use crate::patterns_macros as macros;


include! ("./patterns_separators_macros.in");








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


