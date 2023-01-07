

use crate::prelude::*;




pub mod tokens {
	pub use crate::patterns_tokens::*;
}

pub mod glyphs {
	pub use crate::patterns_glyphs::*;
}

pub mod separators {
	pub use crate::patterns_separators::*;
}




pub fn all_token_patterns () -> RbList<(String, Rb<TokenPattern>)> {
	
	let mut _collector = Vec::with_capacity (1024);
	
	for _patterns in tokens::ALL.iter () {
		for _pattern in _patterns.iter () {
			match _pattern.as_ref () {
				TokenPattern::Named (_identifier, _aliases, _) =>
					_collector.push ((String::from (*_identifier), _pattern.clone ())),
				_ =>
					panic! (0xcb0098dd),
			}
		}
	}
	
	RbList::from_vec (_collector)
}




pub fn get_token_pattern (_identifier : &str) -> Option<Rb<TokenPattern>> {
	
	for _patterns in tokens::ALL.iter () {
		for _pattern in _patterns.iter () {
			match _pattern.as_ref () {
				TokenPattern::Named (_identifier_0, _aliases_0, _) => {
					if *_identifier_0 == _identifier {
						return Some (_pattern.clone ());
					}
					for _alias_0 in *_aliases_0 {
						if *_alias_0 == _identifier {
							return Some (_pattern.clone ());
						}
					}
				}
				_ =>
					(),
			}
		}
	}
	
	None
}


