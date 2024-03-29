

use crate::prelude::*;




pub fn all_token_patterns () -> RbList<(Cow<'static, str>, Rb<TokenPattern>)> {
	
	let mut _collector = Vec::with_capacity (TOKEN_VEC_CAPACITY);
	
	for _patterns in crate::tokens::ALL.iter () {
		for _pattern in _patterns.iter () {
			match _pattern.as_ref () {
				TokenPattern::Tagged (_, _tags) => {
					if let Some (ref _identifier) = _tags.identifier {
						_collector.push ((_identifier.to_string (), _pattern.clone ()));
					}
				}
				_ =>
					panic! (unreachable, 0xcb0098dd),
			}
		}
	}
	
	// FIXME!
	if false {
		_collector.sort_by (|_left, _right| str::cmp (&_left.0, &_right.0));
	}
	
	debug_assert! (_collector.capacity () <= TOKEN_VEC_CAPACITY, "[50582974]  {} <= {}", _collector.capacity (), TOKEN_VEC_CAPACITY);
	
	RbList::from_vec (_collector)
}




pub fn get_token_pattern (_label : &str) -> Option<Rb<TokenPattern>> {
	
	for _patterns in crate::tokens::ALL.iter () {
		for _pattern in _patterns.iter () {
			match _pattern.as_ref () {
				TokenPattern::Tagged (_, _tags) => {
					if let Some (ref _identifier) = _tags.identifier {
						if _identifier.eq (_label) {
							return Some (_pattern.clone ());
						}
					}
					if let Some (ref _aliases) = _tags.aliases {
						for _alias in _aliases.iter () {
							if _alias.eq (_label) {
								return Some (_pattern.clone ());
							}
						}
					}
				}
				_ =>
					panic! (unreachable, 0x978a97f3),
			}
		}
	}
	
	None
}


