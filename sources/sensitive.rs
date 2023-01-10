

use crate::prelude::*;


include! ("./sensitive_macros.in");




pub trait Sensitive {
	fn erase (&mut self);
}








impl <Value : Sized + Sensitive + 'static> Sensitive for Rb<Value> {
	fn erase (&mut self) -> () {
		self.0.erase ();
	}
}

impl <Value : Sized + Sensitive + 'static> Sensitive for RbList<Value> {
	fn erase (&mut self) -> () {
		self.0.erase ();
	}
}




impl <Value : Sized + Sensitive + 'static> Sensitive for RbRef<Value> {
	
	fn erase (&mut self) -> () {
		match self {
			Self::Static (ref mut _ref) =>
				<&'static Value as Sensitive>::erase (_ref),
			Self::Rc (_rc) =>
				_rc.erase (),
		}
	}
}


impl <Value : Sized + Sensitive + 'static> Sensitive for RbListRef<Value> {
	
	fn erase (&mut self) -> () {
		match self {
			Self::Static (ref mut _ref) =>
				<&'static [Rb<Value>] as Sensitive>::erase (_ref),
			Self::Rc (_rc) =>
				_rc.erase (),
		}
	}
}








impl_sensitive_drop! (<{Value}> Rb <Value> where <{ Value : Sized + Sensitive + 'static }>);
impl_sensitive_drop! (<{Value}> RbRef<Value> where <{ Value : Sized + Sensitive + 'static }>);

impl_sensitive_drop! (<{Value}> RbList<Value> where <{ Value : Sized + Sensitive + 'static }>);
impl_sensitive_drop! (<{Value}> RbListRef<Value> where <{ Value : Sized + Sensitive + 'static }>);








// NOTE:  Has only public information.
impl_sensitive_nop! (TokenPattern);
impl_sensitive_nop! (TokenPatternTags);
impl_sensitive_nop! (SeparatorPattern);
impl_sensitive_nop! (AtomPattern);
impl_sensitive_nop! (GlyphPattern);








impl_sensitive! (Token => |self| {
	self.atoms.erase ();
});

impl_sensitive! (Atom => |self| {
	match self {
		Atom::Separator (_separator) =>
			_separator.erase (),
		Atom::Constant (_constant) =>
			_constant.erase (),
		Atom::Glyph (_glyph) =>
			_glyph.erase (),
	}
});

impl_sensitive! (Glyph => |self| {
	match self {
		Glyph::Text (_text) =>
			_text.erase (),
		Glyph::Integer (_integer, _) =>
			_integer.erase (),
		Glyph::Bytes (_bytes, _) =>
			_bytes.erase (),
		Glyph::Timestamp (_timestamp, _) =>
			_timestamp.erase (),
	}
});

impl_sensitive! (Separator => |self| {
	match self {
		Separator::Mandatory (_text) =>
			_text.erase (),
		Separator::Optional (_text) =>
			_text.erase (),
	}
});


impl_sensitive! (Text => |self| {
	match self {
		Text::Char (_char) =>
			_char.erase (),
		Text::Str (_str) =>
			_str.erase (),
		Text::String (_string) =>
			_string.erase (),
	}
});

impl_sensitive! (Bytes => |self| {
	match self {
		Bytes::Static (_slice) =>
			_slice.erase (),
		Bytes::Boxed (_box) =>
			_box.as_mut () .erase (),
	}
});




impl_sensitive! ( <{ Value }> Arc<Value> where <{ Value : Sensitive }> => |self| {
	if let Some (_value) = Arc::get_mut (self) {
		_value.erase ()
	}
});

impl_sensitive! ( <{ Value }> Arc<[Value]> where <{ Value : Sensitive }> => |self| {
	if let Some (_values) = Arc::get_mut (self) {
		for _value in _values {
			_value.erase ()
		}
	}
});

impl_sensitive! ( <{ A, B }> (A, B) where <{ A : Sensitive, B : Sensitive }> => |self| {
	self.0.erase ();
	self.1.erase ();
});

impl_sensitive! ( <{'a}> &'a str => |self| {
	*self = "";
});

impl_sensitive! ( <{'a, Value}> &'a Value => |self| {
	*self = unsafe { mem::transmute ( [0xffu8; mem::size_of::<&'static u8> ()] ) };
});

impl_sensitive! ( <{'a, Value}> &'a [Value] => |self| {
	*self = unsafe { mem::transmute ( [0xffu8; mem::size_of::<&'static [u8]> ()] ) };
});

impl_sensitive! ( <{'a}> Cow<'a, str> => |self| {
	match self {
		Cow::Borrowed (_str) =>
			_str.erase (),
		Cow::Owned (_string) =>
			_string.erase (),
	}
});








impl_sensitive_zeroize! (u128);
impl_sensitive_zeroize! (char);
impl_sensitive_zeroize! (String);
impl_sensitive_zeroize! ([u8]);



